#![warn(
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    clippy::pedantic
)]
#![allow(clippy::needless_pass_by_value, clippy::module_name_repetitions)]
#![cfg_attr(nightly, feature(doc_auto_cfg))]

#[cfg(test)]
#[macro_use]
extern crate rstest;

use std::time::Duration;

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

/// Components that indicates the animation is playing
///
/// Insert the components to play the animation, and remove it to pause it.
///
/// If the animation mode is [`AnimationMode::Once`] this component is automatically removed at the end of the animation.
#[derive(Debug, Copy, Clone, Default)]
pub struct Play;

/// Component that, when applied, can change the playback's rate of the animation.
///
/// Receives a f64 multiplier which will alter the delta, speeding up or slowing down to the desired playback rate.
#[derive(Debug, Copy, Clone)]
pub struct PlaySpeedMultiplier(f64);

impl PlaySpeedMultiplier {
    /// Creates a new `PlaySpeedMultiplier` Component with the multiplier set as desired
    #[must_use]
    pub fn new(multiplier: f64) -> Self {
        Self(multiplier)
    }

    #[allow(dead_code)]
    fn transform(self, duration: Duration) -> Duration {
        duration.mul_f64(self.0)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_transformation() {
        assert_eq!(
            PlaySpeedMultiplier::default().transform(Duration::from_secs(1)),
            Duration::from_secs(1)
        );
    }

    #[test]
    fn double_speed() {
        assert_eq!(
            PlaySpeedMultiplier::from(2.0).transform(Duration::from_secs(2)),
            Duration::from_secs(4)
        );
    }
}
