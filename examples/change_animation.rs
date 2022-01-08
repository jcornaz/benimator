use core::ops::Deref;
use std::time::Duration;

use bevy::prelude::*;

use benimator::*;

// Create a resource to store handles of the animations
#[derive(Default)]
struct Animations {
    slow: Handle<SpriteSheetAnimation>,
    fast: Handle<SpriteSheetAnimation>,
}

fn main() {
    App::new()
        .init_resource::<Animations>()
        .add_plugins(DefaultPlugins)
        .add_plugin(AnimationPlugin)
        .add_startup_system_to_stage(StartupStage::PreStartup, create_animations.system())
        .add_startup_system(spawn_animated_coin.system())
        .add_startup_system(spawn_camera.system())
        .add_system(change_animation.system())
        .run();
}

fn create_animations(
    mut handles: ResMut<Animations>,
    mut assets: ResMut<Assets<SpriteSheetAnimation>>,
) {
    handles.fast = assets.add(SpriteSheetAnimation::from_range(
        0..=4,
        Duration::from_millis(100),
    ));
    handles.slow = assets.add(SpriteSheetAnimation::from_range(
        0..=4,
        Duration::from_millis(250),
    ));
}

fn spawn_animated_coin(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>,
    animations: Res<Animations>,
) {
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
        .insert(animations.fast.clone())
        .insert(Play)
        // Add timer, counting down the time before the animation is changed
        .insert(Timer::from_seconds(5.0, true));
}

fn change_animation(
    time: Res<Time>,
    animations: Res<Animations>,
    mut query: Query<(&mut Timer, &mut Handle<SpriteSheetAnimation>)>,
) {
    let (mut timer, mut animation) = query.single_mut();
    if timer.tick(time.delta()).finished() {
        if animation.deref() == &animations.fast {
            *animation = animations.slow.clone();
        } else {
            *animation = animations.fast.clone();
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
