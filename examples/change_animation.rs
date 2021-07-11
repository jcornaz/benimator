use std::time::Duration;

use bevy::prelude::*;

use benimator::*;

static FAST: Duration = Duration::from_millis(100);
static SLOW: Duration = Duration::from_millis(250);

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(AnimationPlugin) // <-- Add the plugin
        .add_startup_system(spawn.system())
        .add_system(animation.system())
        .run();
}

fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>,
) {
    // Don't forget the camera ;-)
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

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
        // Insert the animation component
        // Here we use an index-range (from 0 to 4) where each frame has the same duration
        .insert(SpriteSheetAnimation::from_range(0..=4, FAST))
        // Start the animation immediately. Remove this component in order to pause the animation.
        .insert(Play)
        // Add component and timer to query
        .insert(Animation::Fast)
        .insert(Timer::from_seconds(5.0, true));
}

enum Animation {
    Fast,
    Slow,
}

fn animation(
    time: Res<Time>,
    mut query: Query<(&mut Timer, &mut Animation, &mut SpriteSheetAnimation)>,
) {
    if let Ok((mut timer, mut animation, mut sprite_sheet_animation)) = query.single_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            match *animation {
                Animation::Fast => {
                    *animation = Animation::Slow;
                    *sprite_sheet_animation = SpriteSheetAnimation::from_range(0..=4, SLOW);
                }
                Animation::Slow => {
                    *animation = Animation::Fast;
                    *sprite_sheet_animation = SpriteSheetAnimation::from_range(0..=4, FAST);
                }
            }
        }
    }
}
