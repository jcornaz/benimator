#[macro_use]
extern crate rstest;

use std::time::Duration;

use bevy::prelude::*;
use bevy_core::CorePlugin;

use animism::*;

#[rstest]
fn repeated_animation(mut app: App) {
    let entity = app
        .world
        .spawn()
        .insert_bundle((
            TextureAtlasSprite::new(0),
            SpriteSheetAnimation::from_range(0..=2, Duration::from_nanos(0)),
        ))
        .id();

    app.update();
    assert_eq!(
        app.world.get::<TextureAtlasSprite>(entity).unwrap().index,
        1
    );

    app.update();
    assert_eq!(
        app.world.get::<TextureAtlasSprite>(entity).unwrap().index,
        2
    );

    app.update();
    assert_eq!(
        app.world.get::<TextureAtlasSprite>(entity).unwrap().index,
        0
    );
}

#[fixture]
fn app() -> App {
    let mut builder = App::build();

    builder.add_plugin(CorePlugin).add_plugin(AnimationPlugin);

    builder.app
}
