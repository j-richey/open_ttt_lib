//! Represents a game board providing multiple ways to access individual squares.
//!
//! Usually you do not need to create a board directly as the game creates the
//! board that it needs. However, the board is available so its representation can
//! be displayed in your application.

use std::fmt;

/// Represents the Tic Tac Toe board providing multiple ways to access individual squares.
#[derive(Clone)]
pub struct Board {
    squares: Vec<Owner>,
    size: Size,
}

impl Board {
    /// Constructs a new board of the given size.
    ///
    /// # Panics
    /// The minimum board size is 1x1. Panics if either the number of rows or
    /// columns is less than one.
    ///
    /// # Examples
    /// ```
    /// use open_ttt_lib::board;
    /// let size = board::Size {
    ///     rows: 3,
    ///     columns: 3,
    /// };
    /// let b = board::Board::new(size);
    ///
    /// assert_eq!(b.size(), size);
    /// ```
    pub fn new(size: Size) -> Board {
        const MIN_BOARD_SIZE: Size = Size {
            rows: 1,
            columns: 1,
        };

        // Validate the provided board size.
        if size.rows < MIN_BOARD_SIZE.rows || size.columns < MIN_BOARD_SIZE.columns {
            panic!(
                "Invalid board size of '{:?}' provided. The minimum board size is '{:?}'",
                size, MIN_BOARD_SIZE
            );
        }

        let total_squares = (size.rows * size.columns) as usize;
        let squares = vec![Owner::default(); total_squares];

        Board { squares, size }
    }

    /// Gets the size of the board.
    pub fn size(&self) -> Size {
        self.size
    }

    /// Returns `true` if the board contains the given position.
    ///
    /// **Note** positions are zero based.
    ///
    /// # Examples
    /// ```
    /// use open_ttt_lib::board;
    ///
    /// let b = board::Board::new(board::Size::from((3, 3)));
    ///
    /// assert!(b.contains(board::Position { row: 2, column: 2 }));
    /// // Since the positions are zero indexed, the board does not
    /// // contain the following position.
    /// assert!(!b.contains(board::Position { row: 3, column: 3 }));
    /// // A negative row or column is also not contained in the board.
    /// assert!(!b.contains(board::Position { row: -1, column: -1 }));
    /// ```
    pub fn contains(&self, position: Position) -> bool {
        let size = self.size();

        position.row >= 0
            && position.row < size.rows
            && position.column >= 0
            && position.column < size.columns
    }

    /// Returns a copy of the owner at the indicated position, or `None`
    /// if the board does not contain the provided position.
    ///
    /// # Examples
    /// ```
    /// use open_ttt_lib::board;
    ///
    /// let b = board::Board::new(board::Size::from((3, 3)));
    ///
    /// assert!(b.get(board::Position { row: 0, column: 0 }).is_some());
    /// assert!(b.get(board::Position { row: -1, column: -1 }).is_none());
    /// ```
    pub fn get(&self, position: Position) -> Option<Owner> {
        if let Some(index) = self.get_squares_index(position) {
            let owner = self.squares[index];
            Some(owner)
        } else {
            None
        }
    }

    /// Gets a mutable reference to the owner at the indicated position.
    ///
    /// This allows the owner of the position to be changed. `None` is returned
    /// if the board does not contain the provided position.
    ///
    /// # Examples
    /// ```
    /// use open_ttt_lib::board;
    ///
    /// let mut b = board::Board::new(board::Size::from((3, 3)));
    /// let position = board::Position { row: 2, column: 2 };
    ///
    /// // Change the owner of the position to Player X.
    /// if let Some(owner) = b.get_mut(position) {
    ///     *owner = board::Owner::PlayerX;
    /// }
    ///
    /// assert_eq!(b.get(position), Some(board::Owner::PlayerX));
    /// ```
    pub fn get_mut(&mut self, position: Position) -> Option<&mut Owner> {
        if let Some(index) = self.get_squares_index(position) {
            self.squares.get_mut(index)
        } else {
            None
        }
    }

    /// Gets an iterator over all the positions in the board.
    ///
    /// The iterator provides tuples containing the position and the owner of the
    /// position. The items are returned in arbitrary order.
    ///
    /// # Examples
    /// ```
    /// use open_ttt_lib::board;
    ///
    /// let b = board::Board::new(board::Size::from((3, 3)));
    ///
    /// // Print the owner of every position.
    /// for (position, owner) in b.iter() {
    ///     println!("{:?} is owned by {:?}", position, owner);
    /// }
    /// ```
    pub fn iter(&self) -> Iter {
        Iter {
            board: &self,
            position: Position { row: 0, column: 0 },
        }
    }

    // Helper function that calculates the index into the squares vector based on the given position.
    // None is returned if the board does not contain the given position.
    fn get_squares_index(&self, position: Position) -> Option<usize> {
        if !self.contains(position) {
            return None;
        }
        let index = (position.row * self.size.columns + position.column) as usize;
        assert!(index < self.squares.len());

        Some(index)
    }

