//! Provides functionality for creating single player games.
//!
//! # Examples
//! ```
//! use open_ttt_lib::ai;
//! use open_ttt_lib::game;
//!
//! let game = game::Game::new();
//! let ai_opponent = ai::Opponent::new(0.0);
//!
//! match ai_opponent.get_move(&game) {
//!     Some(position) => assert!(game.can_move(position)),
//!     None => panic!("The game is over so the AI opponent cannot do a move."),
//! };
//! ```

use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::BuildHasher;

use crate::game;

/// Provides a computer controlled AI opponent.
///
/// This can be used to create single player games or implement a hint system
/// for human users.
pub struct Opponent {
    mistake_probability: f64,
}

impl Opponent {
    /// Constructs a new AI opponent.
    ///
    /// The mistake probability indicates how likely the AI will fail to consider
    /// various situations. A value of 0.0 makes the AI play a perfect game.
    /// A value of 1.0 causes the AI to always pick a random position. Values
    /// less than 0.0 are set to 0.0 and values greater than 1.0 are set to 1.0.
    ///
    /// # Examples
    ///
    /// Construct an unbeatable AI opponent:
    /// ```
    /// use open_ttt_lib::ai;
    ///
    /// let mistake_probability = 0.0;
    /// let unbeatable_opponent = ai::Opponent::new(mistake_probability);
    /// ```
    ///
    /// Construct an AI opponent that randomly picks a position:
    /// ```
    /// use open_ttt_lib::ai;
    ///
    /// let mistake_probability = 1.0;
    /// let rando = ai::Opponent::new(mistake_probability);
    /// ```
    pub fn new(mistake_probability: f64) -> Self {
        Self {
            mistake_probability,
        }
    }

    /// Gets the position the AI opponent wishes to move based on the provided game.
    ///
    /// `None` is returned if the game is over. The AI opponent never tries to
    /// select an invalid position, that is a position that is not free.
    ///
    /// # Examples
    /// ```
    /// use open_ttt_lib::ai;
    /// use open_ttt_lib::game;
    ///
    /// let game = game::Game::new();
    /// let ai_opponent = ai::Opponent::new(0.0);
    ///
    /// match ai_opponent.get_move(&game) {
    ///     Some(position) => assert!(game.can_move(position)),
    ///     None => panic!("The game is over so the AI opponent cannot do a move."),
    /// };
    /// ```
    pub fn get_move(&self, game: &game::Game) -> Option<game::Position> {
        // Return the best position based evaluating the game.
        let outcomes = self.evaluate_game(game);
        best_position(&outcomes)
    }

    /// Evaluates each free position in the provided game.
    ///
    /// Each free position in the game is mapped to an outcome for the AI opponent.
    /// If the game is over an empty map is returned.
    ///
    /// This functionality is useful if you wish to examine how the AI opponent
    /// viewed the game. E.g. this can be helpful for creating a hint system to
    /// help human players pick a position or when fine tuning the AI difficulty
    /// settings.
    ///
    /// # Examples
    /// ```
    /// use open_ttt_lib::ai;
    /// use open_ttt_lib::game;
    ///
    /// let game = game::Game::new();
    /// let ai_opponent = ai::Opponent::new(0.0);
    ///
    /// let outcomes = ai_opponent.evaluate_game(&game);
    ///
    /// // Display the outcome for each position.
    /// for (position, outcome) in outcomes {
    ///     assert!(game.can_move(position));
    ///     println!("position: {:?} outcome: {:?}", position, outcome);
    /// }
    /// ```
    pub fn evaluate_game(&self, game: &game::Game) -> HashMap<game::Position, Outcome> {
        let mut outcomes = HashMap::new();

        // We only evaluate the game if it is not over as if the game is over we
        // cannot determine which player the AI is playing as.
        if !game.state().is_game_over() {
            // Determine which player the AI is playing as.
            let ai_player = AIPlayer::from_game_state(game.state());

            // For each free square, evaluate the consequences of using that
            // square. The outcome for each position and the position is recorded.
            for position in game.free_positions() {
                let outcome = self.evaluate_position(&game, position, ai_player);
                outcomes.insert(position, outcome);
            }
        }

        outcomes
    }

