use open_ttt_lib::{ai, game};

// Ensures the unbeatable opponent never loses.
//
// The unbeatable opponent fully evaluates every possible move and countermove
// to pick the best position. However, if there is an issue, it might lose to a
// random opponent. This test plays several games to ensure the unbeatable
// opponent does not lose.
//
// This test could take a while to exercise so it is disabled by default.
#[test]
#[ignore]
fn unbeatable_opponent_should_never_lose() {
    // The number of games to play. A larger number makes the test take longer
    // to run, but due to the random nature of the test, more likely to find
    // any possible issues.
    const NUM_GAMES: usize = 100;

    let mut game = game::Game::new();

    let random_ai = ai::Opponent::new(ai::Difficulty::None);
    let unbeatable_ai = ai::Opponent::new(ai::Difficulty::Unbeatable);

    for _ in 0..NUM_GAMES {
        let mut move_log = Vec::new();
        loop {
            match game.state() {
                game::State::PlayerXMove => {
                    let position = random_ai.get_move(&game).unwrap();
                    move_log.push(format!("  Random AI as X: {:?}", position));
                    game.do_move(position).unwrap();
                }
                game::State::PlayerOMove => {
                    let position = unbeatable_ai.get_move(&game).unwrap();
                    move_log.push(format!("  Unbeatable AI as O: {:?}", position));
                    game.do_move(position).unwrap();
                }
                game::State::PlayerXWin(_) => {
                    panic!(
                        "\nThe random AI has won over the unbeatable AI. \
                        \n\nList of moves: \n{}\n \
                        \nThe final game board: \n{}\n",
                        move_log.join("\n"),
                        game.board()
                    );
                }
                game::State::PlayerOWin(_) => {
                    break;
                }
                game::State::CatsGame => {
                    break;
                }
            };
        }
        game.start_next_game();
    }
}

// Ensures the easy, medium, and hard are progressively harder by battling a
// a random opponent. We expect the harder difficulties to win more often than
// the easier ones.
#[test]
#[ignore]
fn easy_medium_hard_difficulties_should_increasingly_win_vs_none_difficulty() {
    let easy_scores = battle(ai::Difficulty::Easy, ai::Difficulty::None);
    let medium_scores = battle(ai::Difficulty::Medium, ai::Difficulty::None);
    let hard_scores = battle(ai::Difficulty::Hard, ai::Difficulty::None);

    assert!(
        easy_scores.wins < medium_scores.wins,
        "The Easy difficulty with {} wins has unexpectedly won more than the \
         Medium difficulty with {} wins.",
        easy_scores.wins,
        medium_scores.wins
    );
    assert!(
        medium_scores.wins < hard_scores.wins,
        "The Medium difficulty with {} wins has unexpectedly won more than the \
         Hard difficulty with {} wins.",
        medium_scores.wins,
        hard_scores.wins
    );
}

// Ensures the easy, medium, and hard are progressively harder by battling a
// an unbeatable opponent. Because the unbeatable opponent is, well unbeatable,
// we expect the harder difficulties to get more cat's games than the easier
// opponents.
#[test]
#[ignore]
fn easy_medium_hard_difficulties_should_increasingly_tie_vs_unbeatable_difficulty() {
    let easy_scores = battle(ai::Difficulty::Easy, ai::Difficulty::Unbeatable);
    let medium_scores = battle(ai::Difficulty::Medium, ai::Difficulty::Unbeatable);
    let hard_scores = battle(ai::Difficulty::Hard, ai::Difficulty::Unbeatable);

    assert!(
        easy_scores.cats_games < medium_scores.cats_games,
        "The Easy difficulty with {} cat's games has unexpectedly tied more \
         than the Medium difficulty with {} cat's games.",
        easy_scores.cats_games,
        medium_scores.cats_games
    );
    assert!(
        medium_scores.cats_games < hard_scores.cats_games,
        "The Medium difficulty with {} cat's games has unexpectedly tied more \
        than the Hard difficulty with {} cat's games.",
        medium_scores.cats_games,
        hard_scores.cats_games
    );

    // Also have a sanity check that during the battle, the unbeatable was in
    // fact unbeaten. If these asserts fail then consider seeing if the
    // unbeatable_opponent_should_never_lose test can recreate the failure.
    assert_eq!(
        easy_scores.wins, 0,
        "The Easy opponent with {} wins has unexpectedly won over the \
         unbeatable opponent.",
        easy_scores.wins
    );
    assert_eq!(
        medium_scores.wins, 0,
        "The Medium opponent with {} wins has unexpectedly won over the \
         Unbeatable opponent.",
        medium_scores.wins
    );
    assert_eq!(
        hard_scores.wins, 0,
        "The Hard opponent with {} wins has unexpectedly won over the \
         Unbeatable opponent.",
        hard_scores.wins
    );
}

fn battle(difficulty: ai::Difficulty, reference_difficulty: ai::Difficulty) -> BattleScores {
    // The number of games to play in a battle. A larger number makes the test
    // take longer to run, but due to the random nature of the test, more
    // likely to find any possible issues.
    const NUM_GAMES: i32 = 100;

    // The game logic ensures each opponent takes turns taking the first move,
    // thus start_next_game() is used instead of creating a new game once the
    // game is over.
    let mut game = game::Game::new();
    let player_x = ai::Opponent::new(difficulty);
    let player_o = ai::Opponent::new(reference_difficulty);
    let mut scores = BattleScores::new();

    while scores.total_games() < NUM_GAMES {
        match game.state() {
            game::State::PlayerXMove => {
                let position = player_x.get_move(&game).unwrap();
                game.do_move(position).unwrap();
            }
            game::State::PlayerOMove => {
                let position = player_o.get_move(&game).unwrap();
                game.do_move(position).unwrap();
            }
            game::State::PlayerXWin(_) => {
                scores.wins += 1;
                game.start_next_game();
            }
            game::State::PlayerOWin(_) => {
                scores.losses += 1;
                game.start_next_game();
            }
            game::State::CatsGame => {
                scores.cats_games += 1;
                game.start_next_game();
            }
        };
    }

    scores
}

struct BattleScores {
    wins: i32,
    losses: i32,
    cats_games: i32,
}

impl BattleScores {
    fn new() -> Self {
        BattleScores {
            wins: 0,
            losses: 0,
            cats_games: 0,
        }
    }

    fn total_games(&self) -> i32 {
        self.wins + self.losses + self.cats_games
    }
}
