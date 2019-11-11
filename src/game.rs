use std::collections::HashSet;
use std::iter::FromIterator;
use std::error;
use std::fmt;

use crate::board;


// The size of a Tic Tac Toe board
const BOARD_SIZE: board::Size = board::Size{ rows: 3, columns: 3 };

/// Handles management of Tic Tac Toe games.
///
/// This structure is one of the central types provided by the library. It
/// contains the state machine logic, holds the underlying game board, and
/// enforces the rules of Tic Tac Toe.
#[derive(Clone)]
pub struct Game {
    board: board::Board,
    state: State,
    // The state to use when starting the next game.
    next_game_starting_state: State,
}

impl Game {
    /// Creates a new Tic Tac Toe game structure.
    ///
    /// **Note:** use `start_next_game()` for playing consecutive games to ensure
    /// each player gets to start the game.
    pub fn new() -> Self {

        let board = board::Board::new(BOARD_SIZE);
        let state = State::PlayerXMove;
        let next_game_starting_state = Self::get_next_game_starting_state(&state);

        Game{ board, state, next_game_starting_state }
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
        if self.state.is_game_over() || !self.board.contains(position) {
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
        // Ensure the provided position is valid.
        // if !self.can_move(position) {
        //     panic!("TODO: return an InvalidMoveError.");
        // }

        // Mark the given position as being owned by the player whose turn its.
        let new_owner = match self.state {
            State::PlayerXMove => board::Owner::PlayerX,
            State::PlayerOMove => board::Owner::PlayerO,
            _ => return Err(InvalidMoveError{ }),
        };

        match self.board.get_mut(position) {
            Some(owner) => if let board::Owner::None = owner {
                *owner = new_owner;
            } else {
               return Err(InvalidMoveError{ });
            },
            None => return Err(InvalidMoveError{ }),
        };

        // TODO: Update the state or something.
        self.update_state();

        // Return the current state of the game.
        Ok(self.state())
    }

    /// Starts the next game by resetting the state machine ensuring the player
    /// who went second last game goes first next game.
    pub fn start_next_game(&mut self) -> State {
        // Make a new board thus clearing out all existing positions.
        self.board = board::Board::new(BOARD_SIZE);

        // Set the current state and next game's starting state.
        self.state = self.next_game_starting_state.clone();
        self.next_game_starting_state = Self::get_next_game_starting_state(&self.state);

        self.state()
    }

    fn get_next_game_starting_state(current_starting_state: &State) -> State {
        match current_starting_state {
            State::PlayerXMove => State::PlayerOMove,
            State::PlayerOMove => State::PlayerXMove,
            _ => panic!("Attempting to use one of the of the game's victory \
                    states to start a new game. This conditions indicates there \
                    is a bug in the application's logic: a new game cannot also \
                    be game over."),
        }
    }

    fn update_state(&mut self) {

        let mut all_positions = Vec::new();
        // TODO: Look at all the rows, columns, and both diagonals to see if they
        // are all owned by the
        for row in 0..self.board.size().rows {
            let starting_position = board::Position{ row, column: 0 };
            let next_position_fn = |x: board::Position| board::Position{ row: x.row, column: x.column + 1 };
            if let Some(winning_positions) = Self::check_sequence(&mut self.board.sequence(starting_position, next_position_fn)) {
                all_positions.extend(winning_positions);
            }
        }

        for column in 0..self.board.size().columns {
            let starting_position = board::Position{ row: 0, column, };
            let next_position_fn = |x: board::Position| board::Position{ row: x.row + 1, column: x.column };
            if let Some(winning_positions) = Self::check_sequence(&mut self.board.sequence(starting_position, next_position_fn)) {
                all_positions.extend(winning_positions);
            }
        }

        let starting_position = board::Position{ row: 0, column: 0 };
        let next_position_fn = |x: board::Position| board::Position{ row: x.row + 1, column: x.column + 1 };
        if let Some(winning_positions) = Self::check_sequence(&mut self.board.sequence(starting_position, next_position_fn)) {
            all_positions.extend(winning_positions);
        }

        // TODO: AHHHH, we might need to rethink the type here!
        let starting_position = board::Position{ row: 0, column: 2 };
        let next_position_fn = |x: board::Position| {
            let last_position = board::Position{ row: 2, column: 0 };
            if x != last_position {
                board::Position{ row: x.row + 1, column: x.column - 1 }
            } else {
                board::Position{ row: 100, column: 100 }
            }
        };
        if let Some(winning_positions) = Self::check_sequence(&mut self.board.sequence(starting_position, next_position_fn)) {
            all_positions.extend(winning_positions);
        }

        let winning_positions: HashSet<board::Position> = all_positions.iter().cloned().collect();

        if !winning_positions.is_empty() {
            match self.board.get(*winning_positions.iter().next().unwrap()).unwrap() {
                board::Owner::PlayerX => self.state = State::PlayerXWin(winning_positions),
                board::Owner::PlayerO => self.state = State::PlayerOWin(winning_positions),
                board::Owner::None => panic!("AHHHHH!"),
            };
        } else if self.board.iter().find(|(_position, owner)| *owner == board::Owner::None).is_none() {
            // TODO: check for cat's game! AHHHH.
            self.state = State::CatsGame;
        } else {
            match self.state {
                State::PlayerXMove => self.state = State::PlayerOMove,
                State::PlayerOMove => self.state = State::PlayerXMove,
                _ =>  panic!("AHHHHH!"),
            };
        }
    }

    fn check_sequence(sequence: &mut board::Sequence) -> Option<Vec<board::Position>> {
        let mut winning_positions = Vec::new();
        if let Some((initial_position, initial_owner)) = sequence.next() {
            winning_positions.push(initial_position);
            for (position, owner) in sequence {
                if owner == initial_owner  && owner != board::Owner::None {
                    winning_positions.push(position);
                }
                else {
                    return None;
                }
            }
            return Some(winning_positions);
        }
        else{
            return None;
        }
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

    // Helper function for setting the owner of the given positions.
    //
    // This function does not modify the state of the game or check to see if the
    // provided positions already have an owner.
    fn set_positions(game: &mut Game, owner: board::Owner, positions: &[board::Position]) {
        for position in positions {
            *game.board.get_mut(*position).unwrap() = owner;
        }
    }

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
    fn game_do_move_when_game_over_should_return_error() {
        let position = board::Position{ row: 0, column: 0 };
        let mut game = Game::new();
        // Set a game over state.
        game.state = State::CatsGame;

        let move_result = game.do_move(position);

        assert!(move_result.is_err());
    }

    #[test]
    fn game_do_move_when_position_outside_board_should_return_error() {
        let position_outside_board = board::Position{ row: 100, column: 100 };
        let mut game = Game::new();

        let move_result = game.do_move(position_outside_board);

        assert!(move_result.is_err());
    }

    #[test]
    fn game_do_move_when_three_X_in_row_should_return_player_X_win() {
        let mut game = Game::new();
        game.state = State::PlayerXMove;
        // Configure the board so the next move is a winning move.
        let existing_positions = [
            board::Position{ row: 0, column: 0 },
            board::Position{ row: 0, column: 1 }];
        set_positions(&mut game, board::Owner::PlayerX, &existing_positions);
        let winning_position = board::Position{ row: 0, column: 2 };
        // Build the set of expected winning positions.
        let mut winning_positions: HashSet<board::Position> =
            existing_positions.iter().cloned().collect();
        winning_positions.insert(winning_position);
        let expected_state = State::PlayerXWin(winning_positions);

        // Do the final move to get three X's in a row.
        let actual_state = game.do_move(winning_position).unwrap();

        assert_eq!(expected_state, actual_state);
    }

    #[test]
    fn game_do_move_when_three_X_in_column_should_return_player_X_win() {
        let mut game = Game::new();
        game.state = State::PlayerXMove;
        // Configure the board so the next move is a winning move.
        let existing_positions = [
            board::Position{ row: 0, column: 0 },
            board::Position{ row: 1, column: 0 }];
        set_positions(&mut game, board::Owner::PlayerX, &existing_positions);
        let winning_position = board::Position{ row: 2, column: 0 };
        // Build the set of expected winning positions.
        let mut winning_positions: HashSet<board::Position> =
            existing_positions.iter().cloned().collect();
        winning_positions.insert(winning_position);
        let expected_state = State::PlayerXWin(winning_positions);

        // Do the final move to get three X's in a column.
        let actual_state = game.do_move(winning_position).unwrap();

        assert_eq!(expected_state, actual_state);
    }

    #[test]
    fn game_do_move_when_three_X_in_top_left_to_bottom_right_diagonal_should_return_player_X_win() {
        let mut game = Game::new();
        game.state = State::PlayerXMove;
        // Configure the board so the next move is a winning move.
        let existing_positions = [
            board::Position{ row: 0, column: 0 },
            board::Position{ row: 1, column: 1 }];
        set_positions(&mut game, board::Owner::PlayerX, &existing_positions);
        let winning_position = board::Position{ row: 2, column: 2 };
        // Build the set of expected winning positions.
        let mut winning_positions: HashSet<board::Position> =
            existing_positions.iter().cloned().collect();
        winning_positions.insert(winning_position);
        let expected_state = State::PlayerXWin(winning_positions);

        // Do the final move to get three X's in a diagonal.
        let actual_state = game.do_move(winning_position).unwrap();

        assert_eq!(expected_state, actual_state);
    }

    #[test]
    fn game_do_move_when_three_X_in_top_right_to_bottom_left_diagonal_should_return_player_X_win() {
        let mut game = Game::new();
        game.state = State::PlayerXMove;
        // Configure the board so the next move is a winning move.
        let existing_positions = [
            board::Position{ row: 0, column: 2 },
            board::Position{ row: 1, column: 1 }];
        set_positions(&mut game, board::Owner::PlayerX, &existing_positions);
        let winning_position = board::Position{ row: 2, column: 0 };
        // Build the set of expected winning positions.
        let mut winning_positions: HashSet<board::Position> =
            existing_positions.iter().cloned().collect();
        winning_positions.insert(winning_position);
        let expected_state = State::PlayerXWin(winning_positions);

        // Do the final move to get three X's in a diagonal.
        let actual_state = game.do_move(winning_position).unwrap();

        assert_eq!(expected_state, actual_state, "Board is: \n{}", game.board());
    }

    #[test]
    fn game_do_move_when_both_winning_row_and_diagonal_should_contain_all_winning_positions() {
        let mut game = Game::new();
        game.state = State::PlayerXMove;
        // Create a board where player X is about to win by with both a diagonal and row.
        let existing_positions = [
            board::Position{ row: 0, column: 1 },
            board::Position{ row: 0, column: 2 },
            board::Position{ row: 1, column: 1 },
            board::Position{ row: 2, column: 2 }];
        set_positions(&mut game, board::Owner::PlayerX, &existing_positions);
        let winning_position = board::Position{ row: 0, column: 0 };
        // Build the set of expected winning positions.
        let mut winning_positions: HashSet<board::Position> =
            existing_positions.iter().cloned().collect();
        winning_positions.insert(winning_position);
        let expected_state = State::PlayerXWin(winning_positions);

        // Do the final move to get three X's in a diagonal.
        let actual_state = game.do_move(winning_position).unwrap();

        assert_eq!(expected_state, actual_state);
    }

    // We test at lease one of the victory conditions with player O to ensure
    // it works the same as player X.
    #[test]
    fn game_do_move_when_three_O_in_row_should_return_player_O_win() {
        let mut game = Game::new();
        game.state = State::PlayerOMove;
        // Configure the board so the next move is a winning move.
        let existing_positions = [
            board::Position{ row: 1, column: 0 },
            board::Position{ row: 1, column: 1 }];
        set_positions(&mut game, board::Owner::PlayerO, &existing_positions);
        let winning_position = board::Position{ row: 1, column: 2 };
        // Build the set of expected winning positions.
        let mut winning_positions: HashSet<board::Position> =
            existing_positions.iter().cloned().collect();
        winning_positions.insert(winning_position);
        let expected_state = State::PlayerOWin(winning_positions);

        // Do the final move to get three O's in a row.
        let actual_state = game.do_move(winning_position).unwrap();

        assert_eq!(expected_state, actual_state);
    }

    #[test]
    fn game_do_move_when_last_position_filled_should_return_cats_game() {
        // Configure the board so there are never three marks in a row and the
        // next move fills the board thus making no more free positions.
        // The board is configured as follows:
        //  +---+---+---+
        //  | X | O | X |
        //  +---+---+---+
        //  | X | O | O |
        //  +---+---+---+
        //  | O | X | X*|
        //  +---+---+---+
        // Where the X* is the last position.
        let mut game = Game::new();
        game.state = State::PlayerXMove;
        let existing_X_positions = [
            board::Position{ row: 0, column: 0 },
            board::Position{ row: 0, column: 2 },
            board::Position{ row: 1, column: 0 },
            board::Position{ row: 2, column: 1 }];
        set_positions(&mut game, board::Owner::PlayerX, &existing_X_positions);
        let existing_O_positions = [
            board::Position{ row: 0, column: 1 },
            board::Position{ row: 1, column: 1 },
            board::Position{ row: 1, column: 2 },
            board::Position{ row: 2, column: 0 }];
        set_positions(&mut game, board::Owner::PlayerO, &existing_O_positions);
        let last_position = board::Position{ row: 2, column: 2 };
        let expected_state = State::CatsGame;

        // Fill the final position so there are no more moves.
        let actual_state = game.do_move(last_position).unwrap();

        assert_eq!(expected_state, actual_state);
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
