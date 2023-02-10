mod autocmds;
mod mappings;
mod options;

use nvim_oxi::{api as oxi, Dictionary};

#[nvim_oxi::module]
fn init_rs() -> Result<Dictionary, oxi::Error> {
    options::init();
    mappings::init();
    autocmds::init();

    Ok(Dictionary::new())
}
