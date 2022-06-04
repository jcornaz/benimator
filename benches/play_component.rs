use bevy::prelude::*;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use std::time::Duration;

use benimator::{Play, SpriteSheetAnimation, SpriteSheetAnimationState};

fn create_app<B: Bundle + Default>(size: usize) -> App {
    let mut app = App::new();
    app.world.spawn_batch((0..size).map(|_| B::default()));
    app
}

#[derive(Default, Bundle)]
struct PlayingBundle {
    play: Play,
    state: SpriteSheetAnimationState,
    sprite: TextureAtlasSprite,
    handle: Handle<SpriteSheetAnimation>,
}

#[derive(Default, Bundle)]
struct PausedBundle {
    state: SpriteSheetAnimationState,
    sprite: TextureAtlasSprite,
    handle: Handle<SpriteSheetAnimation>,
}

pub fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("play perfs");

    group
        .significance_level(0.02)
        .confidence_level(0.98)
        .measurement_time(Duration::from_secs(20));

    for size in [0, 5_000, 10_000] {
        group
            .bench_with_input(
                BenchmarkId::new("update when playing", size),
                &size,
                |b, size| {
                    let mut app = create_app::<PlayingBundle>(*size);
                    b.iter(|| app.update())
                },
            )
            .bench_with_input(
                BenchmarkId::new("update when paused", size),
                &size,
                |b, size| {
                    let mut app = create_app::<PausedBundle>(*size);
                    b.iter(|| app.update())
                },
            )
            .bench_with_input(
                BenchmarkId::new("remove play component", size),
                &size,
                |b, size| {
                    let setup = || {
                        let mut app = create_app::<PlayingBundle>(*size);
                        app.update(); // <-- Make sure, the world is properly initialized
                        let entities: Vec<Entity> = app
                            .world
                            .query_filtered::<Entity, With<Play>>()
                            .iter(&app.world)
                            .collect();
                        (app, entities)
                    };
                    let routine = |(mut app, entities): (App, Vec<Entity>)| {
                        for entity in entities {
                            app.world.entity_mut(entity).remove::<Play>();
                        }
                    };
                    b.iter_batched(setup, routine, BatchSize::LargeInput)
                },
            )
            .bench_with_input(
                BenchmarkId::new("insert play component", size),
                &size,
                |b, size| {
                    let setup = || {
                        let mut app = create_app::<PausedBundle>(*size);
                        app.update(); // <-- Make sure, the world is properly initialized
                        let entities: Vec<Entity> = app
                            .world
                            .query_filtered::<Entity, Without<Play>>()
                            .iter(&app.world)
                            .collect();
                        (app, entities)
                    };
                    let routine = |(mut app, entities): (App, Vec<Entity>)| {
                        for entity in entities {
                            app.world.entity_mut(entity).insert(Play);
                        }
                    };
                    b.iter_batched(setup, routine, BatchSize::LargeInput)
                },
            );
    }

    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
