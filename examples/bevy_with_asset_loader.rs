use bevy::{
    asset::{AssetLoader, BoxedFuture, Error, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    render::texture::ImageSettings,
};

// Create tha animation asset
#[derive(TypeUuid, Deref)]
#[uuid = "ae6a74db-f6fa-43c4-ac16-01d13b50e4c6"]
struct Animation(benimator::Animation);

// Create tha player component
#[derive(Default, Component, Deref, DerefMut)]
struct Player(benimator::State);

// Create (and implement) the asset loader
#[derive(Default)]
struct AnimationLoader;

impl AssetLoader for AnimationLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, anyhow::Result<(), Error>> {
        Box::pin(async move {
            // `Animation` implements `serde::Deserialize`,
            // so you may use any serde-compatible deserialization library
            let animation: Animation = Animation(serde_yaml::from_slice(bytes)?);
            load_context.set_default_asset(LoadedAsset::new(animation));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["animation.yml"]
    }
}

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_asset::<Animation>()
        .init_asset_loader::<AnimationLoader>()
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

    // Load an animation
    let animation_handle: Handle<Animation> = asset_server.load("coin.animation.yml");

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
        // Insert the player
        .insert(Player::default());
}

fn animate(
    time: Res<Time>,
    animations: Res<Assets<Animation>>,
    mut query: Query<(&mut Player, &mut TextureAtlasSprite, &Handle<Animation>)>,
) {
    for (mut player, mut texture, handle) in query.iter_mut() {
        let animation = match animations.get(handle) {
            Some(anim) => anim,
            None => continue,
        };
        player.update(animation, time.delta());
        texture.index = player.sprite_frame_index();
    }
}
