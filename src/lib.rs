//! Provides common Tic Tac Toe logic and artificial intelligence algorithms.
//!
//! # Examples
//! The example below creates a Tic Tac Toe game and two AI opponents to play
//! the game.
//! ```
//! # use open_ttt_lib::game;
//! #
//! # fn main() -> Result<(), Box<game::Error>> {
//! use open_ttt_lib::{ai, game};
//!
//! // Create a game and two AI opponents to play the game.
//! let mut game = game::Game::new();
//!
//! // Rando picks random positions.
//! let rando = ai::Opponent::new(1.0);
//! // The flawless opponent cannot loose: it fully evaluates every possible
//! // move and countermove in the game.
//! let flawless_ai = ai::Opponent::new(0.0);
//!
//! // Have the opponents take turns making moves until the game is over.
//! loop {
//!     match game.state() {
//!         game::State::PlayerXMove => {
//!             println!("Rando playing as X turn:");
//!             game.do_move(rando.get_move(&game).unwrap())?;
//!         }
//!         game::State::PlayerOMove => {
//!             println!("Flawless AI playing as O turn:");
//!             game.do_move(flawless_ai.get_move(&game).unwrap())?;
//!         }
//!         game::State::PlayerXWin(_) => {
//!             println!("Game Over: Rando playing as X wins!");
//!             break;
//!         }
//!         game::State::PlayerOWin(_) => {
//!             println!("Game Over: Flawless AI playing as O wins!");
//!             break;
//!         }
//!         game::State::CatsGame => {
//!             println!("Game Over: cat's game.");
//!             break;
//!         }
//!     };
//!
//!     // Print the game's the board.
//!     println!("{}", game.board());
//! }
//! #   Ok(())
//! # }
//! ```

pub mod ai;
pub mod board;
pub mod game;
