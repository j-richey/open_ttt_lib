use std::error;
use std::fmt;

use crate::board;

/// Handles management of Tic Tac Toe games.
///
/// This structure is one of the central types provided by the library. It
/// contains the state machine logic, holds the underlying game board, and
/// enforces the rules of Tic Tac Toe.
#[derive(Clone)]
pub struct Game {

}

impl Game {
    /// Creates a new Tic Tac Toe game structure.
    ///
    /// **Note:** use `start_next_game()` for playing consecutive games to ensure
    /// each player gets to start the game.
    pub fn new() -> Game {
        panic!("This function is not implemented!");
    }

    /// Gets the board associated with the game.
    pub fn board(&self) -> &board::Board {
        panic!("This function is not implemented!");
    }

    /// Gets an iterator that iterates the squares in the game that do not have
    ///  an owner.
    pub fn free_squares(&self) -> FreeSquares {
        panic!("This function is not implemented!");
    }

    /// Gets an iterator that iterates over all the sets of squares that, if all
    /// owned by a player, would make the player victorious.
    ///
    /// E.g. this gets all the rows, columns, and both diagonals.
    pub fn victory_sets(&self) -> VictorySets {
        panic!("This function is not implemented!");
    }

    /// Gets the current state of the game.
    pub fn state(&self) -> State {
        panic!("This function is not implemented!");
    }

    /// Indicates if the square at the indicated position can be marked as owned.
    ///
    /// That is, if `can_move()` returns true then `do_move()` is guaranteed to
    /// not return an error.
    pub fn can_move(&self, position: board::Position) -> bool {
        panic!("This function is not implemented!");
    }

    /// Marks the indicated square as being owned by the current player.
    ///
    /// The state of the game is updated as a side effect of do_move(). The new
    /// state is returned if the move was successful.
    ///
    /// # Errors
    /// An error is returned if the indicated position is already owned or if
    /// the game is over.
    pub fn do_move(&mut self, position: board::Position) -> Result<State, InvalidMoveError> {
        panic!("This function is not implemented!");
    }

    /// Starts the next game by resetting the state machine ensuring the player
    /// who went second last game goes first next game.
    pub fn start_next_game(&mut self) -> State {
        panic!("This function is not implemented!");
    }

}


/// An iterator over free squares in a `Game`.
pub struct FreeSquares {

}

impl Iterator for FreeSquares {
    type Item = board::Square;

    fn next(&mut self) -> Option<Self::Item> {
        panic!("This function is not implemented!");
    }
}

/// An iterator over all the sets of squares that, if all owned by a player,
/// would make the player victorious.
pub struct VictorySets {

}

impl Iterator for VictorySets {
    type Item = Vec<board::Square>;

    fn next(&mut self) -> Option<Self::Item> {
        panic!("This function is not implemented!");
    }
}

/// Error used when an invalid move is attempted.
#[derive(Debug)]
pub struct InvalidMoveError {
    // TODO: Perhaps the current state of the game and the square at the
    // position of the move in question?
}

impl fmt::Display for InvalidMoveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("This function is not implemented!");
    }
}

impl error::Error for InvalidMoveError {

}


/// Indicates the state of the game.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum State {
    /// Player X's turn to mark an empty square.
    PlayerXMove,

    /// Player O's turn to mark an empty square.
    PlayerOMove,

    /// Player X has won the game.
    PlayerXWin,

    /// Player O has won the game.
    PlayerOWin,

    /// The game has ended in a draw where there are no winners.
    CatsGame,
}

impl State {

    /// Indicates if the state represents one of the game over states.
    ///
    /// If either player has won or it is a cat's game then `true` is returned;
    /// otherwise, `false` is returned.
    pub fn is_game_over(&self) -> bool {
        match self {
            State::PlayerXMove => false,
            State::PlayerOMove => false,
            State::PlayerXWin => true,
            State::PlayerOWin => true,
            State::CatsGame => true,
        }
    }
}


#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_is_game_over_when_player_X_move_should_be_false() {
        let state = State::PlayerXMove;
        let expected_is_game_over = false;

        let actual_is_game_over = state.is_game_over();

        assert_eq!(expected_is_game_over, actual_is_game_over);
    }

    #[test]
    fn state_is_game_over_when_player_O_move_should_be_false() {
        let state = State::PlayerOMove;
        let expected_is_game_over = false;

        let actual_is_game_over = state.is_game_over();

        assert_eq!(expected_is_game_over, actual_is_game_over);
    }

    #[test]
    fn state_is_game_over_when_player_X_win_should_be_true() {
        let state = State::PlayerXWin;
        let expected_is_game_over = true;

        let actual_is_game_over = state.is_game_over();

        assert_eq!(expected_is_game_over, actual_is_game_over);
    }

    #[test]
    fn state_is_game_over_when_player_O_win_should_be_true() {
        let state = State::PlayerOWin;
        let expected_is_game_over = true;

        let actual_is_game_over = state.is_game_over();

        assert_eq!(expected_is_game_over, actual_is_game_over);
    }

    #[test]
    fn state_is_game_over_when_cats_game_should_be_true() {
        let state = State::CatsGame;
        let expected_is_game_over = true;

        let actual_is_game_over = state.is_game_over();

        assert_eq!(expected_is_game_over, actual_is_game_over);
    }

}
