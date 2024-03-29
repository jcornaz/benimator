use bevy::prelude::*;

use benimator::FrameRate;

// Create the animation component
// Note: you may make the animation an asset instead of a component
#[derive(Component, Deref)]
struct Animation(benimator::Animation);

// Create the player component
#[derive(Default, Component, Deref, DerefMut)]
struct AnimationState(benimator::State);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, spawn)
        .add_systems(Update, animate)
        .run();
}

fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>,
) {
    // Don't forget the camera ;-)
    commands.spawn(Camera2dBundle::default());

    // Create an animation
    let animation = Animation(benimator::Animation::from_indices(
        0..=4,
        FrameRate::from_fps(12.0),
    ));

    commands
        // Spawn a bevy sprite-sheet
        .spawn(SpriteSheetBundle {
            texture_atlas: textures.add(TextureAtlas::from_grid(
                asset_server.load("coin.png"),
                Vec2::new(16.0, 16.0),
                5,
                1,
                None,
                None,
            )),
            transform: Transform::from_scale(Vec3::splat(10.0)),
            ..Default::default()
        })
        // Insert the animation
        .insert(animation)
        // Insert the state
        .insert(AnimationState::default());
}

fn animate(
    time: Res<Time>,
    mut query: Query<(&mut AnimationState, &mut TextureAtlasSprite, &Animation)>,
) {
    for (mut player, mut texture, animation) in query.iter_mut() {
        // Update the state
        player.update(animation, time.delta());

        // Update the texture atlas
        texture.index = player.frame_index();
    }
}
