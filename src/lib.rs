//! Provides common Tic Tac Toe logic and artificial intelligence algorithms.

mod game;
pub use game::Game;
pub use game::State;

// Public items provided by this library.
mod board;
pub use board::Board;
pub use board::Position;
pub use board::Size;
pub use board::Owner;
