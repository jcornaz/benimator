use std::time::Duration;

use bevy::prelude::*;

use animism::*;

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

    let frame_duration = Duration::from_millis(100);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas,
            transform: Transform::from_scale(Vec3::splat(10.0)),
            ..Default::default()
        })
        .insert(SpriteSheetAnimation::from_frames(
            (0..5)
                .map(|index| Frame::new(index, frame_duration))
                .collect(),
        ));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
