#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;

use open_ttt_lib::ai;
use open_ttt_lib::game;

// Sequence of positions that results in a cats game.
// The resulting game board is as follows:
//  +---+---+---+
//  | X | O | X |
//  +---+---+---+
//  | X | O | O |
//  +---+---+---+
//  | O | X | X |
//  +---+---+---+
// Note: for a particular game the X and O spots might be reversed.
const CATS_GAME_POSITION_SEQUENCE: [game::Position; 9] = [
    game::Position { row: 0, column: 0 },
    game::Position { row: 0, column: 1 },
    game::Position { row: 0, column: 2 },
    game::Position { row: 1, column: 1 },
    game::Position { row: 1, column: 0 },
    game::Position { row: 1, column: 2 },
    game::Position { row: 2, column: 1 },
    game::Position { row: 2, column: 0 },
    game::Position { row: 2, column: 2 },
];

// Plays a complete game that results in a cat's game.
// The exercises the speed of the game's state machine and victory condition logic.
fn complete_game_benchmark(c: &mut Criterion) {
    let mut game = game::Game::new();

    c.bench_function("Complete game resulting in cats game.", |b| {
        b.iter(|| {
            for position in CATS_GAME_POSITION_SEQUENCE.iter() {
                game.do_move(black_box(*position)).unwrap();
            }
            game.start_next_game();
        })
    });
}

// Creates a perfect AI opponent then benchmarks for various numbers of free
// spaces remaining.
fn perfect_ai_moves_benchmarks(c: &mut Criterion) {
    let mut game = game::Game::new();

    let mistake_probability = 0.0;
    let ai_opponent = ai::Opponent::new(mistake_probability);

    // Loop through each position first benchmarking how long the AI takes to
    // select a position, doing the actual move with the predetermined position
    // so next time through the loop there are less free moves remaining.
    for idx in 0..CATS_GAME_POSITION_SEQUENCE.len() - 1 {
        let moves_remaining = game.free_positions().count();

        c.bench_function(
            &format!("Perfect AI with {} moves remaining", moves_remaining),
            |b| b.iter(|| ai_opponent.get_move(&game)),
        );

        game.do_move(CATS_GAME_POSITION_SEQUENCE[idx]).unwrap();
    }
}

criterion_group!(game_bench, complete_game_benchmark);

criterion_group!(
    name = perfect_ai_bench; 
    config = Criterion::default().sample_size(10); 
    targets = perfect_ai_moves_benchmarks);

criterion_main!(game_bench, perfect_ai_bench);
