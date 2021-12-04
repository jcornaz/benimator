use std::time::Duration;

use bevy::prelude::*;

use benimator::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(AnimationPlugin)
        .add_startup_system(spawn_animated_coin.system())
        .add_startup_system(spawn_camera.system())
        .add_system_to_stage(CoreStage::Last, removal_detection.system())
        .run();
}

fn spawn_animated_coin(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>,
    mut animations: ResMut<Assets<SpriteSheetAnimation>>,
) {
    let animation =
        animations.add(SpriteSheetAnimation::from_range(0..=4, Duration::from_millis(100)).once());

    commands
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
        .insert(animation)
        .insert(Play);
}

fn removal_detection(removals: RemovedComponents<Play>) {
    for entity in removals.iter() {
        println!("Animation stopped for: {:?}", entity);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
