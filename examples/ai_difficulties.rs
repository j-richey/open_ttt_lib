//! Example showing the different AI difficulties.

use rand::Rng;
use std::fmt;
use std::io;
use std::io::prelude::*;
use std::time;

use open_ttt_lib::{ai, game};

const INSTRUCTIONS: &str = r#"
AI Difficulty Examples
======================

This example shows how the different AI difficulties compare. AI opponents using
various difficulties play a series of games. The generated table shows the
percentage of wins, losses, and cat's games for each difficulty compared to the
None difficulty which places marks randomly and the Unbeatable difficulty which
never makes a mistake.

This example also demonstrates how to create custom difficulties. Try modifying
the `should_evaluate_node()` function and see how it compares to the builtin
difficulties.

Note: run this example runs significantly faster with the --release flag: e.g:
$ cargo run --release --example ai_difficulties
"#;

// The number of games to play for each battle. More games gives a more accurate
// representation of how the difficulties compare, but takes longer to run.
const NUM_GAMES: i32 = 100;

// Custom difficulty's should evaluate node function. Modify this function to
// experiment with custom difficulties.
fn should_evaluate_node(depth: i32) -> bool {
    if depth == 0 {
        true
    } else {
        let evaluate_node_probability = 0.8;
        rand::thread_rng().gen_bool(evaluate_node_probability)
    }
}

fn main() {
    println!("{}", INSTRUCTIONS);
    print_table_header();

    evaluate_difficulty(ai::Difficulty::None);
    evaluate_difficulty(ai::Difficulty::Easy);
    evaluate_difficulty(ai::Difficulty::Medium);
    evaluate_difficulty(ai::Difficulty::Hard);
    evaluate_difficulty(ai::Difficulty::Custom(should_evaluate_node));
    evaluate_difficulty(ai::Difficulty::Unbeatable);
}

// Compares the provided difficulty to the reference difficulties. The results
// are printed to the screen.
fn evaluate_difficulty(difficulty: ai::Difficulty) {
    let difficulty_name = get_difficulty_name(&difficulty);

    let none_scores = battle(difficulty, ai::Difficulty::None);
    let unbeatable_scores = battle(difficulty, ai::Difficulty::Unbeatable);

    print_table_row(
        difficulty_name,
        &none_scores.to_string(),
        &unbeatable_scores.to_string(),
    );
}

// Has AI opponents of the provided difficulties play a series of games counting
// the wins for each player. Depending on the number of games being played, this
// function might take a while, so the progress of the battle is occasionally
// printed.
fn battle(
    player_x_difficulty: ai::Difficulty,
    player_o_difficulty: ai::Difficulty,
) -> BattleScores {
    // The game logic ensures each opponent takes turns taking the first move,
    // thus start_next_game() is used instead of creating a new game once the
    // game is over.
    let mut game = game::Game::new();

    let player_x_name = get_difficulty_name(&player_x_difficulty);
    let player_x = ai::Opponent::new(player_x_difficulty);
    let player_o_name = get_difficulty_name(&player_o_difficulty);
    let player_o = ai::Opponent::new(player_o_difficulty);

    let mut scores = BattleScores::new();

    let mut last_print_progress_time = time::Instant::now();

    while scores.total_games() < NUM_GAMES {
        // Play one turn of the either getting asking one of the AI players to
        // pick a position or if the game is over updating the scores and starting
        // the next game.
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
                scores.player_x_wins += 1;
                game.start_next_game();
            }
            game::State::PlayerOWin(_) => {
                scores.player_o_wins += 1;
                game.start_next_game();
            }
            game::State::CatsGame => {
                scores.cats_games += 1;
                game.start_next_game();
            }
        };

        print_battle_progress(
            scores.total_games(),
            player_x_name,
            player_o_name,
            &mut last_print_progress_time,
        );
    }

    scores
}

// Prints the table's header.
fn print_table_header() {
    println!("{:10}  {:^18}  {:^18}", "Difficulty", "None", "Unbeatable");
    println!("{:=<10}  {:=<18}  {:=<18}", "", "", "");
}

// Prints a row of the table.
fn print_table_row(col_1: &str, col_2: &str, col_3: &str) {
    println!("{:10}  {:18}  {:18}", col_1, col_2, col_3);
}

// Occasionally prints the progress of a battle.
fn print_battle_progress(
    games_played: i32,
    player_x_name: &str,
    player_o_name: &str,
    last_update_time: &mut time::Instant,
) {
    // The time between updates is set so users can see the program is making
    // progress but so it does not go so fast that the display is just a blur.
    const UPDATE_INTERVAL: time::Duration = time::Duration::from_millis(100);

    if last_update_time.elapsed() >= UPDATE_INTERVAL {
        // Create a description of the progress using the player names abd
        // number of games played.
        let battle_progress = games_played as f64 / NUM_GAMES as f64;
        let progress_text = format!(
            "{} vs. {}  game {} of {}, ({:.0}%)",
            player_x_name,
            player_o_name,
            games_played,
            NUM_GAMES,
            battle_progress * 100.0
        );
        // Print the progress text. The text is padded with spaces and ended with
        // a carriage return so old progress text is overwritten with new text.
        // Also, the standard output is flushed so the user sees the text we
        // printed instead of it getting stuck in the buffer.
        print!("{:50}\r", progress_text);
        let _ignored_result = io::stdout().flush();
        *last_update_time = time::Instant::now();
    }
}

// Gets the name of a provided AI difficulty.
fn get_difficulty_name(difficulty: &ai::Difficulty) -> &str {
    match difficulty {
        ai::Difficulty::None => "None",
        ai::Difficulty::Easy => "Easy",
        ai::Difficulty::Medium => "Medium",
        ai::Difficulty::Hard => "Hard",
        ai::Difficulty::Unbeatable => "Unbeatable",
        ai::Difficulty::Custom(_) => "Custom",
    }
}

// Holds the battle's scores and provides convenience methods for calculating
// the percentage of wins or cats games.
struct BattleScores {
    player_x_wins: i32,
    player_o_wins: i32,
    cats_games: i32,
}

impl BattleScores {
    fn new() -> Self {
        BattleScores {
            player_x_wins: 0,
            player_o_wins: 0,
            cats_games: 0,
        }
    }

    fn total_games(&self) -> i32 {
        self.player_x_wins + self.player_o_wins + self.cats_games
    }

    fn player_x_win_percent(&self) -> f64 {
        self.calculate_percent(self.player_x_wins)
    }

    fn player_o_win_percent(&self) -> f64 {
        self.calculate_percent(self.player_o_wins)
    }

    fn cats_game_percent(&self) -> f64 {
        self.calculate_percent(self.cats_games)
    }

    fn calculate_percent(&self, value: i32) -> f64 {
        if self.total_games() > 0 {
            let fraction = value as f64 / self.total_games() as f64;
            fraction * 100.0
        } else {
            0.0
        }
    }
}

impl fmt::Display for BattleScores {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:3.0}% - {:3.0}% - {:3.0}%",
            self.player_x_win_percent(),
            self.player_o_win_percent(),
            self.cats_game_percent()
        )?;
        Ok(())
    }
}
