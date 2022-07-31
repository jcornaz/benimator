use std::time::Duration;

use bevy_app_08::prelude::*;
use bevy_asset_08::prelude::*;
#[cfg(feature = "load-from-file")]
use bevy_asset_08::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset};
use bevy_ecs_08::{
    component::{SparseStorage, TableStorage},
    prelude::*,
    system::Resource,
};
use bevy_reflect_08::{TypeUuid, Uuid};
use bevy_sprite_08::prelude::*;
use bevy_time_08::prelude::*;

use crate::{Play, PlaySpeedMultiplier, SpriteSheetAnimation, SpriteSheetAnimationState};

impl Component for Play {
    type Storage = SparseStorage;
}

impl Component for PlaySpeedMultiplier {
    type Storage = SparseStorage;
}

impl Component for SpriteSheetAnimationState {
    type Storage = TableStorage;
}

trait TimeResource: Resource {
    fn delta_time(&self) -> Duration;
}

impl TimeResource for Time {
    fn delta_time(&self) -> Duration {
        self.delta()
    }
}

impl Plugin for crate::AnimationPlugin {
    fn build(&self, app: &mut App) {
        install::<Time>(app);
    }
}

fn install<T: TimeResource>(app: &mut App) {
    app.add_asset::<SpriteSheetAnimation>()
        .add_system_set_to_stage(CoreStage::PreUpdate, auto_insert_state())
        .add_system_to_stage(CoreStage::Update, animate::<T>);

    #[cfg(feature = "load-from-file")]
    app.init_asset_loader::<crate::animation::load::SpriteSheetAnimationLoader>();
}

/// Systems to automatically insert (and remove) the state component
fn auto_insert_state() -> SystemSet {
    SystemSet::new()
        .with_system(insert_state)
        .with_system(remove_state)
}

fn insert_state(
    mut commands: Commands<'_, '_>,
    query: Query<
        '_,
        '_,
        Entity,
        (
            With<Handle<SpriteSheetAnimation>>,
            Without<SpriteSheetAnimationState>,
        ),
    >,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(SpriteSheetAnimationState::default());
    }
}

fn remove_state(
    mut commands: Commands<'_, '_>,
    removed: RemovedComponents<'_, Handle<SpriteSheetAnimation>>,
) {
    for entity in removed.iter() {
        commands
            .entity(entity)
            .remove::<SpriteSheetAnimationState>();
    }
}

type AnimationSystemQuery<'a> = (
    Entity,
    &'a mut TextureAtlasSprite,
    &'a Handle<SpriteSheetAnimation>,
    &'a mut SpriteSheetAnimationState,
    Option<&'a PlaySpeedMultiplier>,
);

fn animate<T: TimeResource>(
    mut commands: Commands<'_, '_>,
    time: Res<'_, T>,
    animation_defs: Res<'_, Assets<SpriteSheetAnimation>>,
    mut animations: Query<'_, '_, AnimationSystemQuery<'_>, With<Play>>,
) {
    for (entity, mut sprite, animation_handle, mut state, speed_multiplier) in animations.iter_mut()
    {
        let animation = match animation_defs.get(animation_handle) {
            Some(anim) => anim,
            None => continue,
        };
        let delta_time = speed_multiplier
            .copied()
            .unwrap_or_default()
            .transform(time.delta_time());
        state.update(animation, delta_time);
        sprite.index = state.sprite_frame_index();
        if state.is_ended() {
            commands.entity(entity).remove::<Play>();
        }
    }
}

impl TypeUuid for SpriteSheetAnimation {
    const TYPE_UUID: Uuid = Uuid::from_bytes([
        0x63, 0x78, 0xe9, 0xc2, 0xec, 0xd1, 0x40, 0x29, 0x9c, 0xd5, 0x80, 0x1c, 0xaf, 0x68, 0x51,
        0x7c,
    ]);
}

