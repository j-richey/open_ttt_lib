//! Provides functionality for creating single player games.
//!
//! # Examples
//! ```
//! use open_ttt_lib::ai;
//! use open_ttt_lib::game;
//!
//! let game = game::Game::new();
//! let ai_opponent = ai::Opponent::new(ai::Difficulty::Hard);
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
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Opponent {
    difficulty: Difficulty,
}

impl Opponent {
    /// Constructs a new AI opponent using the provided difficulty.
    ///
    /// # Examples
    ///
    /// Construct a hard AI opponent:
    /// ```
    /// use open_ttt_lib::ai;
    ///
    /// let hard_opponent = ai::Opponent::new(ai::Difficulty::Hard);
    /// ```
    ///
    /// Construct an AI opponent that randomly picks positions:
    /// ```
    /// use open_ttt_lib::ai;
    ///
    /// let rando = ai::Opponent::new(ai::Difficulty::None);
    /// ```
    pub fn new(difficulty: Difficulty) -> Self {
        Self { difficulty }
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
    /// let ai_opponent = ai::Opponent::new(ai::Difficulty::Medium);
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
    /// help human players pick a position or when fine-tuning the AI difficulty
    /// settings.
    ///
    /// # Examples
    /// ```
    /// use open_ttt_lib::ai;
    /// use open_ttt_lib::game;
    ///
    /// let game = game::Game::new();
    /// let ai_opponent = ai::Opponent::new(ai::Difficulty::Medium);
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
        // Check if there is a cached result that saves us from reevaluating the game,
        // otherwise we evaluate the outcome of each position.
        if let Some(outcomes) = self.get_cached_outcomes(&game) {
            outcomes
        } else {
            let mut outcomes = HashMap::new();

            // Determine which player the AI is playing as. Note: we can only
            // determine the AI player if the game is not over, thus we rely on
            // the get_cached_result() call above to handle game over conditions.
            let ai_player = AIPlayer::from_game_state(game.state());

            // For each free square, evaluate the consequences of using that
            // square. The outcome for each position and the position is recorded.
            for position in game.free_positions() {
                let outcome = self.evaluate_position(&game, position, ai_player, 0);
                outcomes.insert(position, outcome);
            }

            outcomes
        }
    }

    // Evaluates what outcome of the game would be by selecting a specific position.
    //
    // This function uses depth first search to examine all possible game outcomes
    // based on the current state of the game board. The algorithm selects a free
    // position then traverses the tree looking for one of the end game
    // conditions: win, loss, or cat’s game. Once the end of the game is found,
    // the result is propagated up the tree. The algorithm takes turns playing
    // as each player and picks the best outcome for the given player.
    //
    // The depth search algorithm can see to the end of the game, thus it cannot
    // be beat. The best possible outcome is a cat’s game. Therefore, the AI's
    // difficulty is checked to see if the current node should be evaluated.
    // Disregarding parts of the solution tree gives human players a chance to win.
    //
    // # Notes
    // * The time complexity of this function is O(n!) where n is the number of
    //   free positions.
    // * This is a recursive function.
    fn evaluate_position(
        &self,
        game: &game::Game,
        position: game::Position,
        ai_player: AIPlayer,
        depth: i32,
    ) -> Outcome {
        // Since this is a recursive function, ensure we have not made a mistake
        // that has lead to us trying to recursive too deep, a sign of potential
        // infinite recursion that can cause a stack overflow.
        const MAX_RECURSION_DEPTH: i32 = 20;
        assert!(
            depth <= MAX_RECURSION_DEPTH,
            "The AI algorithm has reached the maximum recursion limit of {} and \
             cannot continue to evaluate the game. This condition is the result \
             of a bug in the open_ttt_lib used by this application.",
            depth
        );
        debug_assert!(
            game.can_move(position),
            "Cannot move into the provided position, {:?}. Thus, the position \
             cannot be evaluated. Ensure the game is not over and the position \
             is free. This condition is the result of a bug in the open_ttt_lib \
             used by this application.",
            position
        );

        // Ask the difficulty if this node should actually be evaluated.
        if !self.difficulty.should_evaluate_node(depth) {
            return Outcome::Unknown;
        }

        // Check to see if this position is being considered for this AI instance
        // or the if we are simulating the move for the other player.
        let is_my_turn = ai_player == AIPlayer::from_game_state(game.state());

        // Clone the game so we can try out the move without modifying the original game.
        let mut game = game.clone();
        let state = game.do_move(position).unwrap();

        // Check to see if the game is over. If so, return the outcome of the
        // game from the AI's perspective, e.g. win, loss, or cat's game.
        if state.is_game_over() {
            return Outcome::from_game_state(state, ai_player);
        }

        // The game is not over, to evaluate each of the remaining free squares
        // looking for the worst outcome for the AI player. We return early if
        // the worst outcome is found as there is no need to continue evaluating
        // the tree saving a lot of CPU cycles.
        // Note: the game automatically takes care of switching between each
        // player's turn.
        let mut outcomes = HashSet::new();
        for free_position in game.free_positions() {
            let outcome = self.evaluate_position(&game, free_position, ai_player, depth + 1);

            if is_worst_outcome(outcome, is_my_turn) {
                return outcome;
            }

            outcomes.insert(outcome);
        }

        // The AI assumes the other player plays a perfect game, so return the
        // worst outcome that was found.
        worst_outcome(&outcomes, is_my_turn)
    }