    // Helper function for displaying boards that writes the separators between rows.
    fn write_row_separator(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for _ in 0..self.size().columns {
            write!(f, "+---")?;
        }
        writeln!(f, "+")
    }

    // Helper function for displaying boards that writes the content of the row.
    fn write_row_content(&self, f: &mut fmt::Formatter<'_>, row: &[Owner]) -> fmt::Result {
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
        for row in 0..self.size().rows {
            self.write_row_separator(f)?;
            let start = (row * self.size().columns) as usize;
            let end = start + self.size().columns as usize;
            self.write_row_content(f, &self.squares[start..end])?
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
    type Item = (Position, Owner);

    fn next(&mut self) -> Option<Self::Item> {
        // Get the owner at the current position.
        let next_value = match self.board.get(self.position) {
            Some(owner) => Some((self.position, owner)),
            None => None,
        };

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

        next_value
    }
}

/// Represents the size of the board in number of rows and columns.
///
/// # Examples
/// ```
/// use open_ttt_lib::board;
///
/// let size = board::Size { rows: 3, columns: 1 };
///
/// // Sizes can also be constructed from tuples:
/// let size_from_tuple = board::Size::from((3, 1));
///
/// assert_eq!(size, size_from_tuple);
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Size {
    /// The number of rows.
    pub rows: i32,

    /// The number of columns.
    pub columns: i32,
}

impl From<(i32, i32)> for Size {
    /// Creates a Size structure from the given tuple.
    ///
    /// # Examples
    /// ```
    /// use open_ttt_lib::board;
    ///
    /// let size = board::Size::from((3, 1));
    /// assert_eq!(size.rows, 3);
    /// assert_eq!(size.columns, 1);
    /// ```
    #[inline]
    fn from(value: (i32, i32)) -> Size {
        Size {
            rows: value.0,
            columns: value.1,
        }
    }
}

/// Represents a specific board position denoted by row and column.
///
/// The row and column values are zero based indexed. However, negative values
/// can be assigned; often this is convenient when positions are calculated from
/// mathematical algorithms.
///
/// # Examples
/// ```
/// use open_ttt_lib::board;
///
/// let p = board::Position { row: 2, column: 3 };
///
/// // Positions can also be constructed from tuples.
/// let p2 = board::Position::from((2, 3));
/// assert_eq!(p, p2);
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    /// The row associated with the position.
    pub row: i32,

    /// The column associated with the position.
    pub column: i32,
}

impl From<(i32, i32)> for Position {
    /// Creates a Position structure from the given tuple.
    ///
    /// # Examples
    /// ```
    /// use open_ttt_lib::board;
    ///
    /// let p = board::Position::from((2, 3));
    /// assert_eq!(p.row, 2);
    /// assert_eq!(p.column, 3);
    /// ```
    #[inline]
    fn from(value: (i32, i32)) -> Position {
        Position {
            row: value.0,
            column: value.1,
        }
    }
}

/// Indicates which player owns a position, if any.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Owner {
    /// Player X owns the position.
    PlayerX,

    /// Player O owns the position.
    PlayerO,

    /// No player owns the position.
    None,
}

impl Default for Owner {
    fn default() -> Self {
        Owner::None
    }
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
        let expected_size = Size {
            rows: 1,
            columns: 1,
        };

        let board = Board::new(expected_size);
        let actual_size = board.size();

