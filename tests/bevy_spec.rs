#![cfg(feature = "bevy-07")]

use std::time::Duration;

use bevy::asset::AssetPlugin;
use bevy::prelude::*;
use bevy_core::CorePlugin;

use benimator::*;

#[test]
fn plugin_does_not_crash() {
    let mut app = App::new();

    app.add_plugin(CorePlugin)
        .add_plugin(AssetPlugin)
        .add_plugin(AnimationPlugin::default());

    let animation = app
        .world
        .get_resource_mut::<Assets<SpriteSheetAnimation>>()
        .unwrap()
        .add(SpriteSheetAnimation::from_range(
            0..=2,
            Duration::from_nanos(1),
        ));

    app.world
        .spawn()
        .insert_bundle((TextureAtlasSprite::new(0), animation, Play));

    app.update();
    app.update();
}
