//! # init.rs
//!
//! My plugins.
//!
#![deny(unused_extern_crates)]
#![warn(
    box_pointers,
    clippy::all,
    clippy::nursery,
    clippy::pedantic,
    future_incompatible,
    missing_copy_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_qualifications
)]
#![allow(clippy::redundant_pub_crate)]

pub(crate) mod completers;
pub(crate) mod ripgrep;

use nvim_oxi::{Dictionary, Function, Object};
use nvim_sous_chef::{ins_completion::complete_fn::CompleteFn, logger::LogLevel};

#[allow(clippy::unnecessary_wraps)]
#[nvim_oxi::module]
fn init_rs() -> nvim_oxi::Result<Dictionary> {
    let regex_buffer_completer = Function::from_fn(completers::RegexBufferCompleter::complete_fn);

    let enable_logging = Function::from_fn(|(maybe_max_level,)| {
        enable_logging(maybe_max_level);
        Ok::<(), nvim_oxi::Error>(())
    });

    let disable_logging = Function::from_fn(|()| {
        disable_logging();
        Ok::<(), nvim_oxi::Error>(())
    });

    Ok(Dictionary::from_iter([
        (
            "regex_buffer_completer",
            Object::from(regex_buffer_completer),
        ),
        ("enable_logging", Object::from(enable_logging)),
        ("disable_logging", Object::from(disable_logging)),
        ("rg", crate::ripgrep::lua_rg()),
    ]))
}

/// Turns on logging.
///
pub fn enable_logging(max_level: Option<LogLevel>) {
    nvim_oxi::print!("[sous_chef.init_rs] Calling enable()");
    nvim_sous_chef::logger::enable(max_level);
}

/// Turns off logging.
///
pub fn disable_logging() {
    nvim_sous_chef::logger::disable();
}
