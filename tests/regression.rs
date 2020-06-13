use open_ttt_lib::{ai, game};

// Ensures the flawless AI never loses.
//
// The flawless AI fully evaluates every possible move and countermove to pick
// the best position. However, if there is an issue, it might lose to a random
// AI. This test plays several games to ensure the flawless AI does not lose.
//
// This test could take a while to exercise so it is disabled by default.
#[test]
#[ignore]
fn flawless_ai_should_never_lose() {
    // The number of games to play. A larger number makes the test take longer
    // to run, but due to the random nature of the test, more likely to find
    // any possible issues.
    const NUM_GAMES: usize = 100;

    let mut game = game::Game::new();

    let random_ai = ai::Opponent::new(1.0);
    let flawless_ai = ai::Opponent::new(0.0);

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
                    let position = flawless_ai.get_move(&game).unwrap();
                    move_log.push(format!("  Flawless AI as O: {:?}", position));
                    game.do_move(position).unwrap();
                }
                game::State::PlayerXWin(_) => {
                    panic!(
                        "\nThe random AI has won over the flawless AI. \
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