    // Gets a cached collection of outcomes based on the provided game.
    // None is returned if there are no cached outcomes for the provided game.
    //
    // Using cached outcomes helps speed up evaluating the game. However, we
    // want some random behavior from the AI we allow the AI to make mistakes
    // we only cache key outcomes. This provides a balance of evaluation speed
    // while keeping the AI interesting and human like.
    fn get_cached_outcomes(&self, game: &game::Game) -> Option<HashMap<game::Position, Outcome>> {
        if game.state().is_game_over() {
            // For games that are over an empty map is returned.
            Some(HashMap::new())
        } else if is_new_game(&game) {
            // For new games we know that the worst outcome for every position
            // is a cat's game --- if this were not the case then the game would
            // no tbe fair.
            let outcomes =
                initialize_free_position_outcomes(game.free_positions(), Outcome::CatsGame);
            Some(outcomes)
        } else {
            None
        }
    }
}

/// Selects the difficulty used by the [`Opponent`](struct.Opponent.html).
///
/// The exact behavior of `Easy`, `Medium`, and `Hard` difficulties are set via
/// play testing and are subject to adjustment in future library versions.
#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub enum Difficulty {
    /// The `Opponent` picks random positions and does not actually evaluate the
    /// game.
    None,

    /// Intended for players who are new to tic-tac-toe to learn the rules of
    /// the game. The `Opponent` mostly picks random squares, but occasionally
    /// goes for the win or blocks the player from winning.
    Easy,

    /// Medium difficulty is for players who have some experience with
    /// tic-tac-toe. The AI provides a challenge to the player but games are
    /// still winnable, especially if the player plans several moves ahead.
    Medium,

    /// At hard difficulty the computer plays almost perfect games. The player
    /// must capitalize on rare mistakes made by the computer to win. This is
    /// the recommended difficulty for experienced tic-tac-toe players.
    Hard,

    /// The `Opponent` plays perfect games and cannot loose. The best outcome
    /// for the player is a cat's game.
    Unbeatable,

    /// Provides full control over the `Opponent`'s difficulty via the provided
    /// function.
    ///
    /// The AI algorithm selects a free position then traverses the tree of all
    /// possible moves looking for one of the end game conditions: *win*, *loss*,
    /// or *cat's game*. The provided function is invoked before processing each
    /// node in the outcome tree. Return `true` to evaluate the node. Return
    /// `false` to stop processing the node, and all child nodes thus preventing
    /// the algorithm from considering the outcomes from that branch of the tree.
    ///
    /// The depth of the node being considered is provided as the function's
    /// parameter so the custom difficulty can take into account how many moves
    /// ahead the `Opponent` is looking ahead. E.g. the `Opponent` could be more
    /// likely to make mistakes the farther ahead it looks. The depth starts at
    /// zero.
    ///
    /// # Notes
    /// * The number of nodes to evaluate for a game can be large resulting in
    ///   the provided function being invoked many times when evaluating a game.
    /// * The AI algorithms contain speed optimizations that might skip
    ///   evaluating part or all of the outcome tree. In these cases the
    ///   provided function is not called.
    ///
    /// # Examples
    /// Implement custom difficulties with the same behavior as the `None` and
    /// `Unbeatable` variants:
    /// ```
    /// use open_ttt_lib::ai;
    /// let same_as_none = ai::Difficulty::Custom(|_| false);
    /// let same_as_unbeatable = ai::Difficulty::Custom(|_| true);
    /// ```
    ///
    /// Create a custom difficulty that is perfect when looking at the current
    /// move and has a fixed probability of failing to consider deeper parts
    /// of the tree.
    /// ```
    /// use rand::Rng;
    /// use open_ttt_lib::ai;
    ///
    /// fn should_evaluate_node(depth: i32) -> bool {
    ///     if depth == 0 {
    ///         true
    ///     } else {
    ///         let evaluate_node_probability = 0.8;
    ///         rand::thread_rng().gen_bool(evaluate_node_probability)
    ///     }
    /// }
    ///
    /// let custom_difficulty = ai::Difficulty::Custom(should_evaluate_node);
    /// ```
    Custom(fn(depth: i32) -> bool),
}

