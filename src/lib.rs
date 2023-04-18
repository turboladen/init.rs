// mod autocmds;
mod mappings;
mod options;

#[cfg(feature = "oxi")]
#[nvim_oxi::module]
fn init_rs() -> Result<Dictionary, oxi::Error> {
    use nvim_oxi::{api as oxi, Dictionary};
    options::init();
    mappings::init();
    // autocmds::init();
    //

    Ok(Dictionary::new())
    // Ok(Dictionary::from_iter([
    //     (
    //         "load_everforest_hard_dark",
    //         Function::from_fn(everforest::load_hard_dark),
    //     ),
    //     (
    //         "load_everforest_hard_light",
    //         Function::from_fn(everforest::load_hard_light),
    //     ),
    //     (
    //         "load_everforest_medium_dark",
    //         Function::from_fn(everforest::load_medium_dark),
    //     ),
    //     (
    //         "load_everforest_medium_light",
    //         Function::from_fn(everforest::load_medium_light),
    //     ),
    //     (
    //         "load_everforest_soft_dark",
    //         Function::from_fn(everforest::load_soft_dark),
    //     ),
    //     (
    //         "load_everforest_soft_light",
    //         Function::from_fn(everforest::load_soft_light),
    //     ),
    // ]))
}

#[no_mangle]
pub extern "C" fn init_options() {
    options::init();
}

#[no_mangle]
pub extern "C" fn init_mappings() {
    mappings::init();
}

// #[no_mangle]
// pub extern "C" fn init_autocmds() {
//     autocmds::init();
// }
