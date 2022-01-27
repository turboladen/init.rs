mod autocmds;
mod mappings;
mod options;

#[no_mangle]
pub extern "C" fn init() {
    options::init();
    mappings::init();
    autocmds::init();
}
