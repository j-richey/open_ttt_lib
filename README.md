# open_ttt_lib
[![Build Status](https://travis-ci.com/j-richey/open_ttt_lib.svg?branch=master)](https://travis-ci.com/j-richey/open_ttt_lib)
[![Coverage Status](https://coveralls.io/repos/github/j-richey/open_ttt_lib/badge.svg?branch=master)](https://coveralls.io/github/j-richey/open_ttt_lib?branch=master)

Open source Rust library that provides common Tic Tac Toe logic that can be used
by other Rust applications.

:warning: This library is currently under development and has not yet been published
to [crates.io](https://crates.io/).


## Reporting Issues
If you find an issue please include the following information in your report:

* Summary of the problem.
* Steps to reproduce the issue or code snippet that demonstrates the issue.
* The expected behavior.
* Version of the library being used.


## Developing
This library is developed using the [Rust programming language](https://www.rust-lang.org/).
Once Rust is installed on your system you can build and test the library with `cargo`:

```
cargo test
```

All pull requests are checked with [Clippy](https://github.com/rust-lang/rust-clippy)
and [rustfmt](https://github.com/rust-lang/rustfmt) to help ensure code
consistency and quality.

These tools can be installed and run locally with:

```
rustup component add clippy
rustup component add rustfmt
cargo clippy
cargo fmt
```


## License
This library is licensed under the [MIT license](LICENSE.txt).
