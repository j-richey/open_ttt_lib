use std::fmt;

/// Represents the Tic Tac Toe board providing multiple ways to access individual squares.
#[derive(Clone)]
pub struct Board {
    squares: Vec<Vec<Owner>>,
}

impl Board {

    /// Constructs a new board of the given size.
    ///
    /// # Panics
    /// The minimum board size is 1x1. Panics if either the number of rows or
    /// columns is less than one.
    pub fn new(size: Size) -> Board {
        const MIN_BOARD_SIZE: Size = Size{ rows: 1, columns: 1 };

        // Validate the provided board size.
        if size.rows < MIN_BOARD_SIZE.rows || size.columns < MIN_BOARD_SIZE.columns {
            panic!("Invalid board size of '{:?}' provided. The minium board size is '{:?}'",
                size, MIN_BOARD_SIZE);
        }

        let mut squares: Vec<Vec<Owner>> = Vec::new();
        for _ in 0..size.rows {
            let mut row: Vec<Owner> = Vec::new();
            for _ in 0..size.columns {
                row.push(Owner::None);
            }
            squares.push(row);
        }

        Board { squares, }
    }

    /// Gets the size of the board.
    pub fn size(&self) -> Size {
        let rows = self.squares.len();

        // The length of the first row happens to be the number of columns.
        let columns = match self.squares.get(0) {
            Some(row) => row.len(),
            None => 0,
        };

        Size{ rows, columns }
    }

    /// Returns `true` if the board contains the given position.
    ///
    /// Note that positions are zero based.
    ///
    /// # Examples
    /// ```
    /// let board = open_ttt_lib::Board::new(open_ttt_lib::Size { rows: 3, columns: 3 });
    ///
    /// assert_eq!(true, board.contains(open_ttt_lib::Position { row: 2, column: 2 }));
    /// // Since the positions are zero indexed, the board does not
    /// // contain the following position.
    /// assert_eq!(false, board.contains(open_ttt_lib::Position { row: 3, column: 3 }));
    /// ```
    pub fn contains(&self, position: Position) -> bool {
        let size = self.size();

        position.row < size.rows && position.column < size.columns
    }

    /// Returns a copy of the square at the indicated position or `None`
    /// if the board does not contain the provided position.
    pub fn get(&self, position: Position) -> Option<Square> {
        if self.contains(position) {
            let owner = self.squares[position.row][position.column];
            Some(Square{ owner, position })
        } else {
            None
        }
    }

    /// Replaces the square, denoted by its position, with the provided square.
    ///
    /// # Panics
    /// Panics if the board does not contain a square at the provided square's
    /// position.
    pub fn set(&mut self, square: Square) {
        if self.contains(square.position) {
            self.squares[square.position.row][square.position.column] = square.owner;
        } else {
            panic!("The position of the provided square, {:?} is outside the boards size of {:?}.",
                square.position, self.size());
        }
    }

    /// Gets an iterator over all the squares in a `Board`.
    pub fn iter(&self) -> Iter {
        Iter {
            board: &self,
            position: Position{ row: 0, column: 0 },
        }
    }

    // Helper function for displaying boards that writes the separators between rows.
    fn write_row_separator(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for _ in 0..self.size().columns {
            write!(f, "+---")?;
        }
        writeln!(f, "+")
    }

    // Helper function for displaying boards that writes the content of the row.
    fn write_row_content(&self, f: &mut fmt::Formatter<'_>, row: &Vec<Owner>) -> fmt::Result {
        for owner in row {
            match owner {
                Owner::PlayerX => write!(f, "| X "),
                Owner::PlayerO => write!(f, "| O "),
                Owner::None => write!(f, "|   "),
            }?;
        }
        // Write the last vertical bar to close off the cell.
        writeln!(f, "|")
    }
}

impl fmt::Display for Board {
    /// This provides simple formatted output of the board.
    ///
    /// This is suitable for use in simple console applications or debugging
    /// purposes.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.squares {
            self.write_row_separator(f)?;
            self.write_row_content(f, row)?;
        }

        // Write the final separator to finish off the board.
        self.write_row_separator(f)
    }
}


