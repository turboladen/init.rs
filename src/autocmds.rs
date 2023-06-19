use overkill_nvim::api::autocmd;

pub(super) fn init() {
    autocmd::augroup("vimrc").unwrap();
    {
        autocmd::force_autocmd("").unwrap();

        // Delete the buffer once I'm done with it.
        autocmd::autocmd("BufReadPost fugitive://* set bufhidden=delete").unwrap();
        autocmd::autocmd(r#"TextYankPost * silent! lua vim.highlight.on_yank {higroup="IncSearch", timeout=150}"#).unwrap();
    }
    autocmd::augroup("end").unwrap();

    // autocmd::augroup("FTCheck").unwrap();
    // {
    //     autocmd::force_autocmd("").unwrap();
    //     autocmd::autocmd("BufNewFile,BufRead *.jbuilder set ft=ruby").unwrap();
    //     autocmd::autocmd("BufNewFile,BufRead *.rs.hbs setlocal ft=rust.handlebars").unwrap();
    // }
    // autocmd::augroup("end").unwrap();
}
