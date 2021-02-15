# Change Log
All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html)
as described for Rust libraries in
[RFC 1105](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md).


## [0.2.1] - 2021-02-15

### Added
* Documented the library does not use unsafe code. This is enforced with the
  `#![forbid(unsafe_code)]` attribute. 

### Fixed
* Several documentation typos.


## [0.2.0] - 2020-07-18

### Added
* `ai::Difficulty` enumeration containing predefined AI difficulty levels
  including **Easy**, **Medium**, and **Hard** . The **Custom** variant allows
  full control over the AI opponent's difficulty.
* `ai_difficulties` example shows how the different AI difficulties compare and
  demonstrates how to create custom difficulties.

### Changed
* The `ai::Opponent::new` constructor takes a `ai::Difficulty` variant instead
  of a `mistake_probability` number.
* Improved the worst case AI move time by 150x.


## [0.1.2] - 2020-04-18

### Added
* `single_player` example that shows how to use the library to create a complete
  console based Tic Tac Toe game.
* Include a brief overview on the rules of Tic Tac Toe the library documentation.


## [0.1.1] - 2019-12-22

### Fixed
* The crate's README file not being displayed on crates.io


## [0.1.0] - 2019-12-21
First release!

## Added
* `game::Game` structure for handling management of Tic Tac Toe games.
* `ai::Opponent` structure that provides a computer controlled AI opponent.
* `board` module containing helper types for working with Tic Tac Toe boards.
