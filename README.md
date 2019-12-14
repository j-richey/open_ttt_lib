# open_ttt_lib

[![Crates.io](https://img.shields.io/crates/v/open_ttt_lib.svg)](https://crates.io/crates/open_ttt_lib)
[![Documentation](https://docs.rs/open_ttt_lib/badge.svg)](https://docs.rs/open_ttt_lib)
[![License](https://img.shields.io/crates/l/open_ttt_lib.svg)](https://github.com/j-richey/open_ttt_lib/blob/master/LICENSE.txt)
[![Build Status](https://travis-ci.com/j-richey/open_ttt_lib.svg?branch=master)](https://travis-ci.com/j-richey/open_ttt_lib)
[![Coverage Status](https://coveralls.io/repos/github/j-richey/open_ttt_lib/badge.svg?branch=master)](https://coveralls.io/github/j-richey/open_ttt_lib?branch=master)

Open source Rust library that provides common Tic Tac Toe logic that can be used
by other Rust applications.

:warning: This library is currently under development and has not yet been published
to [crates.io](https://crates.io/).

## Usage
Add this to your `Cargo.toml`:

```toml
[dependencies]
open_ttt_lib = "0.1"
```

Below is an example of using this library. Two AI opponents to play a game of
Tic Tac Toe until one is victorious.

```rust
use open_ttt_lib::{ai, game};
 
fn main() -> Result<(), Box<game::Error>> {
    // Create a game and two AI opponents to play the game.
    let mut game = game::Game::new();
 
    // Rando picks random positions.
    let rando = ai::Opponent::new(1.0);
    // The flawless opponent cannot loose: it fully evaluates every possible
    // move and countermove to pick the best position.
    let flawless_ai = ai::Opponent::new(0.0);
 
    // Have the opponents take turns making moves until the game is over.
    loop {
        match game.state() {
            game::State::PlayerXMove => {
                println!("Rando playing as X turn:");
                game.do_move(rando.get_move(&game).unwrap())?;
            }
            game::State::PlayerOMove => {
                println!("Flawless AI playing as O turn:");
                game.do_move(flawless_ai.get_move(&game).unwrap())?;
            }
            game::State::PlayerXWin(_) => {
                println!("Game Over: Rando playing as X wins!");
                break;
            }
            game::State::PlayerOWin(_) => {
                println!("Game Over: Flawless AI playing as O wins!");
                break;
            }
            game::State::CatsGame => {
                println!("Game Over: cat's game.");
                break;
            }
        };
 
        // Print the game's the board.
        println!("{}", game.board());
    }
 
    Ok(())
}
```

See the [documentation](https://docs.rs/open_ttt_lib/) for additional examples 
and details on using the library.


## Feature Requests
Feel free to request new features. File a feature request describing the feature
you would like and what benefit you will get from the feature. A user story is 
one way to capture this information:

> As a **user** I want **goal/desire** so that **benefit**.


## Reporting Issues
If you find an issue please include the following information in your report:

* Summary of the problem.
* Steps to reproduce the issue or code snippet that demonstrates the issue.
* The expected behavior.
* Version of the library being used.


## Contributing
Contributions are welcome! See [CONTRIBUTING.md](https://github.com/j-richey/open_ttt_lib/blob/master/CONTRIBUTING.md) 
for details on how to contribute to this project.


## License
The library is licensed under the [MIT license](https://github.com/j-richey/open_ttt_lib/blob/master/LICENSE.txt).
