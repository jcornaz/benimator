use std::time::Duration;

use bevy::prelude::*;

use benimator::*;

#[bevy_main]
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(AnimationPlugin)
        .add_startup_system(spawn_camera.system())
        .add_startup_system(spawn_coin.system())
        .run();
}

fn spawn_coin(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>,
) {
    let texture = asset_server.load("coin.png");
    let texture_atlas = textures.add(TextureAtlas::from_grid(
        texture,
        Vec2::new(16.0, 16.0),
        5,
        1,
    ));

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas,
            transform: Transform::from_scale(Vec3::splat(10.0)),
            ..Default::default()
        })
        .insert(SpriteSheetAnimation::from_range(
            0..=4,
            Duration::from_millis(100),
        ))
        .insert(Play);
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
