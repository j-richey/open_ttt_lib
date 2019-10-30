use std::fmt;

/// Represents the Tic Tac Toe board providing multiple ways to access individual squares.
#[derive(Clone)]
pub struct Board {

}

impl Board {

    /// Constructs a new board of the given size.
    ///
    /// # Panics
    /// Panics if  either the number of rows or columns is less than one.
    pub fn new(size: Size) -> Board {
        panic!("This function is not implemented");
    }

    /// Gets the size of the board.
    pub fn size(&self) -> Size {
        panic!("This function is not implemented");
    }

    /// Returns `true` if the board contains the given position.
    pub fn contains(&self, position: Position) -> bool {
        panic!("This function is not implemented");
    }

    /// Gets the square at the indicated position.
    ///
    /// # Panics
    /// Panics if the position is outside the area of the board. Use 
    /// `contains()` to check if the position is valid for this board.
    pub fn get(&self, position: Position) -> Square {
        panic!("This function is not implemented");
    }

    /// Gets the owner of the square at the indicated position.
    ///
    /// # Panics
    /// Panics if the position is outside the area of the board. Use 
    /// `contains()` to check if the position is valid for this board.  
    pub fn owner(&self, position: Position) -> Owner {
        panic!("This function is not implemented");
    }

    /// Sets the owner of the square at the indicated position.
    ///
    /// # Panics
    /// Panics if the position is outside the area of the board. Use 
    /// `contains()` to check if the position is valid for this board.  
    pub fn set_owner(&mut self, position: Position, owner: Owner) {
        panic!("This function is not implemented");
    }

    /// Gets an iterator over all the squares in a `Board`.
    pub fn squares(&self) -> Squares {
        panic!("This function is not implemented");
    }
}

impl fmt::Display for Board {
    /// This provides simple formatted output of the board.
    ///
    /// This is suitable for use in simple console applications or debugging 
    /// purposes.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("This function is not implemented");
    }
}


/// An iterator over th squares in a `Board`.
pub struct Squares {
    // TODO: Figure out how to access the board here.

}

impl Iterator for Squares {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        panic!("This function is not implemented");
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
    pub rows: i32,

    /// The number of columns.
    pub columns: i32,
}

impl From<(i32, i32)> for Size {

    /// Creates a Size structure from the given tuple.
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
    pub row: i32,

    /// The column associated with the position.
    pub column: i32,
}

impl From<(i32, i32)> for Position {

    /// Creates a Position structure from the given tuple.
    #[inline]
    fn from(value: (i32, i32)) -> Position {
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
// The test naming format is: <method>_should_<expected>_when_<condition>.
// Also, try test exactly one item per test, e.g. one assert per test.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_new_should_create_3x3_board_when_given_3x3_size() {
        let expected_size = Size::from((3, 3));

        let board = Board::new(expected_size);
        let actual_size = board.size();

        assert_eq!(expected_size, actual_size);
    }

    #[test]
    #[should_panic]
    fn board_new_should_panic_when_given_0x3_size() {
        let invalid_size = Size::from((0, 3));
        let _board = Board::new(invalid_size);
    }

    #[test]
    #[should_panic]
    fn board_new_should_panic_when_given_3x0_size() {
        let invalid_size = Size::from((3, 0));

        let _board = Board::new(invalid_size);
    }

    #[test]
    fn square_are_equal() {
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
    fn square_different_owner_not_equal() {
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
    fn square_different_position_not_equal() {
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
    fn square_can_copy() {
        let expected = Square {
            owner: Owner::PlayerX,
            position: Position::from((1, 2)),
        };

        // Perform a copy.
        let actual = expected;

        assert_eq!(expected, actual);
    }

    #[test]
    fn position_are_equal() {
        let row = 1;
        let column = 2;
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
    fn position_from_tuple() {
        let row = 1;
        let column = 2;
        let expected = Position {
            row,
            column,
        };
        let t = (row, column);

        let actual = Position::from(t);

        assert_eq!(expected, actual);
    }

    #[test]
    fn position_can_copy() {
        let expected = Position {
            row: 1,
            column: 2,
        };

        // Perform a copy.
        let actual = expected;

        assert_eq!(expected, actual);
    }
}
