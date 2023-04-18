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

    autocmd::augroup("FTOptions").unwrap();
    {
        autocmd::force_autocmd("").unwrap();
        autocmd::autocmd("FileType dirvish setlocal nospell").unwrap();
        autocmd::autocmd(r#"FileType haproxy setlocal commentstring=#\ %s"#).unwrap();
        autocmd::autocmd("FileType help nnoremap q :q").unwrap();
        autocmd::autocmd("FileType html setlocal softtabstop=4").unwrap();
        autocmd::autocmd("FileType qf setlocal nospell").unwrap();

        autocmd::autocmd("FileType vim setlocal ts=2 sts=2 sw=2 expandtab").unwrap();
        autocmd::autocmd(
            "FileType yaml setlocal ts=2 sts=2 sw=2 expandtab cursorcolumn indentkeys-=<:>",
        )
        .unwrap();
        autocmd::autocmd("FileType zsh setlocal ts=2 sts=2 sw=2 expandtab").unwrap();
    }
    autocmd::augroup("end").unwrap();
}
