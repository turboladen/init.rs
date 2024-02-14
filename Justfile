default: check-in-nvim

benchmark:
  cargo build --release
  hyperfine --warmup 4 "nvim +q"

fix-formatting:
  cargo +nightly fmt

check-in-nvim:
  cargo build --release
  nvim

# Note that the copy must be run while no instances of nvim are running (well,
# no instances that have this .so file loaded).
build-and-copy:
  cargo build --release
  cp -f target/release/libinit_rs.dylib ~/.config/nvim/lua/init_rs.so

# vim:ft=just
