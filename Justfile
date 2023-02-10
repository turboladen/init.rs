default: check-in-nvim

benchmark:
  cargo build --release
  hyperfine --warmup 4 "nvim +q"

fix-formatting:
  cargo +nightly fmt

check-in-nvim:
  cargo build --release
  nvim

build-and-copy:
  cargo build --release
  cp -f target/release/libinit_rs.dylib ~/.config/nvim/lua/init_rs.so

# vim:ft=just
