

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
