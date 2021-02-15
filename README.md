# open_ttt_lib

[![Crates.io](https://img.shields.io/crates/v/open_ttt_lib.svg)](https://crates.io/crates/open_ttt_lib)
[![Documentation](https://docs.rs/open_ttt_lib/badge.svg)](https://docs.rs/open_ttt_lib)
[![License](https://img.shields.io/crates/l/open_ttt_lib.svg)](https://github.com/j-richey/open_ttt_lib/blob/master/LICENSE.txt)
[![Build Status](https://travis-ci.com/j-richey/open_ttt_lib.svg?branch=master)](https://travis-ci.com/j-richey/open_ttt_lib)
[![Coverage Status](https://coveralls.io/repos/github/j-richey/open_ttt_lib/badge.svg?branch=master)](https://coveralls.io/github/j-richey/open_ttt_lib?branch=master)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

Open source Rust library containing common Tic Tac Toe functionality.


## Usage
Add this to your `Cargo.toml`:

```toml
[dependencies]
open_ttt_lib = "0.2.0"
```

See the library's [documentation](https://docs.rs/open_ttt_lib/) for complete
details on the library's API.


## Examples
Below is a short example of using this library.

```rust
use open_ttt_lib::{ai, board, game};

fn main() -> Result<(), Box<game::Error>> {
    // Create a game struct to manage the game.
    let mut game = game::Game::new();

    // Pick a position to place a mark. Positions are zero based.
    // An error result is returned if the position is outside the bounds
    // of the board, the position is already owned, or the game is over.
    let position = board::Position { row: 0, column: 2 };
    game.do_move(position)?;

    // do_move() updates the game state. The state indicates the player
    // who gets to place to next mark or, if the game is over, the
    // outcome of the game.
    match game.state() {
        game::State::PlayerXMove => println!("X's turn."),
        game::State::PlayerOMove => println!("O's turn."),
        game::State::PlayerXWin(_) => println!("Game Over: X wins!"),
        game::State::PlayerOWin(_) => println!("Game Over: O wins!"),
        game::State::CatsGame => println!("Game Over: cat's game."),
    };

    // Have an AI opponent pick a move.
    let opponent = ai::Opponent::new(ai::Difficulty::Medium);
    if let Some(ai_position) = opponent.get_move(&game) {
        game.do_move(ai_position)?;
    };

    Ok(())
}
```

The `examples` directory contains additional examples.

To run the examples, clone this repository then use `cargo run --example`. E.g:
```text
git clone https://github.com/j-richey/open_ttt_lib.git
cd open_ttt_lib
cargo run --example single_player
```


## Benchmarks
This library includes benchmarks that you can use to evaluate if the library
fits in with your performance goals. Use `cargo bench` to run the benchmark
suite:

```text
git clone https://github.com/j-richey/open_ttt_lib.git
cd open_ttt_lib
cargo bench
```


## Reporting Issues and Feature Requests
Please report issues and feel free to request new features using this project's
[GitHub issue tracker](https://github.com/j-richey/open_ttt_lib/issues).


## Contributing
Contributions are welcome! See [CONTRIBUTING.md](https://github.com/j-richey/open_ttt_lib/blob/master/CONTRIBUTING.md)
for details on how to contribute to this project.


## License
The library is licensed under the [MIT license](https://github.com/j-richey/open_ttt_lib/blob/master/LICENSE.txt).
