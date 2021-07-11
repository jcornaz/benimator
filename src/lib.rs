#![warn(
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    missing_docs,
    clippy::pedantic
)]
#![allow(clippy::needless_pass_by_value, clippy::module_name_repetitions)]

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
//!     App::build()
//!         .add_plugins(DefaultPlugins)
//!         .add_plugin(AnimationPlugin) // <-- Enable sprite-sheet animations
//!         .add_startup_system(spawn.system())
//!         // ...
//!         .run()
//! }
//!
//! fn spawn() { /* ... */ }
//! ```
//!
//! 2. Create [`SpriteSheetAnimation`] and insert the asset handle to the sprite sheet entity you want to animate
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
//! # use bevy::prelude::*;
//! # use benimator::*;
//! # fn spawn(mut commands: Commands, mut animations: ResMut<Assets<SpriteSheetAnimation>>) {
//! commands
//!     .spawn_bundle(SpriteSheetBundle { ..Default::default() })
//!     .insert(animations.add(
//!         SpriteSheetAnimation::from_range(0..=2, Duration::from_millis(100))
//!             .once() // <-- Runs the animation only once
//!     ))
//!     .insert(Play); // <-- This component will be automatically removed once the animation is finished
//! # }
//! ```
//!
//! ## Play/Pause
//!
//! Animations proceed only if the [`Play`] component is in the entity.
//!
//! To pause or resume an animation, simply remove/insert the [`Play`] component.
//!
//! ## Fine-grained frame-duration
//!
//! For more precise configuration, it is possible to define the duration of each frame:
//!
//! ```rust
//! # use benimator::*;
//! # use std::time::Duration;
//! SpriteSheetAnimation::from_frames(vec![
//!     Frame::new(0, Duration::from_millis(120)),
//!     Frame::new(1, Duration::from_millis(100)),
//!     Frame::new(2, Duration::from_millis(80)),
//! ]);
//! ```
//!
#[cfg(test)]
#[macro_use]
extern crate rstest;

use bevy_app::prelude::*;
use bevy_asset::AddAsset;
use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;

pub use animation::{AnimationMode, Frame, SpriteSheetAnimation};

mod animation;
mod state;

/// Plugin to enable sprite-sheet animation
///
/// See crate level documentation for usage
#[derive(Default)]
pub struct AnimationPlugin;

/// Labels of systems that run during the post-update stage
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

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<SpriteSheetAnimation>()
            .add_system_set(state::update_systems())
            .add_system_to_stage(CoreStage::PostUpdate, state::post_update_system());
    }
}
