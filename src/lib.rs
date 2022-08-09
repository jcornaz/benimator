#![warn(
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    missing_docs,
    clippy::pedantic
)]
#![cfg_attr(test, allow(clippy::needless_pass_by_value))]
#![cfg_attr(nightly, feature(doc_auto_cfg))]

//! A sprite animation library for rust game development
//!
//! Initially designed for [bevy], it is now engine agnostic.                       
//!
//! # Get started
//!
//! benimator assumes usage of texture atlas (or equivalent).
//!
//! An [`Animation`] contains the list of animation frames,
//! each frame defined by an index.
//!
//! ```
//! # use std::time::Duration;
//! use benimator::*;
//!
//! // Create an animation
//! let animation = Animation::from_indices(0..=3, FrameRate::from_fps(10.0));
//!
//! // Create a new animation state
//! let mut state = State::new();
//!
//! // In the game loop, for each update, tell the state how much time has elapsed
//! let delta_time = Duration::from_millis(250);
//! state.update(&animation, delta_time);
//!
//! // Then get the current frame index.
//! // (so that we can tell our engine to render the sprite at that index)
//! assert_eq!(state.frame_index(), 2);
//! ```
//!
//! Have a look at the [examples](https://github.com/jcornaz/benimator/tree/main/examples) for complete examples using the [bevy] game engine.
//!
//! [bevy]: https://bevyengine.org

#[cfg(test)]
#[macro_use]
extern crate rstest;

pub use animation::{Animation, Frame, FrameRate};
pub use state::State;

mod animation;
mod state;