    // Evaluates what outcome of the game would be by selecting a specific position.
    //
    // **Note** this is a recursive function.
    fn evaluate_position(
        &self,
        game: &game::Game,
        position: game::Position,
        ai_player: AIPlayer,
    ) -> Outcome {
        debug_assert!(
            game.can_move(position),
            "Cannot move into the provided position, {:?}. Thus, the position \
             cannot be evaluated. Ensure the game is not over and the position \
             is free. This condition is the result of a bug in the open_ttt_lib \
             used by this application.",
            position
        );

        // Check to see if the AI should make a mistake. If so, don't consider
        // this position.
        if self.should_make_mistake() {
            return Outcome::Unknown;
        }

        // Clone the game so we can try out the move without modifying the original game.
        let mut game = game.clone();
        let state = game.do_move(position).unwrap();

        // Check to see if the game is over. If so, return the outcome of the
        // game from the AI's perspective, e.g. win, loss, or cat's game.
        if state.is_game_over() {
            return Outcome::from_game_state(state, ai_player);
        }

        // The game is not over, to evaluate each of the remaining free squares.
        // Note: the game automatically takes care of switching between each
        // player's turn.
        let mut outcomes = HashSet::new();
        for free_position in game.free_positions() {
            let outcome = self.evaluate_position(&game, free_position, ai_player);
            outcomes.insert(outcome);
        }

        // The AI assumes the other player plays a perfect game, so return the
        // worst outcome that was found.
        worst_outcome(&outcomes)
    }

    // Indicates if the AI opponent should make a mistake by skipping examining
    // part of the tree.
    fn should_make_mistake(&self) -> bool {
        // Use a random number generator to get a boolean per the mistake probability.
        rand::thread_rng().gen_bool(self.mistake_probability)
    }
}

/// Represents a game outcome for the AI opponent.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Outcome {
    /// The AI player wins the game.
    Win,

    /// The AI player looses the game.
    Loss,

    /// The game results in a cats game.
    CatsGame,

    /// The outcome of the game is unknown to the AI player.
    Unknown,
}