/// An iterator over the squares in a `Board`.
pub struct Iter<'a> {
    board: &'a Board,
    position: Position,
}

impl Iterator for Iter<'_> {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        // Get the square at the current position.
        let next_item = self.board.get(self.position);

        // Calculate the next position by incrementing the column then checking
        // if we need to wrap to the next row.
        let board_size = self.board.size();
        self.position.column += 1;
        if self.position.column >= board_size.columns {
            self.position.column = 0;
            self.position.row += 1;
        }

        // We also check for the row to exceed the board size. However, in this
        // case we let the position jump to the next row after the board area.
        // This causes next() to return None for the next item ensuring iteration
        // is stopped.
        if self.position.row > board_size.rows {
            self.position.row = 0;
        }

        next_item
    }
}

/// Represents an individual square of the game board.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Square {
    /// The owner of the square.
    pub owner: Owner,

    /// The position the square is located at on the board.
    pub position: Position,
}


/// Represents the size of the board in number of rows and columns.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Size {
    /// The number of rows.
    pub rows: usize,

    /// The number of columns.
    pub columns: usize,
}

impl From<(usize, usize)> for Size {

    /// Creates a Size structure from the given tuple.
    #[inline]
    fn from(value: (usize, usize)) -> Size {
        Size {
            rows: value.0,
            columns: value.1,
        }
    }
}


/// Represents a specific board position denoted by row and column.
///
/// The row and column values are zero based indexed.
///
/// # Examples
/// A convenient way to create a position is from a tuple where the first element
/// is the row and the second element is the column.
/// ```
/// let p = open_ttt_lib::Position::from((2, 3));
/// assert_eq!(p.row, 2);
/// assert_eq!(p.column, 3);
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    /// The row associated with the position.
    pub row: usize,

    /// The column associated with the position.
    pub column: usize,
}

impl From<(usize, usize)> for Position {

    /// Creates a Position structure from the given tuple.
    #[inline]
    fn from(value: (usize, usize)) -> Position {
        Position {
            row: value.0,
            column: value.1,
        }
    }
}


/// Indicates which player owns a square, if any.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Owner {
    /// Player X owns the square.
    PlayerX,

    /// Player O owns the square.
    PlayerO,

    /// No player owns the square.
    None,
}


