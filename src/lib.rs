#![deny(future_incompatible)]
#![warn(nonstandard_style, rust_2018_idioms, missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]

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
//! 2. Insert the [`SpriteSheetAnimation`] component to the sprite sheets you want to animate
//!
//! ```
//! # use std::time::Duration;
//! # use bevy::prelude::*;
//! # use benimator::*;
//!
//! fn spawn(mut commands: Commands) {
//!     commands
//!         .spawn_bundle(SpriteSheetBundle {
//!             // TODO: Configure the sprite sheet
//!             ..Default::default()
//!         })
//!         // Insert the animation component
//!         .insert(SpriteSheetAnimation::from_range(
//!             0..=2,                               // Indices of the sprite atlas
//!             Duration::from_secs_f64(1.0 / 12.0), // Duration of each frame
//!         ))
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
//! # fn spawn(mut commands: Commands) {
//! commands
//!     .spawn_bundle(SpriteSheetBundle { ..Default::default() })
//!     .insert(
//!         SpriteSheetAnimation::from_range(0..=2, Duration::from_millis(100))
//!             .once() // <-- Runs the animation only once
//!     )
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

use std::ops::RangeInclusive;
use std::time::Duration;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;

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

/// Component to animate the `TextureAtlasSprite` of the same entity
///
/// See crate level documentation for usage
#[derive(Debug, Clone, Default, Reflect)]
#[reflect(Component)]
pub struct SpriteSheetAnimation {
    /// Frames
    pub frames: Vec<Frame>,
    /// Animation mode
    pub mode: AnimationMode,
}

/// Components that indicates the animation is playing
///
/// Insert the components to play the animation, and remove it to pause it.
///
/// If the animation mode is [`AnimationMode::Once`] this component is automatically removed at the end of the animation.
#[derive(Debug, Copy, Clone, Default, Reflect)]
#[reflect(Component)]
pub struct Play;

/// Animation mode (run once or repeat)
#[derive(Debug, Copy, Clone, Eq, PartialEq, Reflect)]
pub enum AnimationMode {
    /// Runs the animation once and then stop playing
    Once,

    /// Repeat the animation forever
    Repeat,
}

/// A single animation frame
#[derive(Debug, Copy, Clone, Default, Reflect)]
pub struct Frame {
    /// Index in the sprite atlas
    pub index: u32,
    /// How long should the frame be displayed
    pub duration: Duration,
}

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.register_type::<SpriteSheetAnimation>()
            .add_system_set(state::update_systems())
            .add_system_to_stage(CoreStage::PostUpdate, state::post_update_system());
    }
}

impl SpriteSheetAnimation {
    /// Create a new animation from frames
    #[must_use]
    pub fn from_frames(frames: Vec<Frame>) -> Self {
        Self {
            frames,
            mode: AnimationMode::default(),
        }
    }

    /// Create a new animation from index-range, using the same frame duration for each frame.
    ///
    /// For more granular configuration, see [`from_frames`](SpriteSheetAnimation::from_frames)
    #[must_use]
    pub fn from_range(index_range: RangeInclusive<u32>, frame_duration: Duration) -> Self {
        Self::from_frames(
            index_range
                .map(|index| Frame::new(index, frame_duration))
                .collect(),
        )
    }

    /// Set the animation mode to [`AnimationMode::Once`]
    #[must_use]
    pub fn once(mut self) -> Self {
        self.mode = AnimationMode::Once;
        self
    }

    /// Set the animation mode to [`AnimationMode::Repeat`]
    #[must_use]
    pub fn repeat(mut self) -> Self {
        self.mode = AnimationMode::Repeat;
        self
    }

    fn has_frames(&self) -> bool {
        !self.frames.is_empty()
    }
}

impl Default for AnimationMode {
    #[inline]
    fn default() -> Self {
        Self::Repeat
    }
}

impl Frame {
    /// Create a new animation frame
    #[inline]
    #[must_use]
    pub fn new(index: u32, duration: Duration) -> Self {
        Self { index, duration }
    }
}
