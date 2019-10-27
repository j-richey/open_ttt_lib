
/// Represents a specific board position denoted by row and column.
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_are_equal() {
        let a = Position {
            row: 1,
            column: 2,
        };
        let b = Position {
            row: 1,
            column: 2,
        };

        assert_eq!(a, b);
    }

    #[test]
    fn position_from_tuple() {
        let expected = Position {
            row: 1,
            column: 2,
        };
        let t = (1, 2);

        let actual = Position::from(t);

        assert_eq!(expected, actual);
    }

    #[test]
    fn position_can_copy() {
        let expected = Position {
            row: 1,
            column: 2,
        };

        // Perform  a copy.
        let actual = expected;

        assert_eq!(expected, actual);
    }
}
