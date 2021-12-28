use std::ops::RangeInclusive;
use std::time::Duration;

use bevy_reflect::TypeUuid;

/// Asset that define an animation of `TextureAtlasSprite`
///
/// See crate level documentation for usage
#[derive(Debug, Clone, Default, TypeUuid)]
#[uuid = "6378e9c2-ecd1-4029-9cd5-801caf68517c"]
pub struct SpriteSheetAnimation {
    /// Frames
    pub frames: Vec<Frame>,
    /// Animation mode
    pub mode: AnimationMode,
}

/// Animation mode (run once or repeat)
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AnimationMode {
    /// Runs the animation once and then stop playing
    Once,

    /// Repeat the animation forever
    Repeat,
}

/// A single animation frame
#[derive(Debug, Copy, Clone, Default)]
pub struct Frame {
    /// Index in the sprite atlas
    pub index: usize,
    /// How long should the frame be displayed
    pub duration: Duration,
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
    pub fn from_range(index_range: RangeInclusive<usize>, frame_duration: Duration) -> Self {
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

    pub(crate) fn has_frames(&self) -> bool {
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
    pub fn new(index: usize, duration: Duration) -> Self {
        Self { index, duration }
    }
}
