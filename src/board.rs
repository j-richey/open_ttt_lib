
/// Represents an individual square of the game board.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Square {
    /// The owner of the square.
    pub owner: Owner,

    /// The position the square is located at on the board.
    pub position: Position,
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


#[cfg(test)]
mod tests {
    use super::*;

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
