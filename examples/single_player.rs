//! Example single player Tic Tac Toe game.

use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::Write;

use open_ttt_lib::{ai, board, game};

const INSTRUCTIONS: &str = r#"
Single Player Example Game
==========================

This example shows creating a console based single player Tic Tac Toe game. A
human playing as 'X' and an AI opponent playing as 'O' take turns placing their
marks.

This example includes showing how the AI opponent views the game. The following
characters are used for the game board's display:
  X - Player 'X' owns the square.
  O - Player 'O' owns the square.
  w - The AI opponent will win if it places its mark at this location.
  l - The AI opponent could lose if it places its mark at this location.
  c - This location leads to a cat's game --- neither player wins.
  ? - The AI opponent could not determine the outcome of this location.

Type 'exit' or press Ctrl+C to exit the example.
"#;

fn main() {
    // Create a new game. This needs to be mutable since selecting positions
    // changes the state of the game.
    let mut game = game::Game::new();

    // Adjust the mistake probability to make the AI opponent or harder. As the
    // mistake probability is increased the AI is more likely to be unable to
    // determine the outcome of choosing a particular position.
    let mistake_probability = 0.0;
    let opponent = ai::Opponent::new(mistake_probability);

    println!("{}", INSTRUCTIONS);

    let mut exit_game = false;
    while !exit_game {
        // Determine the action to take based on the current state of the game.
        match game.state() {
            game::State::PlayerXMove => {
                println!("\nPlayer X's turn...\n");
                display_board(&game.board(), None, None);

                // In this example the human player is playing as 'X'. A helper
                // function takes care of the details of getting and parsing
                // the player input.
                exit_game = !do_player_move(&mut game);
            }
            game::State::PlayerOMove => {
                println!("\nPlayer O's turn...\n");

                // The AI opponent is playing as player O. We have the AI
                // opponent evaluate the game. This returns a collection that
                // indicates what would happen if the opponent chose each free
                // square. This is displayed so we can get some insight to how
                // the opponent views the game.
                let ai_outcomes = opponent.evaluate_game(&game);
                display_board(&game.board(), None, Some(&ai_outcomes));

                // Have the opponent pick the best position from the available
                // outcomes.
                game.do_move(ai::best_position(&ai_outcomes).unwrap())
                    .unwrap();
            }

            // Handle the game over states. The winning states are provided the
            // collection of positions that contributed to the win
            game::State::PlayerXWin(winning_positions) => {
                println!("\nGame Over: Player X wins!\n");
                display_board(&game.board(), Some(&winning_positions), None);

                println!("\n\n=== Starting Next Game ===");
                // Tell the game to start the next game. This is preferred over
                // creating a new game as this ensures each player takes turns
                // performing the first move.
                game.start_next_game();
            }
            game::State::PlayerOWin(winning_positions) => {
                println!("\nGame Over: Player O wins!\n");
                display_board(&game.board(), Some(&winning_positions), None);

                println!("\n\n=== Starting Next Game ===");
                game.start_next_game();
            }
            game::State::CatsGame => {
                println!("\nGame Over: cat's game.\n");
                display_board(&game.board(), None, None);

                println!("\n\n=== Starting Next Game ===");
                game.start_next_game();
            }
        };
    }
}

/// Asks the user to pick a square and updates the game accordingly.
///
/// False is returned if the player wishes to exit the game, true is returned
/// otherwise.
fn do_player_move(game: &mut game::Game) -> bool {
    print!("\nSelect a square: ");

    // Get the user input and see if they wish to exit the game.
    let input = get_user_input();
    if input.to_lowercase().trim() == "exit" {
        return false;
    }

    if let Some(position) = parse_input(&input) {
        // Attempt to move into the requested position. An error is returned if
        // the position is already owned or otherwise invalid. The error
        // contains details about the problem.
        if let Err(error) = game.do_move(position) {
            println!("{}", error);
        }
    } else {
        println!(
            "Invalid position of '{}' entered. Select positions using the \
            column letter and and row number. Examples: 'A1' or 'B3'",
            input.trim()
        );
    }

    true
}

