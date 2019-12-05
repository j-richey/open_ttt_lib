use std::collections::HashSet;

use crate::game;

pub struct AIOpponent {}

impl AIOpponent {
    pub fn new(mistake_probability: f64) -> Self {
        unimplemented!();
    }

    pub fn get_move(self, game: &game::Game) -> Option<game::Position> {
        unimplemented!();
    }

    fn evaluate_position(
        self,
        game: &game::Game,
        position: game::Position,
        ai_player: AIPlayer,
    ) -> AIOutcome {
        unimplemented!();
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum AIPlayer {
    PlayerX,
    PlayerO,
}

impl AIPlayer {
    // Determines which player the AI is playing as, X or O, based on the current
    // state of the game.
    //
    // Panics if the game is over.
    fn from_game_state(state: game::State) -> Self {
        match state {
            game::State::PlayerXMove => Self::PlayerX,
            game::State::PlayerOMove => Self::PlayerO,
            _ => panic!(
                "Cannot determine the AI player since the game is over. \
                 This condition is the result of a bug in the \
                 open_ttt_lib used by this application."
            ),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum AIOutcome {
    Win,
    Loss,
    CatsGame,
    Unknown,
}

impl AIOutcome {
    // Determines the outcome of the game or the AI opponent based on the
    // provided game state.
    //
    // Panics if the game is not over.
    fn from_game_state(state: game::State, ai_player: AIPlayer) -> Self {
        match state {
            game::State::CatsGame => AIOutcome::CatsGame,
            game::State::PlayerXWin(_) => match ai_player {
                AIPlayer::PlayerX => AIOutcome::Win,
                AIPlayer::PlayerO => AIOutcome::Loss,
            },
            game::State::PlayerOWin(_) => match ai_player {
                AIPlayer::PlayerX => AIOutcome::Loss,
                AIPlayer::PlayerO => AIOutcome::Win,
            },
            _ => panic!(
                "Cannot determine the AI outcome since the game is not over. \
                 This condition is the result of a bug in the \
                 open_ttt_lib used by this application."
            ),
        }
    }
}

// Gets the worst possible outcome based on the provided outcomes.
//
// The ordering of outcomes returned are: `Loss`, `CatsGame`, `Win`.
// `Unknown` is returned if the provided slice is empty or only contains unknown
// outcomes.
fn worst_outcome(outcomes: &HashSet<AIOutcome>) -> AIOutcome {
    // The set is checked for the outcomes from worst to best, finally returning
    // unknown if it is empty or no other information is known.
    if outcomes.contains(&AIOutcome::Loss) {
        AIOutcome::Loss
    } else if outcomes.contains(&AIOutcome::CatsGame) {
        AIOutcome::CatsGame
    } else if outcomes.contains(&AIOutcome::Win) {
        AIOutcome::Win
    } else {
        AIOutcome::Unknown
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ai_player_from_game_state_when_player_X_move_should_be_player_X() {
        let game_state = game::State::PlayerXMove;
        let expected_ai_player = AIPlayer::PlayerX;

        let actual_ai_player = AIPlayer::from_game_state(game_state);

        assert_eq!(expected_ai_player, actual_ai_player);
    }

    #[test]
    fn ai_player_from_game_state_when_player_O_move_should_be_player_O() {
        let game_state = game::State::PlayerOMove;
        let expected_ai_player = AIPlayer::PlayerO;

        let actual_ai_player = AIPlayer::from_game_state(game_state);

        assert_eq!(expected_ai_player, actual_ai_player);
    }

    #[test]
    #[should_panic]
    fn ai_player_from_game_state_when_game_over_should_panic() {
        // Set the game state to a game over state.
        let game_state = game::State::CatsGame;

        let _actual_ai_player = AIPlayer::from_game_state(game_state);
    }

    #[test]
    fn ai_outcome_from_game_state_when_cats_game_should_be_cats_game() {
        let game_state = game::State::CatsGame;
        let ai_player = AIPlayer::PlayerX;
        let expected_ai_outcome = AIOutcome::CatsGame;

        let actual_ai_outcome = AIOutcome::from_game_state(game_state, ai_player);

        assert_eq!(expected_ai_outcome, actual_ai_outcome);
    }

    #[test]
    fn ai_outcome_from_game_state_when_player_X_win_and_player_X_should_be_win() {
        let game_state = game::State::PlayerXWin(Default::default());
        let ai_player = AIPlayer::PlayerX;
        let expected_ai_outcome = AIOutcome::Win;

        let actual_ai_outcome = AIOutcome::from_game_state(game_state, ai_player);

        assert_eq!(expected_ai_outcome, actual_ai_outcome);
    }

    #[test]
    fn ai_outcome_from_game_state_when_player_X_win_and_player_O_should_be_loss() {
        let game_state = game::State::PlayerXWin(Default::default());
        let ai_player = AIPlayer::PlayerO;
        let expected_ai_outcome = AIOutcome::Loss;

        let actual_ai_outcome = AIOutcome::from_game_state(game_state, ai_player);

        assert_eq!(expected_ai_outcome, actual_ai_outcome);
    }

    #[test]
    fn ai_outcome_from_game_state_when_player_O_win_and_player_O_should_be_win() {
        let game_state = game::State::PlayerOWin(Default::default());
        let ai_player = AIPlayer::PlayerO;
        let expected_ai_outcome = AIOutcome::Win;

        let actual_ai_outcome = AIOutcome::from_game_state(game_state, ai_player);

        assert_eq!(expected_ai_outcome, actual_ai_outcome);
    }

    #[test]
    fn ai_outcome_from_game_state_when_player_O_win_and_player_X_should_be_loss() {
        let game_state = game::State::PlayerOWin(Default::default());
        let ai_player = AIPlayer::PlayerX;
        let expected_ai_outcome = AIOutcome::Loss;

        let actual_ai_outcome = AIOutcome::from_game_state(game_state, ai_player);

        assert_eq!(expected_ai_outcome, actual_ai_outcome);
    }

    #[test]
    fn worst_outcome_when_empty_should_be_unknown() {
        let outcomes = Default::default();
        let expected_outcome = AIOutcome::Unknown;

        let actual_outcome = worst_outcome(&outcomes);

        assert_eq!(expected_outcome, actual_outcome);
    }

    #[test]
    fn worst_outcome_when_win_and_loss_should_be_loss() {
        let outcomes = [AIOutcome::Win, AIOutcome::Loss].iter().cloned().collect();
        let expected_outcome = AIOutcome::Loss;

        let actual_outcome = worst_outcome(&outcomes);

        assert_eq!(expected_outcome, actual_outcome);
    }

    #[test]
    fn worst_outcome_when_cats_game_and_loss_should_be_loss() {
        let outcomes = [AIOutcome::CatsGame, AIOutcome::Loss]
            .iter()
            .cloned()
            .collect();
        let expected_outcome = AIOutcome::Loss;

        let actual_outcome = worst_outcome(&outcomes);

        assert_eq!(expected_outcome, actual_outcome);
    }

    #[test]
    fn worst_outcome_when_cats_game_and_cats_game_should_be_cats_game() {
        let outcomes = [AIOutcome::Win, AIOutcome::CatsGame]
            .iter()
            .cloned()
            .collect();
        let expected_outcome = AIOutcome::CatsGame;

        let actual_outcome = worst_outcome(&outcomes);

        assert_eq!(expected_outcome, actual_outcome);
    }
}
