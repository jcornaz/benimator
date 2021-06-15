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
//! use animism::*;
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
//! ```rust
//! # use std::time::Duration;
//! # use bevy::prelude::*;
//! # use animism::*;
//!
//! fn spawn(mut commands: Commands) {
//!     commands
//!         .spawn_bundle(SpriteSheetBundle {
//!             // TODO: Configure your sprite sheet
//!             ..Default::default()
//!         })
//!         // Insert the animation component
//!         .insert(SpriteSheetAnimation::from_range(
//!             0..=2,                               // Indices of the sprite atlas
//!             Duration::from_secs_f64(1.0 / 12.0), // Duration of each frame
//!         ));
//! }
//! ```
//!
//! ## Fine-grained frame-duration
//!
//! For more precise configuration, it is possible to define the duration of each frame:
//!
//! ```rust
//! # use animism::*;
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

use std::ops::{DerefMut, RangeInclusive};
use std::time::Duration;

use bevy_app::prelude::*;
use bevy_core::Time;
use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;
use bevy_sprite::TextureAtlasSprite;

/// Plugin to enable sprite-sheet animation
///
/// See crate level documentation for usage
#[derive(Default)]
pub struct AnimationPlugin;

/// Labels of systems that run during the update stage
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, SystemLabel)]
pub enum AnimationUpdateSystem {
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
    #[reflect(ignore)]
    current_frame: usize,
    #[reflect(ignore)]
    elapsed_in_frame: Duration,
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
            .add_system(animate.system().label(AnimationUpdateSystem::Animate));
    }
}

impl SpriteSheetAnimation {
    /// Create a new animation from frames
    #[must_use]
    pub fn from_frames(frames: Vec<Frame>) -> Self {
        Self {
            frames,
            current_frame: 0,
            elapsed_in_frame: Duration::from_nanos(0),
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

    #[inline]
    fn can_update(&self) -> bool {
        !self.frames.is_empty()
    }

    fn update(&mut self, mut sprite: impl DerefMut<Target = TextureAtlasSprite>, delta: Duration) {
        debug_assert!(self.can_update());

        let mut frame = self.frames[self.current_frame % self.frames.len()];

        self.elapsed_in_frame += delta;
        if self.elapsed_in_frame >= frame.duration {
            self.elapsed_in_frame -= frame.duration;

            self.current_frame += 1;
            if self.current_frame >= self.frames.len() {
                self.current_frame = 0;
            }

            frame = self.frames[self.current_frame];
            sprite.index = frame.index;
        } else if sprite.index != frame.index {
            sprite.index = frame.index;
        }
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

fn animate(
    time: Res<'_, Time>,
    mut animations: Query<'_, (&mut TextureAtlasSprite, &mut SpriteSheetAnimation)>,
) {
    for (sprite, mut animation) in animations.iter_mut().filter(|(_, anim)| anim.can_update()) {
        animation.update(sprite, time.delta());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[fixture]
    fn sprite() -> TextureAtlasSprite {
        TextureAtlasSprite::new(0)
    }

    #[fixture]
    fn sprite_at_second_frame() -> TextureAtlasSprite {
        TextureAtlasSprite::new(1)
    }

    #[fixture]
    fn frame_duration() -> Duration {
        Duration::from_secs(1)
    }

    #[fixture]
    fn smaller_duration(frame_duration: Duration) -> Duration {
        frame_duration - Duration::from_millis(1)
    }

    mod on_first_frame {
        use super::*;

        #[fixture]
        fn animation(frame_duration: Duration) -> SpriteSheetAnimation {
            SpriteSheetAnimation::from_range(0..=1, frame_duration)
        }

        #[rstest]
        fn nothing_happens_if_not_enough_time_has_elapsed_and_index_is_already_set(
            mut animation: SpriteSheetAnimation,
            mut sprite: TextureAtlasSprite,
            smaller_duration: Duration,
        ) {
            animation.update(&mut sprite, smaller_duration);
            assert_eq!(sprite.index, 0);
        }

        #[rstest]
        fn updates_index_if_not_on_expected_index(
            mut animation: SpriteSheetAnimation,
            mut sprite_at_second_frame: TextureAtlasSprite,
            smaller_duration: Duration,
        ) {
            animation.update(&mut sprite_at_second_frame, smaller_duration);
            assert_eq!(sprite_at_second_frame.index, 0);
        }

        #[rstest]
        fn updates_index_if_enough_time_has_elapsed(
            mut animation: SpriteSheetAnimation,
            mut sprite: TextureAtlasSprite,
            frame_duration: Duration,
        ) {
            animation.update(&mut sprite, frame_duration);
            assert_eq!(sprite.index, 1);
        }

        #[rstest]
        fn updates_index_if_enough_time_has_elapsed_after_multiple_updates(
            mut animation: SpriteSheetAnimation,
            mut sprite: TextureAtlasSprite,
            smaller_duration: Duration,
        ) {
            animation.update(&mut sprite, smaller_duration);
            animation.update(&mut sprite, smaller_duration);
            assert_eq!(sprite.index, 1);
        }

        #[rstest]
        fn elapsed_duration_is_reset(
            mut animation: SpriteSheetAnimation,
            mut sprite: TextureAtlasSprite,
            frame_duration: Duration,
            smaller_duration: Duration,
        ) {
            animation.update(&mut sprite, smaller_duration);
            animation.update(&mut sprite, smaller_duration);
            assert_eq!(
                animation.elapsed_in_frame,
                (smaller_duration + smaller_duration) - frame_duration
            );
        }
    }

    mod on_last_frame {
        use super::*;

        #[fixture]
        fn animation(frame_duration: Duration) -> SpriteSheetAnimation {
            SpriteSheetAnimation {
                frames: vec![Frame::new(0, frame_duration), Frame::new(1, frame_duration)],
                current_frame: 1,
                elapsed_in_frame: Duration::from_nanos(0),
            }
        }

        #[rstest]
        fn returns_to_first_frame(
            mut animation: SpriteSheetAnimation,
            mut sprite_at_second_frame: TextureAtlasSprite,
            frame_duration: Duration,
        ) {
            animation.update(&mut sprite_at_second_frame, frame_duration);
            assert_eq!(sprite_at_second_frame.index, 0);
        }
    }

    mod after_last_frame {
        use super::*;

        #[fixture]
        fn animation(frame_duration: Duration) -> SpriteSheetAnimation {
            SpriteSheetAnimation {
                frames: vec![Frame::new(0, frame_duration), Frame::new(1, frame_duration)],
                current_frame: 2,
                elapsed_in_frame: Duration::from_nanos(0),
            }
        }

        #[rstest]
        fn returns_to_first_frame(
            mut animation: SpriteSheetAnimation,
            mut sprite_at_second_frame: TextureAtlasSprite,
            frame_duration: Duration,
        ) {
            animation.update(&mut sprite_at_second_frame, frame_duration);
            assert_eq!(sprite_at_second_frame.index, 0);
        }
    }
}
