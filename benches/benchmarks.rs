#[macro_use]
extern crate criterion;

use criterion::Criterion;

use open_ttt_lib::ai;
use open_ttt_lib::game;

fn flawless_ai_moves_benchmarks(c: &mut Criterion) {
    let game = game::Game::new();
    let mistake_probability = 0.0;
    let ai_opponent = ai::Opponent::new(mistake_probability);

    c.bench_function("New game with flawless AI", |b| {
        b.iter(|| ai_opponent.get_move(&game))
    });
}

fn build_benchmark_configuration() -> Criterion {
    // At the moment the AI takes a LONG time to update so reduce
    // the sample size so the test completes in this decade.
    Criterion::default().sample_size(10)
}

criterion_group!(
    name = benches; 
    config = build_benchmark_configuration(); 
    targets = flawless_ai_moves_benchmarks);

criterion_main!(benches);
