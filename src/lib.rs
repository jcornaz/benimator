#![warn(
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    missing_docs,
    clippy::pedantic
)]
#![allow(clippy::needless_pass_by_value, clippy::module_name_repetitions)]
#![cfg_attr(nightly, feature(doc_auto_cfg))]

//! A sprite animation library for rust game development
//!
//! Initially designed for [bevy](https://bevyengine.org), it is now engine agnostic.                       
//!
//! # Get started
//!
//! benimator assumes usage of texture atlas (or equivalent).
//!
//! An [`Animation`] contains the list of animation frames,
//! each frame defined by an index.
//!
//! ```
//! use std::time::Duration;
//! use benimator::{Animation, FrameRate, State};
//!
//! // Create an animation
//! let animation = Animation::from_indices(0..=3, FrameRate::from_fps(10.0));
//!
//! // Get a new animation state
//! let mut state = State::default();
//!
//! // At each game-loop update, tell the state how much time has elapsed
//! let delta_time = Duration::from_millis(250);
//! state.update(&animation, delta_time);
//!
//! // Then we can get the current frame index in order to render the right sprite
//! assert_eq!(state.sprite_frame_index(), 2);
//! ```

#[cfg(test)]
#[macro_use]
extern crate rstest;

pub use animation::{Animation, Frame, FrameRate};
pub use state::State;

mod animation;
mod state;
