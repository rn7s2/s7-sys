# s7-sys

[![crates.io](https://img.shields.io/crates/v/s7-sys.svg)](https://crates.io/crates/s7-sys)

Rust bindings to s7 Scheme C API

About s7 (and API document):
[https://ccrma.stanford.edu/software/snd/snd/s7.html](https://ccrma.stanford.edu/software/snd/snd/s7.html).

## Prerequisites:

- `Clang` toolchain is required to generate the bindings

## Examples:

- REPL [examples/repl.rs](examples/repl.rs):

  ```
  cargo run --example repl
  ```

  screenshot on Windows:

  <img src="examples/repl.jpeg" style="width:550px" />
