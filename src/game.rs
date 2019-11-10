use std::collections::HashSet;
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
    board: board::Board,
    state: State,
}

impl Game {
    /// Creates a new Tic Tac Toe game structure.
    ///
    /// **Note:** use `start_next_game()` for playing consecutive games to ensure
    /// each player gets to start the game.
    pub fn new() -> Game {
        const BOARD_SIZE: board::Size = board::Size{ rows: 3, columns: 3 };
        let board = board::Board::new(BOARD_SIZE);
        let state = State::PlayerXMove;

        Game{ board, state }
    }

    /// Gets the board associated with the game.
    pub fn board(&self) -> &board::Board {
        &self.board
    }

    /// Gets the current state of the game.
    pub fn state(&self) -> State {
        self.state.clone()
    }

    /// Gets an iterator over the free positions that do not have an owner and
    /// thus can be provided to `do_move()`.
    ///
    /// When the game is over there are no free positions.
    pub fn free_positions(&self) -> FreePositions {
        FreePositions {
            board_iter: self.board.iter(),
            is_game_over: self.state.is_game_over()
        }
    }

    /// Indicates if the square at the indicated position can be marked as owned.
    ///
    /// That is, if `can_move()` returns true then `do_move()` is guaranteed to
    /// not return an error. False is returned if the position is owned, if the
    /// game is over, or if the position is outside the area of the board.
    pub fn can_move(&self, position: board::Position) -> bool {
        // If the game is over or if the position is outside the board area the
        // a move cannot be performed. Otherwise, a move can be performed if
        // the position has no owner.
        if self.state.is_game_over() {
            false
        }
        else if !self.board.contains(position) {
            false
        }
        else {
            self.board().get(position).unwrap() == board::Owner::None
        }
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


/// An iterator over free positions in a `Game`; that is positions do not have an owner.
pub struct FreePositions<'a> {
    board_iter: board::Iter<'a>,
    is_game_over: bool,
}

impl Iterator for FreePositions<'_> {
    type Item = board::Position;

    fn next(&mut self) -> Option<Self::Item> {
        // There are no free positions if the game is over.
        if self.is_game_over {
            return None;
        }

        // Loop over all the positions looking for ones that are not Owned.
        loop {
            match self.board_iter.next() {
                Some((position, owner)) => {
                    if owner == board::Owner::None {
                        return Some(position);
                    }
                }
                None => return None
            }
        }
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
///
/// The set of positions provided to PlayerXWin and PlayerOWin contain all the
/// positions that contributed to the victory. Usually, this will be positions
/// representing a row, column, or diagonal. However, there are some situations
/// where more than one row, column, or diagonal contributed to a victory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    /// Player X's turn to mark a free position.
    PlayerXMove,

    /// Player O's turn to mark a free position.
    PlayerOMove,

    /// Player X has won the game. The set of positions that contributed to the
    /// win are provided as the enum value.
    PlayerXWin(HashSet<board::Position>),

    /// Player O has won the game. The set of positions that contributed to the
    /// win are provided as the enum value.
    PlayerOWin(HashSet<board::Position>),

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
            State::PlayerXWin(_) => true,
            State::PlayerOWin(_) => true,
            State::CatsGame => true,
        }
    }
}


