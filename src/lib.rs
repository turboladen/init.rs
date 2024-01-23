pub(crate) mod completers;

use nvim_oxi::{Dictionary, Function, Object};
use nvim_sous_chef_complete_fn::CompleteFn;
use nvim_sous_chef_logger::LogLevel;

#[nvim_oxi::module]
fn init_rs() -> nvim_oxi::Result<Dictionary> {
    let regex_buffer_completer = Function::from_fn(completers::RegexBufferCompleter::complete_fn);
    let enable_logging = Function::from_fn(|(maybe_max_level,)| {
        enable_logging(maybe_max_level);
        Ok::<(), nvim_oxi::Error>(())
    });
    let disable_logging = Function::from_fn(|_: ()| {
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
    ]))
}

pub fn enable_logging(max_level: Option<LogLevel>) {
    nvim_oxi::print!("[sous_chef.init_rs] Calling enable()");
    nvim_sous_chef_logger::enable(max_level);
}

pub fn disable_logging() {
    nvim_sous_chef_logger::disable();
}