#[cfg(feature = "load-from-file")]
impl AssetLoader for crate::animation::load::SpriteSheetAnimationLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext<'_>,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let custom_asset = self.load(
                load_context.path().extension().unwrap().to_str().unwrap(),
                bytes,
            )?;
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        self.supported_extensions()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use bevy_asset_08::AssetPlugin;

    use super::*;

    #[rstest]
    fn updates_sprite_atlas(mut app: App) {
        set_delta_time_per_update(&mut app, Duration::from_secs(1));
        let animation = add_animation(
            &mut app,
            SpriteSheetAnimation::from_range(0..=2, Duration::from_secs(1)),
        );
        let entity = spawn(&mut app, (TextureAtlasSprite::new(0), animation, Play));

        app.update();

        assert_eq!(
            app.world.get::<TextureAtlasSprite>(entity).unwrap().index,
            1
        );
    }

    #[rstest]
    fn does_not_update_without_play_component(mut app: App) {
        set_delta_time_per_update(&mut app, Duration::from_secs(1));
        let animation = add_animation(
            &mut app,
            SpriteSheetAnimation::from_range(0..=2, Duration::from_secs(1)),
        );
        let entity = spawn(&mut app, (TextureAtlasSprite::new(0), animation));

        app.update();

        assert_eq!(
            app.world.get::<TextureAtlasSprite>(entity).unwrap().index,
            0
        );
    }

    #[rstest]
    fn removes_play_at_end_of_animation(mut app: App) {
        set_delta_time_per_update(&mut app, Duration::from_secs(2));
        let animation = add_animation(
            &mut app,
            SpriteSheetAnimation::from_range(0..=1, Duration::from_secs(1)).once(),
        );
        let entity = spawn(&mut app, (TextureAtlasSprite::new(0), animation, Play));

        app.update();

        assert!(app.world.get::<Play>(entity).is_none());
    }

    #[rstest]
    fn speed_is_affected_by_playbackspeed_component(mut app: App) {
        set_delta_time_per_update(&mut app, Duration::from_secs(1));
        let animation = add_animation(
            &mut app,
            SpriteSheetAnimation::from_range(0..=3, Duration::from_secs(1)).once(),
        );
        let entity = spawn(
            &mut app,
            (
                TextureAtlasSprite::new(0),
                animation,
                Play,
                PlaySpeedMultiplier::from(2.0),
            ),
        );

        app.update();

        assert_eq!(
            app.world.get::<TextureAtlasSprite>(entity).unwrap().index,
            2
        );
    }

    #[cfg(all(feature = "load-from-file", feature = "yaml"))]
    #[rstest]
    fn load_asset_file(mut app: App) {
        let handle: Handle<SpriteSheetAnimation> = app
            .world
            .resource::<AssetServer>()
            .load("coin.animation.yml");

        app.update();
        let mut loops = 0;
        while !matches!(
            app.world.resource::<AssetServer>().get_load_state(&handle),
            bevy_asset_08::LoadState::Loaded
        ) {
            assert!(loops < 100);
            loops += 1;
            std::thread::sleep(Duration::from_millis(50));
            app.update();
        }
        assert_eq!(
            app.world.resource::<AssetServer>().get_load_state(&handle),
            bevy_asset_08::LoadState::Loaded
        );
        assert!(app
            .world
            .resource::<Assets<SpriteSheetAnimation>>()
            .get(&handle)
            .is_some());
    }

    #[fixture]
    fn app() -> App {
        let mut app = App::new();
        app.add_plugin(bevy::core::CorePlugin)
            .add_plugin(AssetPlugin);
        app.world.insert_resource(Duration::ZERO);
        install::<Duration>(&mut app);
        app
    }

    fn set_delta_time_per_update(app: &mut App, delta: Duration) {
        app.world.insert_resource(delta);
    }

    fn spawn(app: &mut App, bundle: impl Bundle) -> Entity {
        app.world.spawn().insert_bundle(bundle).id()
    }

    fn add_animation(
        app: &mut App,
        animation: SpriteSheetAnimation,
    ) -> Handle<SpriteSheetAnimation> {
        app.world
            .resource_mut::<Assets<SpriteSheetAnimation>>()
            .add(animation)
    }

    impl TimeResource for Duration {
        fn delta_time(&self) -> Duration {
            *self
        }
    }
}
