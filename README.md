# init.rs

The portion of neovim config that's written in Rust.

## Setup

1. Build the library: `cargo build --release`
2. Copy the library to `~/.cargo/nvim/lua/init_rs.so` (use `.so` even if Mac)

(or do both by `just build-and-copy`)

3. Call any functions in Lua using `require("init_rs").do_a_thing()`
