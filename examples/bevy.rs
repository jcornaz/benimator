use std::time::Duration;

use bevy::{prelude::*, render::texture::ImageSettings};

// Create tha animation component
// Note: you may make the animation an asset instead of a component
#[derive(Component, Deref)]
struct Animation(benimator::Animation);

// Create tha player component
#[derive(Default, Component, Deref, DerefMut)]
struct Player(benimator::State);

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn)
        .add_system(animate)
        .run();
}

fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>,
) {
    // Don't forget the camera ;-)
    commands.spawn_bundle(Camera2dBundle::default());

    // Create an animation
    const FPS: f64 = 12.0;
    let frame_duration: Duration = Duration::from_secs(1).div_f64(FPS);
    let animation = Animation(benimator::Animation::from_range(0..=4, frame_duration));

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
        // Insert the animation
        .insert(animation)
        // Insert the player
        .insert(Player::default());
}

fn animate(time: Res<Time>, mut query: Query<(&mut Player, &mut TextureAtlasSprite, &Animation)>) {
    for (mut player, mut texture, animation) in query.iter_mut() {
        player.update(animation, time.delta());
        texture.index = player.sprite_frame_index();
    }
}