impl Difficulty {
    // Based on the difficulty and current depth of the outcome tree,
    // indicates if the `Opponent` should evaluate the current node.
    fn should_evaluate_node(&self, depth: i32) -> bool {
        match self {
            Self::None => Difficulty::none_should_evaluate_node(),
            Self::Easy => Difficulty::easy_should_evaluate_node(depth),
            Self::Medium => Difficulty::medium_should_evaluate_node(depth),
            Self::Hard => Difficulty::hard_should_evaluate_node(depth),
            Self::Unbeatable => Difficulty::unbeatable_should_evaluate_node(),
            Self::Custom(custom_should_evaluate_node) => custom_should_evaluate_node(depth),
        }
    }

    // None does not evaluate any nodes, thus making the opponent pick a random
    // position.
    fn none_should_evaluate_node() -> bool {
        false
    }

    // Easy has a 50/50 chance of going for a win or blocking a loss. Otherwise,
    // it does not evaluate the tree.
    fn easy_should_evaluate_node(depth: i32) -> bool {
        if depth == 0 {
            rand::thread_rng().gen_bool(0.5)
        } else {
            false
        }
    }

    // Medium high chance of going for the win or blocking a loss. However, as
    // the tree gets deeper it is more likely not evaluate that part of the tree.
    fn medium_should_evaluate_node(depth: i32) -> bool {
        if depth == 0 {
            rand::thread_rng().gen_bool(0.9)
        } else {
            rand::thread_rng().gen_bool(0.75)
        }
    }

    // Hard looks several moves ahead. Past that there is a small chance if it
    // not evaluating a node.
    fn hard_should_evaluate_node(depth: i32) -> bool {
        if depth <= 1 {
            true
        } else {
            rand::thread_rng().gen_bool(0.97)
        }
    }

    // Unbeatable evaluates all nodes causing the opponent to play a perfect game.
    fn unbeatable_should_evaluate_node() -> bool {
        true
    }
}

/// Represents a game outcome for the AI opponent.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Outcome {
    /// The AI player wins the game.
    Win,

    /// The AI player loses the game.
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
/// let ai_opponent = ai::Opponent::new(ai::Difficulty::Medium);
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

// Initializes the outcomes for the provided positions to the specified value.
fn initialize_free_position_outcomes(
    free_positions: game::FreePositions,
    outcome: Outcome,
) -> HashMap<game::Position, Outcome> {
    let mut outcomes = HashMap::new();
    for position in free_positions {
        outcomes.insert(position, outcome);
    }

    outcomes
}

// Gets an array of worst to best game outcomes for the AI player .
//
// The worst possible outcome depends on if is it the turn of this AI opponent
// or if it is simulating the other player. The work outcome for this AI opponent
// is `Loss`, `CatsGame`, `Win`. If it's the other player's turn the ordering is
// reversed.
fn worst_to_best_outcomes(is_my_turn: bool) -> [Outcome; 3] {
    if is_my_turn {
        [Outcome::Loss, Outcome::CatsGame, Outcome::Win]
    } else {
        [Outcome::Win, Outcome::CatsGame, Outcome::Loss]
    }
}

// Returns true if the provided outcome is the worst outcome for the AI opponent,
// otherwise false is returned,
fn is_worst_outcome(outcome: Outcome, is_my_turn: bool) -> bool {
    const WORST_OUTCOME_INDEX: usize = 0;
    worst_to_best_outcomes(is_my_turn)[WORST_OUTCOME_INDEX] == outcome
}

