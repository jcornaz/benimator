#![warn(
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    missing_docs,
    clippy::pedantic
)]
#![allow(clippy::needless_pass_by_value, clippy::module_name_repetitions)]
#![cfg_attr(nightly, feature(doc_auto_cfg))]

//! A sprite-sheet animation plugin for [bevy](https://bevyengine.org)
//!
//! ## Usage
//!
//! 1. Add the [`AnimationPlugin`] plugin
//!
//! ```no_run
//! use std::time::Duration;
//! use bevy::prelude::*;
//! use benimator::*;
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         .add_plugin(AnimationPlugin::default()) // <-- Enable sprite-sheet animations
//!         .add_startup_system(spawn.system())
//!         // ...
//!         .run()
//! }
//!
//! fn spawn() { /* ... */ }
//! ```
//!
//! 2. Create a [`SpriteSheetAnimation`] and insert the asset handle to the sprite sheet entity you want to animate
//!
//! ```
//! # use std::time::Duration;
//! # use bevy::prelude::*;
//! # use benimator::*;
//!
//! fn spawn(mut commands: Commands, mut animations: ResMut<Assets<SpriteSheetAnimation>>) {
//!
//!     // Create an animation
//!     let animation_handle = animations.add(SpriteSheetAnimation::from_range(
//!         0..=2,                               // Indices of the sprite atlas
//!         Duration::from_secs_f64(1.0 / 12.0), // Duration of each frame
//!     ));
//!
//!     commands
//!         .spawn_bundle(SpriteSheetBundle {
//!             // TODO: Configure the sprite sheet
//!             ..Default::default()
//!         })
//!         // Insert the asset handle of the animation
//!         .insert(animation_handle)
//!         // Start the animation immediately
//!         .insert(Play);
//! }
//! ```
//!
//! ## Run the animation only once
//!
//! By default the animation loops forever. But it is possible to configure it differently:
//!
//! ```
//! # use std::time::Duration;
//! # use benimator::*;
//! SpriteSheetAnimation::from_range(0..=2, Duration::from_millis(100))
//!      .once(); // <-- Runs the animation only once
//! ```
//!
//! Note that, for animations that run once, the `Play` component is automatically removed when the animation is done.
//! So you can use the `RemovedComponents<Play>` system parameter to execute logic at the end of the animation.
//!
//! ## Play/Pause
//!
//! Animations proceed only if the [`Play`] component is present in the entity.
//!
//! To pause or resume an animation, simply remove/insert the [`Play`] component.
//!
//! ## Fine-grained frame-duration
//!
//! For a more precise configuration, it is possible to define the duration of each frame:
//!
//! ```
//! # use benimator::*;
//! # use std::time::Duration;
//! SpriteSheetAnimation::from_frames(vec![
//!     Frame::new(0, Duration::from_millis(120)),
//!     Frame::new(1, Duration::from_millis(100)),
//!     Frame::new(2, Duration::from_millis(80)),
//! ]);
//! ```
//!
//! ## Reset animation
//!
//! For each entity with a [`SpriteSheetAnimation`], a [`SpriteSheetAnimationState`] component is automatically inserted.
//! It can be used to reset the animation state by calling [`SpriteSheetAnimationState::reset`]
//!
//! ```
//! # use bevy::prelude::*;
//! # use benimator::SpriteSheetAnimationState;
//!
//! fn restart_anim_from_start(mut query: Query<&mut SpriteSheetAnimationState>) {
//!   for mut state in query.iter_mut() {
//!     state.reset();
//!   }
//! }
//! ```
//!
//! ## Load animation from file **(Unstable)**
//!
//! By enabling the cargo feature: `unstable-load-from-file` you can write the animation in an asset file.
//!
//! First, create an asset file with the extension `.animation.yml`:
//! ```yaml
//! # The mode can be one of: 'Once', 'Repeat', 'PingPong'
//! # or 'RepeatFrom: n' (where 'n' is the frame-index to repeat from)
//! # The default is 'Repeat'
//! mode: PingPong
//! frame_duration: 100  # duration of the frame in milliseconds
//! frames: [0, 1, 2]  # Sequence of frame index
//! ```
//!
//! You can specify different duration on each frame if you need to:
//! ```yaml
//! frames:
//!   - index: 0
//!     duration: 100
//!   - index: 1
//!     duration: 200
//! ```
//!
//! And then load it with bevy's `AssetServer`:
//! ```
//! # use bevy::prelude::*;
//! # use benimator::*;
//! # fn load(mut commands: Commands, asset_server: Res<AssetServer>) {
//! let handle: Handle<SpriteSheetAnimation> = asset_server.load("player_run.animation.yml");
//! # }
//! ```
//!
//! It is also possible to use `ron` instead of `yaml`.
//!
//! For more info on the format see: [`SpriteSheetAnimation::from_yaml_str`] and [`SpriteSheetAnimation::from_ron_str`].

#[cfg(test)]
#[macro_use]
extern crate rstest;

use bevy_app::prelude::*;
use bevy_asset::AddAsset;
use bevy_ecs::component::SparseStorage;
use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;

pub use animation::{Frame, SpriteSheetAnimation};
pub use state::SpriteSheetAnimationState;

#[allow(deprecated)]
pub use animation::AnimationMode;

mod animation;
mod state;

/// Plugin to enable sprite-sheet animation
///
/// See crate level documentation for usage
#[non_exhaustive]
#[derive(Default)]
pub struct AnimationPlugin;

/// Labels of systems that run during the post-update stage
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, SystemLabel)]
pub enum AnimationPostUpdateSystem {
    /// System that update the sprite atlas textures
    Animate,
}

/// Components that indicates the animation is playing
///
/// Insert the components to play the animation, and remove it to pause it.
///
/// If the animation mode is [`AnimationMode::Once`] this component is automatically removed at the end of the animation.
#[derive(Debug, Copy, Clone, Default, Reflect)]
#[reflect(Component)]
pub struct Play;

impl Component for Play {
    type Storage = SparseStorage;
}

/// Component that, when applied, can change the playback's rate of the animation.
///
/// Receives a f64 multiplier which will alter the delta, speeding up or slowing down to the desired playback rate.
#[derive(Debug, Copy, Component, Clone, Reflect)]
#[reflect(Component)]
pub struct PlaySpeedMultiplier(f64);

impl PlaySpeedMultiplier {
    /// Creates a new `PlaySpeedMultiplier` Component with the multiplier set as desired
    #[must_use]
    pub fn new(multiplier: f64) -> Self {
        Self(multiplier)
    }
}

impl Default for PlaySpeedMultiplier {
    fn default() -> Self {
        Self(1.0)
    }
}

impl From<f64> for PlaySpeedMultiplier {
    fn from(mult: f64) -> Self {
        Self(mult)
    }
}

impl From<f32> for PlaySpeedMultiplier {
    fn from(mult: f32) -> Self {
        Self(mult.into())
    }
}

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<SpriteSheetAnimation>()
            .add_system_set_to_stage(CoreStage::PreUpdate, state::maintenance_systems())
            .add_system_set_to_stage(CoreStage::Update, state::post_update_systems());

        #[cfg(feature = "unstable-load-from-file")]
        app.init_asset_loader::<animation::load::SpriteSheetAnimationLoader>();
    }
}