#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_new_should_create_3x3_board() {
        let expected_size = board::Size { rows: 3, columns: 3 };

        let game = Game::new();
        let actual_size = game.board().size();

        assert_eq!(expected_size, actual_size);
    }

    #[test]
    fn game_new_should_not_be_game_over_state() {
        let expected_is_game_over = false;

        let game = Game::new();
        let actual_is_game_over = game.state().is_game_over();

        assert_eq!(expected_is_game_over, actual_is_game_over);
    }

    #[test]
    fn game_new_should_all_positions_should_be_free() {
        let game = Game::new();
        let board_size = game.board().size();
        // Calculate the expected number of free positions. This is equal to the
        // total number of positions.
        let expected_free_squares = board_size.rows * board_size.columns;

        let actual_free_squares = game.free_positions().count();

        assert_eq!(expected_free_squares, actual_free_squares);
    }

    #[test]
    fn game_free_positions_should_not_contain_any_owned_positions() {
        let mut game = Game::new();
        // Configure the board so each player owns some positions.
        *game.board
            .get_mut(board::Position{ row: 0, column: 0 })
            .unwrap() = board::Owner::PlayerX;
        *game.board
            .get_mut(board::Position{ row: 0, column: 1 })
            .unwrap() = board::Owner::PlayerO;
        let expected_num_owned_positions = 0;

        let actual_num_owned_positions = game.free_positions().filter(
            |x| game.board().get(*x).unwrap() != board::Owner::None).count();

        assert_eq!(expected_num_owned_positions, actual_num_owned_positions);
    }

    #[test]
    fn game_free_positions_when_game_over_should_be_none() {
        let mut game = Game::new();
        // Force the game to be in a game over state.
        game.state = State::CatsGame;
        let expected_num_free_positions = 0;

        let actual_num_free_positions = game.free_positions().count();

        assert_eq!(expected_num_free_positions, actual_num_free_positions);
    }

    #[test]
    fn game_can_move_when_unowned_positions_should_be_true() {
        let position = board::Position{ row: 0, column: 0, };
        let game = Game::new();
        // No positions are owned yet.
        let expected_can_move = true;

        let actual_can_move = game.can_move(position);

        assert_eq!(expected_can_move, actual_can_move);
    }

    #[test]
    fn game_can_move_when_owned_positions_should_be_false() {
        let position = board::Position{ row: 0, column: 0, };
        let mut game = Game::new();
        // Give the position an owner.
        *game.board.get_mut(position).unwrap() = board::Owner::PlayerX;
        let expected_can_move = false;

        let actual_can_move = game.can_move(position);

        assert_eq!(expected_can_move, actual_can_move);
    }

    #[test]
    fn game_can_move_when_game_over_should_be_false() {
        let position = board::Position{ row: 0, column: 0, };
        let mut game = Game::new();
        game.state = State::CatsGame;
        let expected_can_move = false;

        let actual_can_move = game.can_move(position);

        assert_eq!(expected_can_move, actual_can_move);
    }

    #[test]
    fn game_can_move_when_outside_game_board_should_be_false() {
        let position_outside_game_board = board::Position{ row: 1000, column: 1000, };
        let game = Game::new();
        let expected_can_move = false;

        let actual_can_move = game.can_move(position_outside_game_board);

        assert_eq!(expected_can_move, actual_can_move);
    }

    #[test]
    fn game_do_move_when_free_position_and_not_game_over_should_return_next_player_move_state() {
        let mut game = Game::new();
        game.state = State::PlayerXMove;
        let expected_state = State::PlayerOMove;

        let actual_state = game.do_move(board::Position{ row: 0, column: 0 }).unwrap();

        assert_eq!(expected_state, actual_state);
    }

    #[test]
    fn game_do_move_when_owned_position_should_return_error() {
        let position = board::Position{ row: 0, column: 0 };
        let mut game = Game::new();
        // Mark the position as being owned by doing a move on it.
        game.do_move(position).unwrap();

        // Do a second move to the same position.
        let move_result = game.do_move(position);

        assert!(move_result.is_err());
    }

    #[test]
    fn game_do_move_when_winning_move_should_return_game_over_state() {
        let mut game = Game::new();
        game.state = State::PlayerXMove;
        // Configure the board so the next move is a winning move.
        *game.board.get_mut(board::Position{ row: 0, column: 0 }).unwrap() = board::Owner::PlayerX;
        *game.board.get_mut(board::Position{ row: 0, column: 1 }).unwrap() = board::Owner::PlayerX;

        // Do the final move ot get three X's in a row.
        let actual_state = game.do_move(board::Position{ row: 0, column: 2 }).unwrap();

        assert!(actual_state.is_game_over());
    }

    #[test]
    fn game_start_next_game_should_ensure_player_who_went_went_second_goes_first_next_game() {
        let mut game = Game::new();
        let first_player_last_game = game.state();

        let first_player_next_game = game.start_next_game();

        assert_ne!(first_player_last_game, first_player_next_game);
    }

    #[test]
    fn game_start_next_game_should_alternate_between_players_who_go_first() {
        let mut game = Game::new();

        let player_1 = game.start_next_game();
        let player_2 = game.start_next_game();

        assert_ne!(player_1, player_2);
    }

    #[test]
    fn game_start_next_game_when_game_not_over_should_start_next_game() {
        let mut game = Game::new();
        let expected_is_game_over = false;

        game.start_next_game();
        let actual_is_game_over = game.state().is_game_over();

        assert_eq!(expected_is_game_over, actual_is_game_over);
    }


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
        let state = State::PlayerXWin(Default::default());
        let expected_is_game_over = true;

        let actual_is_game_over = state.is_game_over();

        assert_eq!(expected_is_game_over, actual_is_game_over);
    }

    #[test]
    fn state_is_game_over_when_player_O_win_should_be_true() {
        let state = State::PlayerOWin(Default::default());
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
