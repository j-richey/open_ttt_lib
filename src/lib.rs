//! # Overview
//! Tic Tac Toe is a game of strategy where two players, X and O, take turns
//! placing their mark in a 3 x 3 gird. The first player to get three marks in a
//! row, column, or diagonal wins the game. The game can also end in a draw,
//! known as a *cat's game*.
//!
//! This library contains logic to enforce the rules of Tic Tac Toe, manage the
//! game's state, and provides artificial intelligence algorithms for single
//! player games.
//!
//! # Usage
//! The [`Game`](game/struct.Game.html) structure is the central type provided
//! by this crate. It enforces the rules and manages the
//! [`State`](game/enum.State.html) of the game.
//! The [`Opponent`](ai/struct.Opponent.html) structure provides support for
//! single player games.
//!
//! # Example
//! ```
//! # use open_ttt_lib::game;
//! # fn main() -> Result<(), Box<game::Error>> {
//! use open_ttt_lib::{ai, board, game};
//! // Create a game to manage the game.
//! let mut game = game::Game::new();
//!
//! // Pick a position to place a mark.
//! let position = board::Position { row: 0, column: 0 };
//! game.do_move(position)?;
//!
//! // Get the state of the game to see who's turn it is or if the game is over.
//! match game.state() {
//!     game::State::PlayerXMove => println!("X's turn."),
//!     game::State::PlayerOMove => println!("O's turn."),
//!     game::State::PlayerXWin(_) => println!("Game Over: X wins!"),
//!     game::State::PlayerOWin(_) => println!("Game Over: O wins!"),
//!     game::State::CatsGame => println!("Game Over: cat's game."),
//! };
//!
//! // Have an unbeatable opponent pick a move.
//! let mistake_probability = 0.0;
//! let opponent = ai::Opponent::new(mistake_probability);
//! if let Some(ai_position) = opponent.get_move(&game) {
//!     game.do_move(ai_position)?;
//! }
//!
//! # Ok(())
//! # }
//! ```

#![doc(html_root_url = "https://docs.rs/open_ttt_lib/0.1.1")]

pub mod ai;
pub mod board;
pub mod game;

// Ensure the examples in the README file also work as expected.
extern crate doc_comment;
doc_comment::doctest!("../README.md", examples_in_readme);