impl Outcome {
    // Determines the outcome of the game or the AI opponent based on the
    // provided game state.
    //
    // Panics if the game is not over.
    fn from_game_state(state: game::State, ai_player: AIPlayer) -> Self {
        match state {
            game::State::CatsGame => Outcome::CatsGame,
            game::State::PlayerXWin(_) => match ai_player {
                AIPlayer::PlayerX => Outcome::Win,
                AIPlayer::PlayerO => Outcome::Loss,
            },
            game::State::PlayerOWin(_) => match ai_player {
                AIPlayer::PlayerX => Outcome::Loss,
                AIPlayer::PlayerO => Outcome::Win,
            },
            _ => panic!(
                "Cannot determine the AI outcome since the game is not over. \
                 This condition is the result of a bug in the \
                 open_ttt_lib used by this application."
            ),
        }
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

/// Picks a position with the best outcome based on the provided mapping of
/// positions to outcomes.
///
/// The ordering of outcomes from best to worst are: `Win`, `CatsGame`,
/// `Unknown`, `Loss`. A cats game is considered better than unknown as the
/// AI would rather have the game end in a draw than risk a loss. If there
/// are multiple positions with the same outcome, one of the positions is
/// picked at random.
///
/// # Examples
/// ```
/// use open_ttt_lib::ai;
/// use open_ttt_lib::game;
///
/// let game = game::Game::new();
/// let ai_opponent = ai::Opponent::new(0.0);
///
/// let outcomes = ai_opponent.evaluate_game(&game);
///
/// // Get the best position to use based on the outcomes.
/// if let Some(position) = ai::best_position(&outcomes) {
///     assert!(game.can_move(position));
/// }
/// ```
pub fn best_position<S: BuildHasher>(
    outcomes: &HashMap<game::Position, Outcome, S>,
) -> Option<game::Position> {
    // Build a mapping from outcomes to positions so one of the positions with
    // the best outcome can be selected.
    let mut outcome_to_position_map = HashMap::new();
    for (position, outcome) in outcomes {
        if !outcome_to_position_map.contains_key(outcome) {
            outcome_to_position_map.insert(outcome, Vec::new());
        }
        outcome_to_position_map
            .get_mut(outcome)
            .unwrap()
            .push(position);
    }

    // Iterate over the outcomes from best to worst returning a position with the
    // best outcome. If there are multiple positions, a random one is selected.
    let best_to_worst_outcomes = [
        Outcome::Win,
        Outcome::CatsGame,
        Outcome::Unknown,
        Outcome::Loss,
    ];
    for outcome in best_to_worst_outcomes.iter() {
        if outcome_to_position_map.contains_key(outcome) {
            let random_position = **outcome_to_position_map
                .get(outcome)
                .unwrap()
                .choose(&mut rand::thread_rng())
                .unwrap();

            return Some(random_position);
        }
    }

    // No suitable positions were found, so return None.
    None
}

// Gets the worst possible outcome based on the provided outcomes.
//
// The ordering of outcomes returned are: `Loss`, `CatsGame`, `Win`.
// `Unknown` is returned if the provided slice is empty or only contains unknown
// outcomes.
fn worst_outcome(outcomes: &HashSet<Outcome>) -> Outcome {
    // Search through the outcomes, from worst to best, returning the first one found.
    let worst_to_best_outcomes = [Outcome::Loss, Outcome::CatsGame, Outcome::Win];
    for outcome in worst_to_best_outcomes.iter() {
        if outcomes.contains(outcome) {
            return *outcome;
        }
    }

    // None of the other outcomes were found so return unknown.
    Outcome::Unknown
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;

    // Create several game boards for use with the unit tests. An asterisk (*)
    // marks the last position placed.

    //  +---+---+---+
    //  | X | O | X |
    //  +---+---+---+
    //  |   | O |   |
    //  +---+---+---+
    //  | X |   | O*|
    //  +---+---+---+
    const PLAYER_X_MOVE_WITH_WIN_AVAILABLE: [game::Position; 6] = [
        game::Position { row: 0, column: 0 },
        game::Position { row: 0, column: 1 },
        game::Position { row: 0, column: 2 },
        game::Position { row: 1, column: 1 },
        game::Position { row: 2, column: 0 },
        game::Position { row: 2, column: 2 },
    ];

    //  +---+---+---+
    //  | X | O | X |
    //  +---+---+---+
    //  | X*| O |   |
    //  +---+---+---+
    //  | X |   | O |
    //  +---+---+---+
    const PLAYER_X_WIN: [game::Position; 7] = [
        game::Position { row: 0, column: 0 },
        game::Position { row: 0, column: 1 },
        game::Position { row: 0, column: 2 },
        game::Position { row: 1, column: 1 },
        game::Position { row: 2, column: 0 },
        game::Position { row: 2, column: 2 },
        game::Position { row: 1, column: 0 },
    ];

    // Helper function that creates a game where the provided positions are
    // owned. The positions are marked in the order contained in the slice.
    //
    // # Panics
    // Panics if the game's do move method returns an error.
    fn create_game(owned_positions: &[game::Position]) -> game::Game {
        let mut game = game::Game::new();
        for position in owned_positions {
            game.do_move(*position).unwrap();
        }

        game
    }

    #[test]
    fn opponent_get_move_when_game_is_over_should_be_none() {
        // Create a game where the game is over.
        let game = create_game(&PLAYER_X_WIN);
        let opponent = Opponent::new(0.0);
        let expected_position = None;

        let actual_position = opponent.get_move(&game);

        assert_eq!(
            expected_position,
            actual_position,
            "\nGame board used for this test: \n{}",
            game.board()
        );
    }

    #[test]
    fn opponent_get_move_when_zero_mistake_probability_should_pick_wining_position() {
        // Create a game where the AI player has a wining move available.
        // The flawless AI should pick this position.
        let game = create_game(&PLAYER_X_MOVE_WITH_WIN_AVAILABLE);
        let opponent = Opponent::new(0.0);
        let expected_position = game::Position { row: 1, column: 0 };

        let actual_position = opponent.get_move(&game).unwrap();

        assert_eq!(
            expected_position,
            actual_position,
            "\nGame board used for this test: \n{}",
            game.board()
        );
    }

    #[test]
    fn opponent_evaluate_game_when_game_over_should_be_empty_map() {
        let game = create_game(&PLAYER_X_WIN);
        let mistake_probability = 0.0;
        let opponent = Opponent::new(mistake_probability);
        let expected_outcomes = HashMap::new();

        let actual_outcomes = opponent.evaluate_game(&game);

        assert_eq!(
            expected_outcomes,
            actual_outcomes,
            "\nGame board used for this test: \n{}",
            game.board()
        );
    }

    #[test]
    fn opponent_evaluate_game_when_zero_mistake_probability_should_evaluate_all_positions() {
        // Create a game where the AI player has a wining move available.
        // The flawless AI should determine the outcome of all remaining positions.
        let game = create_game(&PLAYER_X_MOVE_WITH_WIN_AVAILABLE);
        let mistake_probability = 0.0;
        let opponent = Opponent::new(mistake_probability);
        let mut expected_outcomes = HashMap::new();
        expected_outcomes.insert(game::Position { row: 1, column: 0 }, Outcome::Win);
        expected_outcomes.insert(game::Position { row: 1, column: 2 }, Outcome::Loss);
        expected_outcomes.insert(game::Position { row: 2, column: 1 }, Outcome::CatsGame);

        let actual_outcomes = opponent.evaluate_game(&game);

        assert_eq!(
            expected_outcomes,
            actual_outcomes,
            "\nGame board used for this test: \n{}",
            game.board()
        );
    }

    #[test]
    fn opponent_evaluate_game_when_one_mistake_probability_should_see_unknown_outcome_for_all_positions(
    ) {
        // Create a game where the AI player has a wining move available.
        // The AI that always makes mistakes should see the outcome as unknown for all positions.
        let game = create_game(&PLAYER_X_MOVE_WITH_WIN_AVAILABLE);
        let mistake_probability = 1.0;
        let opponent = Opponent::new(mistake_probability);
        let mut expected_outcomes = HashMap::new();
        expected_outcomes.insert(game::Position { row: 1, column: 0 }, Outcome::Unknown);
        expected_outcomes.insert(game::Position { row: 1, column: 2 }, Outcome::Unknown);
        expected_outcomes.insert(game::Position { row: 2, column: 1 }, Outcome::Unknown);

        let actual_outcomes = opponent.evaluate_game(&game);

        assert_eq!(
            expected_outcomes,
            actual_outcomes,
            "\nGame board used for this test: \n{}",
            game.board()
        );
    }

    #[test]
    fn opponent_best_position_when_outcomes_empty_should_none() {
        let outcomes = HashMap::new();
        let expected_position = None;

        let actual_position = best_position(&outcomes);

        assert_eq!(expected_position, actual_position);
    }

    #[test]
    fn opponent_best_position_when_win_and_cats_game_should_be_win() {
        let mut outcomes = HashMap::new();
        outcomes.insert(game::Position { row: 0, column: 0 }, Outcome::CatsGame);
        let expected_position = game::Position { row: 0, column: 1 };
        outcomes.insert(expected_position, Outcome::Win);

        let actual_position = best_position(&outcomes);

        assert_eq!(Some(expected_position), actual_position);
    }

    #[test]
    fn opponent_best_position_when_win_and_unknown_should_be_win() {
        let mut outcomes = HashMap::new();
        outcomes.insert(game::Position { row: 0, column: 0 }, Outcome::Unknown);
        let expected_position = game::Position { row: 0, column: 1 };
        outcomes.insert(expected_position, Outcome::Win);

        let actual_position = best_position(&outcomes);

        assert_eq!(Some(expected_position), actual_position);
    }

    #[test]
    fn opponent_best_position_when_win_and_loss_should_be_win() {
        let mut outcomes = HashMap::new();
        outcomes.insert(game::Position { row: 0, column: 0 }, Outcome::Loss);
        let expected_position = game::Position { row: 0, column: 1 };
        outcomes.insert(expected_position, Outcome::Win);

        let actual_position = best_position(&outcomes);

        assert_eq!(Some(expected_position), actual_position);
    }

    #[test]
    fn opponent_best_position_when_cats_game_and_loss_should_be_cats_game() {
        let mut outcomes = HashMap::new();
        outcomes.insert(game::Position { row: 0, column: 0 }, Outcome::Loss);
        let expected_position = game::Position { row: 0, column: 1 };
        outcomes.insert(expected_position, Outcome::CatsGame);

        let actual_position = best_position(&outcomes);

        assert_eq!(Some(expected_position), actual_position);
    }

    #[test]
    fn opponent_best_position_when_cats_game_and_unknown_should_be_cats_game() {
        let mut outcomes = HashMap::new();
        outcomes.insert(game::Position { row: 0, column: 0 }, Outcome::Unknown);
        let expected_position = game::Position { row: 0, column: 1 };
        outcomes.insert(expected_position, Outcome::CatsGame);

        let actual_position = best_position(&outcomes);

        assert_eq!(Some(expected_position), actual_position);
    }

    #[test]
    fn opponent_best_position_when_unknown_and_loss_should_be_unknown() {
        let mut outcomes = HashMap::new();
        outcomes.insert(game::Position { row: 0, column: 0 }, Outcome::Loss);
        let expected_position = game::Position { row: 0, column: 1 };
        outcomes.insert(expected_position, Outcome::Unknown);

        let actual_position = best_position(&outcomes);

        assert_eq!(Some(expected_position), actual_position);
    }

    #[test]
    fn opponent_best_position_when_same_outcome_should_pick_random_position() {
        let mut outcomes = HashMap::new();
        outcomes.insert(game::Position { row: 0, column: 0 }, Outcome::CatsGame);
        outcomes.insert(game::Position { row: 0, column: 1 }, Outcome::CatsGame);
        outcomes.insert(game::Position { row: 0, column: 2 }, Outcome::CatsGame);
        // A set is used to see which positions were picked.
        let mut positions_set = HashSet::new();

        // This test exercises code that has random behavior. Therefore, we act
        // multiple times to hopefully cover the distribution of outcomes.
        const NUM_TIMES: i32 = 300;
        for _ in 0..NUM_TIMES {
            let position = best_position(&outcomes);
            positions_set.insert(position);
        }

        // Given a sufficient number of times getting the best position we expect
        // each position to be returned at least once.
        assert_eq!(
            outcomes.len(),
            positions_set.len(),
            "This test relies on random behavior. Therefore, it is possible, \
             although highly unlikely, that the test could fail even if the \
             code is working as expected. If this happens try re-running the \
             test a few times. Continual failures indicate there is a problem \
             that needs addressed in the code as the requirement of picking \
             random positions is not being fulfilled."
        );
    }

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
    fn outcome_from_game_state_when_cats_game_should_be_cats_game() {
        let game_state = game::State::CatsGame;
        let ai_player = AIPlayer::PlayerX;
        let expected_outcome = Outcome::CatsGame;

        let actual_outcome = Outcome::from_game_state(game_state, ai_player);

        assert_eq!(expected_outcome, actual_outcome);
    }

    #[test]
    fn outcome_from_game_state_when_player_X_win_and_player_X_should_be_win() {
        let game_state = game::State::PlayerXWin(Default::default());
        let ai_player = AIPlayer::PlayerX;
        let expected_outcome = Outcome::Win;

        let actual_outcome = Outcome::from_game_state(game_state, ai_player);

        assert_eq!(expected_outcome, actual_outcome);
    }

    #[test]
    fn outcome_from_game_state_when_player_X_win_and_player_O_should_be_loss() {
        let game_state = game::State::PlayerXWin(Default::default());
        let ai_player = AIPlayer::PlayerO;
        let expected_outcome = Outcome::Loss;

        let actual_outcome = Outcome::from_game_state(game_state, ai_player);

        assert_eq!(expected_outcome, actual_outcome);
    }

    #[test]
    fn outcome_from_game_state_when_player_O_win_and_player_O_should_be_win() {
        let game_state = game::State::PlayerOWin(Default::default());
        let ai_player = AIPlayer::PlayerO;
        let expected_outcome = Outcome::Win;

        let actual_outcome = Outcome::from_game_state(game_state, ai_player);

        assert_eq!(expected_outcome, actual_outcome);
    }

    #[test]
    fn outcome_from_game_state_when_player_O_win_and_player_X_should_be_loss() {
        let game_state = game::State::PlayerOWin(Default::default());
        let ai_player = AIPlayer::PlayerX;
        let expected_outcome = Outcome::Loss;

        let actual_outcome = Outcome::from_game_state(game_state, ai_player);

        assert_eq!(expected_outcome, actual_outcome);
    }

    #[test]
    fn worst_outcome_when_empty_should_be_unknown() {
        let outcomes = Default::default();
        let expected_outcome = Outcome::Unknown;

        let actual_outcome = worst_outcome(&outcomes);

        assert_eq!(expected_outcome, actual_outcome);
    }

    #[test]
    fn worst_outcome_when_win_and_loss_should_be_loss() {
        let outcomes = [Outcome::Win, Outcome::Loss].iter().cloned().collect();
        let expected_outcome = Outcome::Loss;

        let actual_outcome = worst_outcome(&outcomes);

        assert_eq!(expected_outcome, actual_outcome);
    }

    #[test]
    fn worst_outcome_when_cats_game_and_loss_should_be_loss() {
        let outcomes = [Outcome::CatsGame, Outcome::Loss].iter().cloned().collect();
        let expected_outcome = Outcome::Loss;

        let actual_outcome = worst_outcome(&outcomes);

        assert_eq!(expected_outcome, actual_outcome);
    }

    #[test]
    fn worst_outcome_when_cats_game_and_cats_game_should_be_cats_game() {
        let outcomes = [Outcome::Win, Outcome::CatsGame].iter().cloned().collect();
        let expected_outcome = Outcome::CatsGame;

        let actual_outcome = worst_outcome(&outcomes);

        assert_eq!(expected_outcome, actual_outcome);
    }
}
