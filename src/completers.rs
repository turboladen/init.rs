use nvim_oxi::{self, api::Buffer};
use nvim_sous_chef::complete_fn::{
    completion::matches::{List, Matches},
    CompleteFn,
};
use regex::bytes::Regex;

/// Struct for implementing `CompleteFn` on.
///
pub(crate) struct RegexBufferCompleter;

impl CompleteFn for RegexBufferCompleter {
    fn make_matches(base: nvim_oxi::String) -> nvim_oxi::Result<Matches> {
        let mut matches = List::default();

        let Ok(base_str) = std::str::from_utf8(base.as_bytes()) else {
            log::error!("Unable to read base as UTF-8 bytes: {base}");
            return Ok(Matches::List(matches));
        };
        log::debug!("Completing using base_str: {base_str}");

        let file_bytes = {
            let mut file_bytes: Vec<u8> = vec![];
            let current_buffer = Buffer::current();

            for line in current_buffer.get_lines(..=current_buffer.line_count()?, false)? {
                file_bytes.extend_from_slice(line.as_bytes());
                file_bytes.extend_from_slice(b"\n");
            }

            file_bytes
        };
        log::trace!("Got file bytes");

        {
            let re = Regex::new(&format!("\\b({base_str}\\w*)")).unwrap();

            for (_, [word_bytes]) in re.captures_iter(&file_bytes).map(|c| c.extract()) {
                let word = std::str::from_utf8(word_bytes).unwrap();
                log::debug!("Found match: {word}");
                matches.push(word);
            }
        }

        matches.sort_words();

        Ok(Matches::List(matches))
    }
}
