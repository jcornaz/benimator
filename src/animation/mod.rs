#[cfg(feature = "serde")]
mod dto;

use std::{ops::RangeInclusive, time::Duration};

#[cfg(feature = "serde")]
use serde::Deserialize;

/// Asset that define an animation of `TextureAtlasSprite`
///
/// See crate level documentation for usage
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "dto::AnimationDto"))]
pub struct SpriteSheetAnimation {
    /// Frames
    pub(crate) frames: Vec<Frame>,
    /// Animation mode
    pub(crate) mode: Mode,
}

/// A single animation frame
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Frame {
    /// Index in the sprite atlas
    pub(crate) index: usize,
    /// How long should the frame be displayed
    pub(crate) duration: Duration,
}

impl SpriteSheetAnimation {
    /// Create a new animation from frames
    #[must_use]
    pub fn from_frames(frames: Vec<Frame>) -> Self {
        Self {
            frames,
            mode: Mode::default(),
        }
    }

    /// Create a new animation from index-range, using the same frame duration for each frame.
    ///
    /// For more granular configuration, see [`from_frames`](SpriteSheetAnimation::from_frames)
    ///
    /// # Panics
    ///
    /// Panics if the duration is zero
    #[must_use]
    pub fn from_range(index_range: RangeInclusive<usize>, frame_duration: Duration) -> Self {
        Self::from_iter(index_range, frame_duration)
    }

    /// Create a new animation from an index iterator, using the same frame duration for each frame.
    ///
    /// # Example
    ///
    /// You may use this to create a reversed animation:
    /// ```
    /// # use benimator::SpriteSheetAnimation;
    /// # use std::time::Duration;
    /// let animation = SpriteSheetAnimation::from_iter((0..5).rev(), Duration::from_millis(100));
    /// ```
    ///
    /// For more granular configuration, see [`from_frames`](SpriteSheetAnimation::from_frames)
    ///
    /// # Panics
    ///
    /// Panics if the duration is zero
    pub fn from_iter(indices: impl IntoIterator<Item = usize>, frame_duration: Duration) -> Self {
        indices
            .into_iter()
            .map(|index| Frame::new(index, frame_duration))
            .collect()
    }

    /// Runs the animation once and then stop playing
    #[must_use]
    pub fn once(mut self) -> Self {
        self.mode = Mode::Once;
        self
    }

    /// Repeat the animation forever
    #[must_use]
    pub fn repeat(mut self) -> Self {
        self.mode = Mode::RepeatFrom(0);
        self
    }

    /// Repeat the animation forever, from a given frame index (loop back to it at the end of the
    /// animation)
    #[must_use]
    pub fn repeat_from(mut self, frame_index: usize) -> Self {
        self.mode = Mode::RepeatFrom(frame_index);
        self
    }

    /// Repeat the animation forever, going back and forth between the first and last frame.
    #[must_use]
    pub fn ping_pong(mut self) -> Self {
        self.mode = Mode::PingPong;
        self
    }

    pub(crate) fn has_frames(&self) -> bool {
        !self.frames.is_empty()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum Mode {
    Once,
    RepeatFrom(usize),
    PingPong,
}

impl FromIterator<Frame> for SpriteSheetAnimation {
    fn from_iter<T: IntoIterator<Item = Frame>>(iter: T) -> Self {
        Self::from_frames(iter.into_iter().collect())
    }
}

impl Default for Mode {
    #[inline]
    fn default() -> Self {
        Self::RepeatFrom(0)
    }
}

impl Frame {
    /// Create a new animation frame
    ///
    /// The duration must be > 0
    ///
    /// # Panics
    ///
    /// Panics if the duration is zero
    #[inline]
    #[must_use]
    pub fn new(index: usize, duration: Duration) -> Self {
        assert!(
            !duration.is_zero(),
            "zero-duration is invalid for animation frame"
        );
        Self { index, duration }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn panics_for_zero_duration() {
        let _ = Frame::new(0, Duration::ZERO);
    }
}