// Gets the worst possible outcome based on the provided outcomes.
//
// `Unknown` is returned if the provided slice is empty or only contains unknown
// outcomes.
fn worst_outcome(outcomes: &HashSet<Outcome>, is_my_turn: bool) -> Outcome {
    // Search through the outcomes, from worst to best, returning the first one found.
    for outcome in worst_to_best_outcomes(is_my_turn).iter() {
        if outcomes.contains(outcome) {
            return *outcome;
        }
    }

    // None of the other outcomes were found so return unknown.
    Outcome::Unknown
}

// Returns true if the provided game is a new game; that is all positions are
// free.
fn is_new_game(game: &game::Game) -> bool {
    let board_size = game.board().size();
    let total_positions = board_size.columns * board_size.rows;

    game.free_positions().count() as i32 == total_positions
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
    fn opponent_new_should_set_difficulty() {
        let expected_difficulty = Difficulty::Medium;

        let opponent = Opponent::new(expected_difficulty);
        let actual_difficulty = opponent.difficulty;

        assert_eq!(expected_difficulty, actual_difficulty);
    }

    #[test]
    fn opponent_get_move_when_game_is_over_should_be_none() {
        // Create a game where the game is over.
        let game = create_game(&PLAYER_X_WIN);
        let opponent = Opponent::new(Difficulty::None);
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
    fn opponent_get_move_when_unbeatable_difficulty_should_pick_wining_position() {
        // Create a game where the AI player has a wining move available.
        // The unbeatable AI should pick this position.
        let game = create_game(&PLAYER_X_MOVE_WITH_WIN_AVAILABLE);
        let opponent = Opponent::new(Difficulty::Unbeatable);
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
    fn opponent_evaluate_game_when_new_game_and_unbeatable_difficulty_should_be_cats_game_for_all_positions(
    ) {
        let game = game::Game::new();
        let opponent = Opponent::new(Difficulty::Unbeatable);
        let expected_outcomes =
            initialize_free_position_outcomes(game.free_positions(), Outcome::CatsGame);

        let actual_outcomes = opponent.evaluate_game(&game);

        assert_eq!(
            expected_outcomes,
            actual_outcomes,
            "\nGame board used for this test: \n{}",
            game.board()
        );
    }

    #[test]
    fn opponent_evaluate_game_when_game_over_should_be_empty_map() {
        let game = create_game(&PLAYER_X_WIN);
        let opponent = Opponent::new(Difficulty::Unbeatable);
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
    fn opponent_evaluate_game_when_unbeatable_difficulty_should_evaluate_all_positions() {
        // Create a game where the AI player has a wining move available.
        // The unbeatable AI should determine the outcome of all remaining positions.
        let game = create_game(&PLAYER_X_MOVE_WITH_WIN_AVAILABLE);
        let opponent = Opponent::new(Difficulty::Unbeatable);
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
    fn opponent_evaluate_game_when_none_difficulty_should_see_unknown_outcome_for_all_positions() {
        // Create a game where the AI player has a wining move available.
        // The opponent that uses the None difficulty does not actually evaluate
        // any nodes and should see the outcome as unknown for all positions.
        let game = create_game(&PLAYER_X_MOVE_WITH_WIN_AVAILABLE);
        let opponent = Opponent::new(Difficulty::None);
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
    fn opponent_evaluate_game_depth_should_start_at_zero() {
        // We create a game that is already in progress to ensure we get past
        // some of the caching the opponent does --- returning a cached result
        // means our custom function would never be called!
        let game = create_game(&PLAYER_X_MOVE_WITH_WIN_AVAILABLE);

        let opponent = Opponent::new(Difficulty::Custom(|depth| {
            assert_eq!(depth, 0);
            // Tell the game to not evaluate any further since we are only
            // interested in the initial depth. Note: this test could also fail
            // if returning `false` does not prevent the algorithm from going
            // deeper into the tree.
            false
        }));

        opponent.evaluate_game(&game);
    }

    #[test]
    #[should_panic(expected = "The depth has been incremented.")]
    fn opponent_evaluate_game_should_increment_depth() {
        // The custom difficulty takes a function and not a closure so we use
        // a bit of a hack to ensure the provided function is called as we expect:
        // we panic when the condition is met with a specific message. Perhaps
        // there is a better way to do this in the future?
        // We create a game that is already in progress to ensure we get past
        // some of the caching the opponent does --- returning a cached result
        // means our custom function would never be called!
        let game = create_game(&PLAYER_X_MOVE_WITH_WIN_AVAILABLE);

        let opponent = Opponent::new(Difficulty::Custom(|depth| {
            if depth > 0 {
                panic!("The depth has been incremented.");
            }
            // Tell the opponent to keep evaluating nodes so it goes deeper into
            // the tree, thus hopefully incrementing the depth. Note: this test
            // could also fail if returning `true` does not result in deeper
            // parts of the tree being evaluated.
            true
        }));

        opponent.evaluate_game(&game);
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
    fn difficulty_when_custom_should_call_provided_function() {
        // To ensure our custom function is called, we create a function that
        // returns true only when a specific depth value is provided.
        const TRUE_DEPTH_VALUE: i32 = 42_000;
        let custom_difficulty = Difficulty::Custom(|depth| depth == TRUE_DEPTH_VALUE);

        // Try calling our custom function twice, once with the specific value
        // and once without it. The ensures one of the predefined difficulty
        // functions is not being called.
        assert!(custom_difficulty.should_evaluate_node(TRUE_DEPTH_VALUE));
        assert!(!custom_difficulty.should_evaluate_node(0));
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
        let is_my_turn = true;
        let expected_outcome = Outcome::Unknown;

        let actual_outcome = worst_outcome(&outcomes, is_my_turn);

        assert_eq!(expected_outcome, actual_outcome);
    }

    #[test]
    fn worst_outcome_when_my_turn_with_win_and_loss_should_be_loss() {
        let outcomes = [Outcome::Win, Outcome::Loss].iter().cloned().collect();
        let is_my_turn = true;
        let expected_outcome = Outcome::Loss;

        let actual_outcome = worst_outcome(&outcomes, is_my_turn);

        assert_eq!(expected_outcome, actual_outcome);
    }

    #[test]
    fn worst_outcome_when_my_turn_with_cats_game_and_loss_should_be_loss() {
        let outcomes = [Outcome::CatsGame, Outcome::Loss].iter().cloned().collect();
        let is_my_turn = true;
        let expected_outcome = Outcome::Loss;

        let actual_outcome = worst_outcome(&outcomes, is_my_turn);

        assert_eq!(expected_outcome, actual_outcome);
    }

    #[test]
    fn worst_outcome_when_my_turn_with_cats_game_and_cats_game_should_be_cats_game() {
        let outcomes = [Outcome::Win, Outcome::CatsGame].iter().cloned().collect();
        let is_my_turn = true;
        let expected_outcome = Outcome::CatsGame;

        let actual_outcome = worst_outcome(&outcomes, is_my_turn);

        assert_eq!(expected_outcome, actual_outcome);
    }

    #[test]
    fn worst_outcome_when_not_my_turn_with_win_and_loss_should_be_win() {
        let outcomes = [Outcome::Win, Outcome::Loss].iter().cloned().collect();
        let is_my_turn = false;
        let expected_outcome = Outcome::Win;

        let actual_outcome = worst_outcome(&outcomes, is_my_turn);

        assert_eq!(expected_outcome, actual_outcome);
    }

    #[test]
    fn worst_outcome_when_not_my_turn_with_cats_game_and_loss_should_be_cats_game() {
        let outcomes = [Outcome::CatsGame, Outcome::Loss].iter().cloned().collect();
        let is_my_turn = false;
        let expected_outcome = Outcome::CatsGame;

        let actual_outcome = worst_outcome(&outcomes, is_my_turn);

        assert_eq!(expected_outcome, actual_outcome);
    }

    #[test]
    fn worst_outcome_when_not_my_turn_with_cats_game_and_cats_game_should_be_win() {
        let outcomes = [Outcome::Win, Outcome::CatsGame].iter().cloned().collect();
        let is_my_turn = false;
        let expected_outcome = Outcome::Win;

        let actual_outcome = worst_outcome(&outcomes, is_my_turn);

        assert_eq!(expected_outcome, actual_outcome);
    }

    #[test]
    fn initialize_free_position_outcomes_should_set_indicated_outcome() {
        let game = game::Game::new();
        let expected_outcome = Outcome::Win;

        let actual_outcomes =
            initialize_free_position_outcomes(game.free_positions(), expected_outcome);

        assert!(actual_outcomes
            .iter()
            .all(|(_position, outcome)| *outcome == expected_outcome));
    }
}
