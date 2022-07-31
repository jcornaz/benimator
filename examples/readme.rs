use std::time::Duration;

use bevy::{prelude::*, render::texture::ImageSettings};

use benimator::*;

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(AnimationPlugin::default()) // <-- Add the plugin
        .add_startup_system(spawn)
        .run();
}

fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>,
    mut animations: ResMut<Assets<SpriteSheetAnimation>>,
) {
    // Don't forget the camera ;-)
    commands.spawn_bundle(Camera2dBundle::default());

    // Create an animation
    // Here we use an index-range (from 0 to 4) where each frame has the same duration
    let animation_handle = animations.add(SpriteSheetAnimation::from_range(
        0..=4,
        Duration::from_millis(100),
    ));

    commands
        // Spawn a bevy sprite-sheet
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.add(TextureAtlas::from_grid(
                asset_server.load("coin.png"),
                Vec2::new(16.0, 16.0),
                5,
                1,
            )),
            transform: Transform::from_scale(Vec3::splat(10.0)),
            ..Default::default()
        })
        // Insert the asset handle of the animation
        .insert(animation_handle)
        // Start the animation immediately. Remove this component in order to pause the animation.
        .insert(Play);
}
