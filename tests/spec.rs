#[macro_use]
extern crate rstest;

use std::time::Duration;

use bevy::asset::AssetPlugin;
use bevy::prelude::*;
use bevy_core::CorePlugin;

use benimator::*;

#[rstest]
fn repeated(mut app: App) {
    let animation = app
        .world
        .get_resource_mut::<Assets<SpriteSheetAnimation>>()
        .unwrap()
        .add(SpriteSheetAnimation::from_range(
            0..=2,
            Duration::from_nanos(0),
        ));

    let entity = app
        .world
        .spawn()
        .insert_bundle((TextureAtlasSprite::new(0), animation, Play))
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

#[rstest]
fn run_once(mut app: App) {
    let animation = app
        .world
        .get_resource_mut::<Assets<SpriteSheetAnimation>>()
        .unwrap()
        .add(SpriteSheetAnimation::from_range(0..=2, Duration::from_nanos(0)).once());

    let entity = app
        .world
        .spawn()
        .insert_bundle((TextureAtlasSprite::new(0), animation, Play))
        .id();

    app.update();
    assert!(app.world.get::<Play>(entity).is_some());
    assert_eq!(
        app.world.get::<TextureAtlasSprite>(entity).unwrap().index,
        1
    );

    app.update();
    assert!(app.world.get::<Play>(entity).is_some());
    assert_eq!(
        app.world.get::<TextureAtlasSprite>(entity).unwrap().index,
        2
    );

    app.update();
    assert!(app.world.get::<Play>(entity).is_none());
    assert_eq!(
        app.world.get::<TextureAtlasSprite>(entity).unwrap().index,
        2
    );
}

#[rstest]
fn run_ping_pong(mut app: App) {
    let animation = app
        .world
        .get_resource_mut::<Assets<SpriteSheetAnimation>>()
        .unwrap()
        .add(SpriteSheetAnimation::from_range(0..=2, Duration::from_nanos(0)).ping_pong());

    let entity = app
        .world
        .spawn()
        .insert_bundle((TextureAtlasSprite::new(0), animation, Play))
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
        1
    );

    app.update();
    assert_eq!(
        app.world.get::<TextureAtlasSprite>(entity).unwrap().index,
        0
    );

    app.update();
    assert_eq!(
        app.world.get::<TextureAtlasSprite>(entity).unwrap().index,
        1
    );
}

#[fixture]
fn app() -> App {
    let mut app = App::new();

    app.add_plugin(CorePlugin)
        .add_plugin(AssetPlugin)
        .add_plugin(AnimationPlugin);

    app
}
