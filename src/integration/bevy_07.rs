use std::ops::DerefMut;

use crate::{
    state::SpriteState, Play, PlaySpeedMultiplier, SpriteSheetAnimation, SpriteSheetAnimationState,
};
use bevy_app_07::prelude::*;
use bevy_asset::prelude::*;
use bevy_core::prelude::*;
use bevy_ecs::prelude::*;
use bevy_sprite_07::prelude::*;

impl Plugin for crate::AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<crate::SpriteSheetAnimation>()
            .add_system_set_to_stage(CoreStage::PreUpdate, auto_insert_state())
            .add_system_set_to_stage(CoreStage::Update, animation_systems::<TextureAtlasSprite>());

        #[cfg(feature = "unstable-load-from-file")]
        app.init_asset_loader::<crate::animation::load::SpriteSheetAnimationLoader>();
    }
}

impl SpriteState for TextureAtlasSprite {
    fn set_current_index(&mut self, index: usize) {
        self.index = index;
    }
}

/// Systems to automatically insert (and remove) the state component
fn auto_insert_state() -> SystemSet {
    SystemSet::new()
        .with_system(insert_state)
        .with_system(remove_state)
}

/// Animation systems
///
/// Generic over the type of sprite atlas comonent.
///
/// # Required resources
///
/// * `bevy_asset::assets::Assets<benimator::SpriteSheetAnimation>`
/// * `bevy_core::time::Time`
fn animation_systems<T: SpriteState + Component>() -> SystemSet {
    SystemSet::new().with_system(animate::<T>)
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

type AnimationSystemQuery<'a, T> = (
    Entity,
    &'a mut T,
    &'a Handle<SpriteSheetAnimation>,
    &'a mut SpriteSheetAnimationState,
    Option<&'a PlaySpeedMultiplier>,
);

fn animate<T: SpriteState + Component>(
    mut commands: Commands<'_, '_>,
    time: Res<'_, Time>,
    animation_defs: Res<'_, Assets<SpriteSheetAnimation>>,
    mut animations: Query<'_, '_, AnimationSystemQuery<'_, T>, With<Play>>,
) {
    for (entity, mut sprite, animation, mut state, speed_multiplier) in
        animations.iter_mut().filter_map(
            |(entity, sprite, anim_handle, state, optional_speed_multiplier)| {
                animation_defs
                    .get(anim_handle)
                    .filter(|anim| anim.has_frames())
                    .map(|anim| (entity, sprite, anim, state, optional_speed_multiplier))
            },
        )
    {
        let delta = speed_multiplier
            .copied()
            .unwrap_or_default()
            .transform(time.delta());

        if state.update(&mut sprite, animation, delta) {
            commands.entity(entity).remove::<Play>();
        }
    }
}

impl<'w, T: SpriteState> SpriteState for Mut<'w, T> {
    fn set_current_index(&mut self, index: usize) {
        self.deref_mut().set_current_index(index);
    }
}