        assert_eq!(expected_size, actual_size);
    }

    #[test]
    fn board_new_should_contain_squares_with_no_owner() {
        let size = Size {
            rows: 1,
            columns: 1,
        };
        let position = Position { row: 0, column: 0 };
        let expected_owner = Owner::None;

        let board = Board::new(size);
        let actual_owner = board.get(position).unwrap();

        assert_eq!(expected_owner, actual_owner);
    }

    #[test]
    fn board_new_when_given_3x3_size_should_create_3x3_board() {
        let expected_size = Size {
            rows: 3,
            columns: 3,
        };

        let board = Board::new(expected_size);
        let actual_size = board.size();

        assert_eq!(expected_size, actual_size);
    }

    #[test]
    fn board_new_when_given_1x3_size_should_create_1x3_board() {
        let expected_size = Size {
            rows: 1,
            columns: 3,
        };

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
        let board = Board::new(Size {
            rows: 1,
            columns: 1,
        });
        let position_in_board = Position { row: 0, column: 0 };

        let actual = board.contains(position_in_board);

        assert_eq!(true, actual);
    }

    #[test]
    fn board_contains_when_row_outside_board_should_be_false() {
        let board = Board::new(Size {
            rows: 1,
            columns: 1,
        });
        let position_not_in_board = Position { row: 1, column: 0 };

        let actual = board.contains(position_not_in_board);

        assert_eq!(false, actual);
    }

    #[test]
    fn board_contains_when_column_outside_board_should_be_false() {
        let board = Board::new(Size {
            rows: 1,
            columns: 1,
        });
        let position_not_in_board = Position { row: 0, column: 1 };

        let actual = board.contains(position_not_in_board);

        assert_eq!(false, actual);
    }

    #[test]
    fn board_contains_when_negative_row_should_be_false() {
        let board = Board::new(Size {
            rows: 1,
            columns: 1,
        });
        let position_not_in_board = Position { row: -1, column: 0 };

        let actual = board.contains(position_not_in_board);

        assert_eq!(false, actual);
    }

    #[test]
    fn board_contains_when_negative_column_should_be_false() {
        let board = Board::new(Size {
            rows: 1,
            columns: 1,
        });
        let position_not_in_board = Position { row: 0, column: -1 };

        let actual = board.contains(position_not_in_board);

        assert_eq!(false, actual);
    }

    #[test]
    fn board_get_when_contains_position_should_be_some_owner() {
        let board = Board::new(Size {
            rows: 1,
            columns: 1,
        });
        let position = Position { row: 0, column: 0 };
        // Note that for new boards positions start with no owner.
        let expected = Some(Owner::None);

        let actual = board.get(position);

        assert_eq!(expected, actual);
    }

    #[test]
    fn board_get_when_not_contains_position_should_be_none() {
        // Simulate a parameterized test by wrapping in a for loop. Positions are picked to exercise
        // various edge cases such as wrapping to the next row.
        let positions_not_in_board = vec![
            Position {
                row: -1,
                column: -1,
            },
            Position { row: 0, column: -1 },
            Position { row: -1, column: 0 },
            Position { row: 0, column: 4 },
            Position { row: 1, column: 4 },
            Position { row: 2, column: 0 },
        ];
        for position in positions_not_in_board {
            let board = Board::new(Size {
                rows: 2,
                columns: 4,
            });
            let expected = None;

            let actual = board.get(position);
            assert_eq!(
                expected,
                actual,
                "Position {:?} found in board of size {:?}",
                position,
                board.size()
            );
        }
    }

    #[test]
    fn board_get_mut_when_given_new_owner_should_change_owner() {
        let mut board = Board::new(Size {
            rows: 1,
            columns: 1,
        });
        let position = Position { row: 0, column: 0 };
        let expected_owner = Owner::PlayerX;

        *board.get_mut(position).unwrap() = expected_owner;
        let actual_owner = board.get(position).unwrap();

        assert_eq!(expected_owner, actual_owner);
    }

    #[test]
    fn board_get_mut_when_given_position_outside_board_should_return_none() {
        let mut board = Board::new(Size {
            rows: 1,
            columns: 1,
        });
        let position_outside_board = Position { row: 1, column: 0 };
        let expected = None;

        let actual = board.get_mut(position_outside_board);

        assert_eq!(expected, actual);
    }

    #[test]
    fn board_iter_should_include_all_positions() {
        // To see if this iter contains all the positions we count the number of
        // items seen by the iter compared to the expected value.
        let rows = 1;
        let columns = 1;
        let board = Board::new(Size { rows, columns });
        let expected = rows * columns;

        let actual = board.iter().count() as i32;

        assert_eq!(expected, actual);
    }

    #[test]
    fn board_iter_should_provide_position_and_owner() {
        let board = Board::new(Size {
            rows: 1,
            columns: 1,
        });
        let expected = (Position { row: 0, column: 0 }, Owner::None);

        let actual = board.iter().next().unwrap();

        assert_eq!(expected, actual);
    }

    #[allow(non_snake_case)]
    #[test]
    fn board_display_when_X_own_squares_should_contain_X_characters() {
        let mut board = Board::new(Size {
            rows: 1,
            columns: 1,
        });
        let position = Position { row: 0, column: 0 };
        *board.get_mut(position).unwrap() = Owner::PlayerX;

        // Rust's to_string() method uses the display method.
        let textual_representation = board.to_string();

        assert!(textual_representation.contains('X'));
    }

    #[allow(non_snake_case)]
    #[test]
    fn board_display_when_O_own_squares_should_contain_O_characters() {
        let mut board = Board::new(Size {
            rows: 1,
            columns: 1,
        });
        let position = Position { row: 0, column: 0 };
        *board.get_mut(position).unwrap() = Owner::PlayerO;

        // Rust's to_string() method uses the display method.
        let textual_representation = board.to_string();

        assert!(textual_representation.contains('O'));
    }

    #[test]
    fn size_when_same_should_compare_equal() {
        let rows = 0;
        let columns = 0;
        let a = Size { rows, columns };
        let b = Size { rows, columns };

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
        let a = Position { row, column };
        let b = Position { row, column };

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
        let expected = Position { row: 1, column: 2 };

        // Perform a copy.
        let actual = expected;

        assert_eq!(expected, actual);
    }

    #[test]
    fn position_when_cloned_should_compare_equal() {
        let expected = Position { row: 1, column: 2 };

        let actual = expected.clone();

        assert_eq!(expected, actual);
    }
}
