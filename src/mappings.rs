use overkill_nvim::mapping::{MapMode, Mapper};

pub(super) fn init() {
    let mut n = Mapper::new(MapMode::Normal);
    let v = Mapper::new(MapMode::Visual);

    // Silent group
    n.group(|mapper| {
        let mapper = mapper.silent();

        // Save some ring-finger key strokes
        mapper.noremap("<C-l>", "<C-w>l");
        mapper.noremap("<C-h>", "<C-w>h");
        mapper.noremap("<C-j>", "<C-w>j");
        mapper.noremap("<C-k>", "<C-w>k");

        // Split a line and remove whitespace from old line.
        // https://www.reddit.com/r/vim/comments/3g8y3r/finally_hacked_together_a_quick_split_line/ctw4b0i
        mapper.noremap("S", r#"i<CR><ESC>^mwgk:silent! s/\v +$/<CR>:noh<CR>"#);
    });

    //-------------------------------------------------------------------------
    // eDIT MY vIMRC FILE
    //-------------------------------------------------------------------------
    n.noremap("<leader>ev", "<cmd>vsplit $MYVIMRC<CR>");
    n.noremap(
        "<leader>ep",
        "<cmd>vsplit ~/.config/nvim/lua/plugins.lua<CR>",
    );
    n.noremap(
        "<leader>el",
        "<cmd>vsplit ~/.config/nvim/lua/turboladen/lsp.lua<CR>",
    );

    // Reload all the things
    n.noremap("<leader>v", "<cmd>source $MYVIMRC<CR>");

    n.map("Q", "<NOP>");

    // After block yank and paste, move cursor to the end of operated text
    // Also, don't copy over-pasted text in visual mode
    v.noremap("y", "y`]");
    v.noremap("p", r#""_dP"#);
    n.noremap("p", "p`]");
}
