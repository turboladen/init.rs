use std::path::Path;

use grep::{
    matcher::Matcher as _,
    regex::RegexMatcher,
    searcher::{sinks::UTF8, Searcher},
};
use ignore::WalkBuilder;
use nvim_oxi::{Array, Dictionary, Function, Object};

pub(crate) fn ripgrep() -> Dictionary {
    let rg = Function::from_fn(|(matcher,)| rg(matcher));

    Dictionary::from_iter([("rg", Object::from(rg))])
}

fn rg(requested_regex_str: Option<nvim_oxi::String>) -> nvim_oxi::Result<Array> {
    let pattern = make_match_str(requested_regex_str)?;
    let matcher = RegexMatcher::new(&pattern.to_string_lossy()).map_err(swap_error)?;

    let cwd = std::env::current_dir().map_err(swap_error)?;

    let search_results = search(&pattern, &cwd, &matcher)?;
    let array: Array = search_results
        .into_iter()
        .enumerate()
        .map(|(i, item)| {
            nvim_oxi::print!("Item num {i}");
            Object::from(item)
        })
        .collect();

    Ok(array)
}

fn swap_error<E>(error: E) -> nvim_oxi::Error
where
    E: std::error::Error,
{
    nvim_oxi::Error::Api(nvim_oxi::api::Error::Other(error.to_string()))
}

fn make_match_str(matcher: Option<nvim_oxi::String>) -> nvim_oxi::Result<nvim_oxi::String> {
    match matcher {
        Some(match_string) if !match_string.is_empty() => Ok(match_string),
        _ => nvim_oxi::api::call_function("expand", ("<cword>",)).map_err(swap_error),
    }
}

fn search(
    pattern: &nvim_oxi::String,
    cwd: &Path,
    regex_matcher: &RegexMatcher,
) -> nvim_oxi::Result<Vec<Match>> {
    let walker = WalkBuilder::new(cwd).build();

    let mut matches: Vec<Match> = vec![];

    for entry in walker {
        let entry = entry.map_err(swap_error)?;

        let sink = UTF8(|line_number, line| {
            if let Ok(Some(mymatch)) = regex_matcher.find(line.as_bytes()) {
                nvim_oxi::print!("Found match: {:#?}", &mymatch);

                matches.push(Match {
                    file_name: nvim_oxi::String::from_bytes(
                        entry.path().to_string_lossy().as_bytes(),
                    ),
                    line_number: line_number.try_into().unwrap_or_default(),
                    column_start: mymatch.start().try_into().unwrap_or_default(),
                    column_end: mymatch.end().try_into().unwrap_or_default(),
                    text_line: nvim_oxi::String::from_bytes(line.trim().as_bytes()),
                    pattern: pattern.clone(),
                });
            }

            Ok(true)
        });

        Searcher::new()
            .search_path(regex_matcher, entry.path(), sink)
            .ok();
    }

    Ok(matches)
}

struct Match {
    file_name: nvim_oxi::String,
    line_number: i64,
    column_start: i64,
    column_end: i64,
    text_line: nvim_oxi::String,
    pattern: nvim_oxi::String,
}

impl From<Match> for Object {
    fn from(value: Match) -> Self {
        Dictionary::from(value).into()
    }
}

impl From<Match> for Dictionary {
    fn from(value: Match) -> Self {
        Self::from_iter([
            ("file_name", Object::from(value.file_name)),
            ("line_number", Object::from(value.line_number)),
            ("column_start", Object::from(value.column_start)),
            ("column_end", Object::from(value.column_end)),
            ("text_line", Object::from(value.text_line)),
            ("pattern", Object::from(value.pattern)),
        ])
    }
}
