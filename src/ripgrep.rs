// let g:rg_format = "%f:%l:%c:%m"
// :h grepformat
// :h errorformat
// %f = file name
// %l = line number
// %c = column number
// %m = message

use std::path::{Path, PathBuf};

use grep::{
    matcher::Matcher as _,
    regex::RegexMatcher,
    searcher::{sinks::UTF8, Searcher},
};
use ignore::WalkBuilder;
use nvim_oxi::{Function, Object};

pub(crate) fn lua_rg() -> Object {
    let f = Function::from_fn(|(matcher,)| rg(matcher));

    Object::from(f)
}

fn rg(requested_matcher: nvim_oxi::String) -> nvim_oxi::Result<()> {
    let matcher = make_matcher(requested_matcher)?;
    let cwd = std::env::current_dir().map_err(swap_error)?;
    let search_results = search(&cwd, &matcher)?;

    nvim_oxi::print!(
        "Search results ({}): {:#?}",
        search_results.len(),
        &search_results
    );

    Ok(())
}

fn swap_error<E>(error: E) -> nvim_oxi::Error
where
    E: std::error::Error,
{
    nvim_oxi::Error::Api(nvim_oxi::api::Error::Other(error.to_string()))
}

fn make_matcher(matcher: nvim_oxi::String) -> nvim_oxi::Result<RegexMatcher> {
    let term = if matcher.is_empty() {
        nvim_oxi::api::call_function("expand", ("<cword>",)).map_err(swap_error)?
    } else {
        matcher
    };

    RegexMatcher::new(&term.to_string_lossy())
        .map_err(|e| nvim_oxi::Error::Api(nvim_oxi::api::Error::Other(e.to_string())))
}

fn search(cwd: &Path, regex_matcher: &RegexMatcher) -> nvim_oxi::Result<Vec<Match>> {
    let walker = WalkBuilder::new(cwd).build();

    let mut matches: Vec<Match> = vec![];

    for entry in walker {
        let entry = entry.map_err(swap_error)?;

        let sink = UTF8(|line_number, line| {
            if let Ok(Some(mymatch)) = regex_matcher.find(line.as_bytes()) {
                nvim_oxi::print!("Found match: {:#?}", &mymatch);

                matches.push(Match {
                    file_path: entry.clone().into_path(),
                    line_number,
                    column_number: mymatch.start(),
                    matching_text: line[mymatch].to_string(),
                });
            }

            Ok(true)
        });

        if let Err(e) = Searcher::new().search_path(regex_matcher, entry.path(), sink) {
            // nvim_oxi::print!("Error searching file: {e}");
        }
    }

    Ok(matches)
}

#[derive(Debug)]
struct Match {
    file_path: PathBuf,
    line_number: u64,
    column_number: usize,
    matching_text: String,
}
