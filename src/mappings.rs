use std::fmt::Display;

use overkill_nvim::mapping::{MapMode, Mapper};

pub(super) fn init() {
    let n = Mapper::new(MapMode::Normal);
    let v = Mapper::new(MapMode::Visual);

    general_vim_mappings(n, v);
    plugin_mappings(n, v);
}

fn general_vim_mappings(mut n: Mapper, v: Mapper) {
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
    n.noremap("<leader>ev", &cmd("vsplit $MYVIMRC"));
    n.noremap("<leader>ep", &cmd("vsplit ~/.config/nvim/lua/plugins.lua"));
    n.noremap(
        "<leader>el",
        &cmd("vsplit ~/.config/nvim/lua/turboladen/lsp.lua"),
    );

    // Reload all the things
    n.noremap("<leader>v", &cmd("source $MYVIMRC"));

    n.map("Q", "<NOP>");

    // After block yank and paste, move cursor to the end of operated text
    // Also, don't copy over-pasted text in visual mode
    v.noremap("y", "y`]");
    v.noremap("p", r#""_dP"#);
    n.noremap("p", "p`]");
}

fn plugin_mappings(mut n: Mapper, mut v: Mapper) {
    // Normal, silent
    n.group(|n| {
        let n = n.silent();

        //-----------------------
        // lazygit
        //-----------------------
        n.noremap("<leader>gg", &cmd("LazyGit<CR>"));

        //-----------------------
        // nvim-dap
        //-----------------------
        n.noremap(
            "<leader>di",
            &cmd("lua require('dap.ui.variables').hover()"),
        );
        n.noremap(
            "<leader>d?",
            &cmd("lua require('dap.ui.variables').scopes()"),
        );

        //-----------------------
        // nvim-hlslens
        //-----------------------
        n.noremap(
            "n",
            &cmd("execute('normal! ' . v:count1 . 'n')<CR><cmd>lua require('hlslens').start()"),
        );
        n.noremap(
            "N",
            &cmd("execute('normal! ' . v:count1 . 'N')<CR><cmd>lua require('hlslens').start()"),
        );

        //-----------------------
        // telescope.nvim
        //-----------------------
        n.noremap("<leader><space>", &telescope_builtin("find_files"));
        n.noremap("<leader><CR>", &telescope_builtin("buffers"));
        n.noremap("<leader>/", &telescope_builtin("live_grep"));
        n.noremap("<leader>fk", &telescope_builtin("grep_string"));
        n.noremap("<leader>fm", &telescope_builtin("marks"));
        n.noremap("<leader>fo", &telescope_builtin("oldfiles"));
        n.noremap(
            "<leader>ft",
            &cmd("lua require('telescope.builtin').grep_string({ search = 'TODO' })"),
        );

        //-----------------------
        // trouble.nvim
        //-----------------------
        n.noremap("<leader>xx", &cmd("Trouble"));
        n.noremap("<leader>xw", &trouble_toggle("workspace_diagnostics"));
        n.noremap("<leader>xd", &trouble_toggle("document_diagnostics"));
        n.noremap("<leader>xR", &trouble_toggle("lsp_references"));
        n.noremap("<leader>xD", &trouble_toggle("lsp_definitions"));
        n.noremap("<leader>xT", &trouble_toggle("lsp_type_definitions"));
        n.noremap("<leader>xr", &cmd("TroubleRefresh"));
        n.noremap("<leader>xl", &trouble_toggle("loclist"));
        n.noremap("<leader>xq", &trouble_toggle("quickfix"));

        // jump to the next item, skipping the groups
        n.noremap(
            "<leader>]",
            &cmd("lua require('trouble').next({skip_groups = true, jump = true})"),
        );
        // jump to the previous item, skipping the groups
        n.noremap(
            "<leader>[",
            &cmd("lua require('trouble').previous({skip_groups = true, jump = true})"),
        );
    });

    // Visual, silent
    v.group(|v| {
        // nvim-dap
        v.noremap(
            "<leader>di",
            &cmd("lua require('dap.ui.variables').visual_hover()"),
        );
    });

    //-----------------------
    // nvim-dap
    //-----------------------
    n.noremap("<F5>", &cmd("lua require('dap').continue()"));
    n.noremap("<leader>dk", &cmd("lua require('dap').step_out()"));
    n.noremap("<leader>dl", &cmd("lua require('dap').step_into()"));
    n.noremap("<leader>dj", &cmd("lua require('dap').step_over()"));
    n.noremap("<leader>db", &cmd("lua require('dap').toggle_breakpoint()"));
    n.noremap(
        "<leader>dB",
        &cmd("lua require('dap').set_breakpoint(vim.fn.input('Breakpoint condition: '))"),
    );
    n.noremap(
        "<leader>lp",
        &cmd("lua require('dap').set_breakpoint(nil, nil, vim.fn.input('Log point message: '))"),
    );
    n.noremap("<leader>dr", &cmd("lua require('dap').repl.open()"));
    n.noremap("<leader>dL", &cmd("lua require('dap').run_last()"));
    n.noremap(
        "<leader>de",
        &cmd("lua require('dap').set_exception_breakpoints({'all'})"),
    );

    // nvim-dap-ui
    n.noremap("<leader>dt", &cmd("lua require('dapui').toggle()"));

    //-----------------------
    // nvim-hlslens
    //-----------------------
    // n.noremap("*", "*<cmd>lua require('hlslens').start()<CR>");
    // n.noremap("#", "#<cmd>lua require('hlslens').start()<CR>");
    // n.noremap("g*", "g*<cmd>lua require('hlslens').start()<CR>");
    // n.noremap("g#", "g#<cmd>lua require('hlslens').start()<CR>");

    //-----------------------
    // vim-ripgrep
    //-----------------------
    n.noremap("<leader>.", ":Rg<Space>");

    //-----------------------
    // vim-test
    //-----------------------
    // n.noremap("<leader>tn", &cmd("TestNearest"));
    // n.noremap("<leader>tf", &cmd("TestFile"));
    // n.noremap("<leader>ta", &cmd("TestSuite"));
    // n.noremap("<leader>tl", &cmd("TestLast"));
    // n.noremap("<leader>tv", &cmd("TestVisit"));

    //-----------------------
    // rust-tools
    //-----------------------
    n.noremap("<leader>rr", &cmd("RustRunnables"));
}

fn cmd<T: Display>(cmd: T) -> String {
    format!("<cmd>{cmd}<CR>")
}

fn telescope_builtin<T: Display>(cmd_string: T) -> String {
    cmd(format!("lua require('telescope.builtin').{cmd_string}()"))
}

fn trouble_toggle<T: Display>(cmd_string: T) -> String {
    cmd(format!("TroubleToggle {cmd_string}"))
}