/// Prints the game board to the screen.
///
/// This includes showing the row and column labels, marking wining positions,
/// and optionally showing the AI's view of the board.
///
/// Note: `board::Board` implements the `Display` trait which provides a basic
/// view of the board. We use our own custom display function here so we can
/// show additional information such as the key mappings and winning positions.
fn display_board(
    board: &board::Board,
    winning_positions: Option<&HashSet<board::Position>>,
    ai_outcomes: Option<&HashMap<game::Position, ai::Outcome>>,
) {
    let empty_winning_positions = HashSet::new();
    let empty_ai_outcomes = HashMap::new();

    // Print the board's column labels.
    assert!(board.size().columns == 3);
    println!("     A   B   C");

    // Print each row including the separators and content.
    for row in 0..board.size().rows {
        display_row_separator(&board);
        display_row_content(
            &board,
            row,
            &winning_positions
                .or(Some(&empty_winning_positions))
                .unwrap(),
            &ai_outcomes.or(Some(&empty_ai_outcomes)).unwrap(),
        );
    }

    // Display the final separator to finish off the board.
    display_row_separator(&board);
}

/// Prints the row separator marks.
fn display_row_separator(board: &board::Board) {
    print!("   ");
    for _ in 0..board.size().columns {
        print!("+---");
    }
    println!("+");
}

/// Prints the content of a row.
///
/// The content includes the owner of each square, marking the winning positions,
/// and printing the AI's view of the board.
fn display_row_content(
    board: &board::Board,
    row: i32,
    winning_positions: &HashSet<board::Position>,
    ai_outcomes: &HashMap<game::Position, ai::Outcome>,
) {
    // Print the row label. Note, the row labels are one indexed.
    print!(" {} ", row + 1);

    for column in 0..board.size().columns {
        let position = game::Position { row, column };

        // Get the mark to use for the position. First, the player characters
        // are considered, then the AI outcomes are examined. Finally, a blank
        // space is used.
        let mark = match board.get(position).unwrap() {
            board::Owner::PlayerX => "X",
            board::Owner::PlayerO => "O",
            board::Owner::None => {
                if ai_outcomes.contains_key(&position) {
                    match ai_outcomes.get(&position).unwrap() {
                        ai::Outcome::Win => "w",
                        ai::Outcome::CatsGame => "c",
                        ai::Outcome::Loss => "l",
                        ai::Outcome::Unknown => "?",
                    }
                } else {
                    " "
                }
            }
        };

        // Get the filler character to use in the cell. This is used to mark
        // winning positions.
        let filler = if winning_positions.contains(&position) {
            "*"
        } else {
            " "
        };

        // Finally we can print the cell
        print!("|{0}{1}{0}", filler, mark);
    }

    // Print the last vertical bar to close off the cell.
    println!("|");
}

/// Reads user input returning a corresponding value.
fn get_user_input() -> String {
    // Flush the output before reading any user input.
    io::stdout().flush().unwrap();

    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line.");

    value
}

/// Converts the provided input string into a board position.
///
/// The input string is expected to contain two characters: a letter indicating
/// the column and a number indicating the row. Column letters start at A and
/// are case insensitive. Row numbers start at 1.
///
/// None is returned if the input could not be parsed.
fn parse_input(value: &str) -> Option<board::Position> {
    // First, trim the string and normalize the case.
    let normalized_string = value.trim().to_uppercase();

    // Ensure the resulting string is the expected length.
    if normalized_string.len() != 2 {
        return None;
    }

    // Get the first character and convert it to the column index.
    let column = if let Some(column_char) = normalized_string.chars().next() {
        match column_char {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            _ => return None,
        }
    } else {
        return None;
    };

    // Get the last character and convert it to the row index.
    let row = if let Some(row_char) = normalized_string.chars().last() {
        match row_char {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            _ => return None,
        }
    } else {
        return None;
    };

    Some(board::Position { row, column })
}