// This module contains the tests for the types in this file.
//
// The test naming format is:
//   <method>_when_<scenario_being_tested>_should_<expected_behavior>
// Also, try test exactly one item per test, e.g. one assert per test.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_new_when_given_1x1_size_should_create_1x1_board() {
        let expected_size = Size{ rows: 1, columns: 1, };

        let board = Board::new(expected_size);
        let actual_size = board.size();

        assert_eq!(expected_size, actual_size);
    }

    #[test]
    fn board_new_should_contain_squares_with_no_owner() {
        let size = Size{ rows: 1, columns: 1, };
        let position = Position {row: 0, column: 0 };
        let expected_owner = Owner::None;

        let board = Board::new(size);
        let actual_owner = board.get(position).unwrap().owner;

        assert_eq!(expected_owner, actual_owner);
    }

    #[test]
    fn board_new_when_given_3x3_size_should_create_3x3_board() {
        let expected_size = Size{ rows: 3, columns: 3, };

        let board = Board::new(expected_size);
        let actual_size = board.size();

        assert_eq!(expected_size, actual_size);
    }

    #[test]
    fn board_new_when_given_1x3_size_should_create_1x3_board() {
        let expected_size = Size{ rows: 1, columns: 3, };

        let board = Board::new(expected_size);
        let actual_size = board.size();

        assert_eq!(expected_size, actual_size);
    }

    #[test]
    #[should_panic]
    fn board_new_when_given_0x1_size_should_panic() {
        let invalid_size = Size {
            rows: 0,
            columns: 1,
        };

        let _board = Board::new(invalid_size);
    }

    #[test]
    #[should_panic]
    fn board_new_when_given_1x0_size_should_panic() {
        let invalid_size = Size {
            rows: 1,
            columns: 0,
        };

        let _board = Board::new(invalid_size);
    }

    #[test]
    fn board_contains_when_includes_position_should_be_true() {
        let board = Board::new(Size { rows: 1, columns: 1 });
        let position_in_board = Position { row: 0, column: 0 };

        let actual = board.contains(position_in_board);

        assert_eq!(true, actual);
    }

    #[test]
    fn board_contains_when_excludes_position_should_be_false() {
        let board = Board::new(Size { rows: 1, columns: 1 });
        let position_not_in_board = Position { row: 1, column: 0 };

        let actual = board.contains(position_not_in_board);

        assert_eq!(false, actual);
    }

    #[test]
    fn board_get_when_contains_position_should_be_some_square() {
        let board = Board::new(Size { rows: 1, columns: 1 });
        let position = Position { row: 0, column: 0 };
        // Note that new board squares start with no owner.
        let square = Square {owner: Owner::None, position, };
        let expected = Some(square);

        let actual = board.get(position);

        assert_eq!(expected, actual);
    }

    #[test]
    fn board_get_when_non_square_board_should_get_correct_position() {
        // Make a board that is non square. This ensures the correct row and
        // column are returned.
        let mut board = Board::new(Size { rows: 2, columns: 5 });
        let position = Position { row: 1, column: 3 };
        // Since squares start with no owner, mark the square at the expected
        // position to ensure we are getting the correct square.
        let square = Square {owner: Owner::PlayerX, position, };
        let expected = Some(square);

        board.set(square);
        let actual = board.get(position);

        assert_eq!(expected, actual);
    }

    #[test]
    fn board_get_when_not_contains_position_should_be_none() {
        let board = Board::new(Size { rows: 1, columns: 1 });
        let position_not_in_board = Position { row: 1, column: 0 };
        let expected = None;

        let actual = board.get(position_not_in_board);

        assert_eq!(expected, actual);
    }

    #[test]
    fn board_set_when_given_square_with_different_owner_should_change_square_owner() {
        let mut board = Board::new(Size { rows: 1, columns: 1 });
        let position = Position { row: 0, column: 0 };
        let expected = Square{ owner: Owner::PlayerX, position, };

        board.set(expected);
        let actual = board.get(position).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    #[should_panic]
    fn board_set_when_given_square_outside_board_should_panic() {
        let mut board = Board::new(Size { rows: 1, columns: 1 });
        let position_outside_board = Position { row: 1, column: 0 };
        let square_outside_board = Square{ owner: Owner::PlayerX, position: position_outside_board, };

        board.set(square_outside_board);
    }

    #[test]
    fn board_iter_should_include_all_squares() {
        // To see if this iter contains all the squares we count the number of
        // squares seen by the iter compared to the expected value.
        let rows = 1;
        let columns = 1;
        let board = Board::new(Size { rows, columns, });
        let expected = rows * columns;

        let actual = board.iter().count();

        assert_eq!(expected, actual);
    }

    #[allow(non_snake_case)]
    #[test]
    fn board_display_when_X_own_squares_should_contain_X_characters() {
        let mut board = Board::new(Size { rows: 1, columns: 1, });
        let position = Position { row: 0, column: 0 };
        let square = Square{ owner: Owner::PlayerX, position, };
        board.set(square);

        // Rust's to_string() method uses the display method.
        let textual_representation = board.to_string();

        assert!(textual_representation.contains("X"));
    }

    #[allow(non_snake_case)]
    #[test]
    fn board_display_when_O_own_squares_should_contain_O_characters() {
        let mut board = Board::new(Size { rows: 1, columns: 1, });
        let position = Position { row: 0, column: 0 };
        let square = Square{ owner: Owner::PlayerO, position, };
        board.set(square);

        // Rust's to_string() method uses the display method.
        let textual_representation = board.to_string();

        assert!(textual_representation.contains("O"));
    }


    #[test]
    fn square_when_same_owner_and_position_should_equal() {
        let owner = Owner::PlayerX;
        let position = Position::from((1, 2));
        let a = Square {
            owner,
            position,
        };
        let b = Square {
            owner,
            position,
        };

        assert_eq!(a, b);
    }

    #[test]
    fn square_when_different_owner_should_not_equal() {
        let position = Position::from((1, 2));
        let a = Square {
            owner: Owner::PlayerX,
            position,
        };
        let b = Square {
            owner: Owner::PlayerO,
            position,
        };

        assert_ne!(a, b);
    }

    #[test]
    fn square_when_different_position_should_not_equal() {
        let owner = Owner::PlayerX;
        let a = Square {
            owner,
            position: Position::from((0, 0)),
        };
        let b = Square {
            owner,
            position: Position::from((1, 2)),
        };

        assert_ne!(a, b);
    }

    #[test]
    fn square_when_copied_should_compare_equal() {
        let expected = Square {
            owner: Owner::PlayerX,
            position: Position::from((1, 2)),
        };

        // Perform a copy.
        let actual = expected;

        assert_eq!(expected, actual);
    }

    #[test]
    fn square_when_cloned_should_compare_equal() {
        let expected = Square {
            owner: Owner::PlayerX,
            position: Position::from((1, 2)),
        };

        let actual = expected.clone();

        assert_eq!(expected, actual);
    }


    #[test]
    fn size_when_same_should_compare_equal() {
        let rows = 0;
        let columns = 0;
        let a = Size {
            rows,
            columns,
        };
        let b = Size {
            rows,
            columns,
        };

        assert_eq!(a, b);
    }

    #[test]
    fn size_from_tuple_first_item_should_be_rows() {
        // A nonzero value is used so we test non default behavior.
        let expected_rows = 1;

        let actual = Size::from((expected_rows, 0));

        assert_eq!(expected_rows, actual.rows);
    }

    #[test]
    fn size_from_tuple_second_item_should_be_columns() {
        // A nonzero value is used so we test non default behavior.
        let expected_columns = 1;

        let actual = Size::from((0, expected_columns));

        assert_eq!(expected_columns, actual.columns);
    }

    #[test]
    fn size_when_copied_should_compare_equal() {
        let expected = Size {
            rows: 1,
            columns: 2,
        };

        // Perform a copy.
        let actual = expected;

        assert_eq!(expected, actual);
    }

    #[test]
    fn size_when_cloned_should_compare_equal() {
        let expected = Size {
            rows: 1,
            columns: 2,
        };

        let actual = expected.clone();

        assert_eq!(expected, actual);
    }


    #[test]
    fn position_when_same_should_compare_equal() {
        let row = 0;
        let column = 0;
        let a = Position {
            row,
            column,
        };
        let b = Position {
            row,
            column,
        };

        assert_eq!(a, b);
    }

    #[test]
    fn position_from_tuple_first_item_should_be_row() {
        // A nonzero value is used so we test non default behavior.
        let expected_row = 1;

        let actual = Position::from((expected_row, 0));

        assert_eq!(expected_row, actual.row);
    }

    #[test]
    fn position_from_tuple_second_item_should_be_column() {
        // A nonzero value is used so we test non default behavior.
        let expected_column = 1;

        let actual = Position::from((0, expected_column));

        assert_eq!(expected_column, actual.column);
    }

    #[test]
    fn position_when_copied_should_compare_equal() {
        let expected = Position {
            row: 1,
            column: 2,
        };

        // Perform a copy.
        let actual = expected;

        assert_eq!(expected, actual);
    }

    #[test]
    fn position_when_cloned_should_compare_equal() {
        let expected = Position {
            row: 1,
            column: 2,
        };

        let actual = expected.clone();

        assert_eq!(expected, actual);
    }
}
