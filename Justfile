benchmark:
  cargo build --release
  hyperfine --warmup 4 "nvim +q"

fix-formatting:
  cargo +nightly fmt
