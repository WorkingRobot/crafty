use crafty::{Action, CraftContext, CraftOptions, Player, Recipe, SearchOptions, Simulator};
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use std::time::Duration;
use Action::*;

fn setup_sim(rng_seed: Option<u32>) -> (CraftContext, SearchOptions) {
    let recipe = Recipe {
        rlvl: 560,
        level: 90,
        progress_max: 3500,
        quality_max: 7200,
        durability_max: 80,
        progress_divider: 130,
        progress_modifier: 90,
        quality_divider: 115,
        quality_modifier: 80,
        is_expert: false,
        conditions_flag: 15,
    };
    let player = Player::new(90, 3304, 3374, 575);
    let craft_options = CraftOptions {
        max_steps: 15,
        ..Default::default()
    };
    let context = CraftContext::new(&player, &recipe, craft_options);
    let options = SearchOptions {
        iterations: 50_000,
        rng_seed,
        ..Default::default()
    };
    (context, options)
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("rotation", |b| {
        b.iter_batched(
            || setup_sim(None),
            |(context, _)| {
                Simulator::simulate(&context, black_box(ROTATION_1.to_vec()));
            },
            BatchSize::SmallInput,
        )
    });

    let mut group = c.benchmark_group("search");
    group
        .warm_up_time(Duration::new(5, 0))
        .measurement_time(Duration::new(30, 0));
    for seed in 0..5_u32 {
        group.bench_function(seed.to_string().as_str(), |b| {
            b.iter_batched(
                || setup_sim(Some(seed)),
                |(context, options)| {
                    Simulator::search_oneshot(&context, black_box(vec![]), options);
                },
                BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = criterion_benchmark
);
criterion_main!(benches);

const ROTATION_1: &[Action] = &[
    Reflect,
    Manipulation,
    PreparatoryTouch,
    WasteNotII,
    PreparatoryTouch,
    Innovation,
    PreparatoryTouch,
    PreparatoryTouch,
    GreatStrides,
    ByregotsBlessing,
    Veneration,
    GroundworkTraited,
    GroundworkTraited,
    GroundworkTraited,
];
