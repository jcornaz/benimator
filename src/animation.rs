use std::{ops::RangeInclusive, time::Duration};

#[cfg(feature = "unstable-load-from-file")]
use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use bevy_reflect::TypeUuid;
#[cfg(feature = "unstable-load-from-file")]
use serde::{de, Deserialize, Deserializer};

/// Asset that define an animation of `TextureAtlasSprite`
///
/// See crate level documentation for usage
#[derive(Debug, Clone, Default, TypeUuid)]
#[cfg_attr(feature = "unstable-load-from-file", derive(Deserialize))]
#[uuid = "6378e9c2-ecd1-4029-9cd5-801caf68517c"]
pub struct SpriteSheetAnimation {
    /// Frames
    pub(crate) frames: Vec<Frame>,
    /// Animation mode
    #[cfg_attr(feature = "unstable-load-from-file", serde(default))]
    pub(crate) mode: Mode,
}

/// Animation mode (run once, repeat or ping-pong)
///
/// Deprecated
/// ---
/// This is not exposed in any of the public APIs of the crate so there is no reason to depend on
/// it. Use 'builder-style' methods like [`SpriteSheetAnimation::repeat`] instead.
#[deprecated]
#[doc(hidden)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AnimationMode {
    /// Runs the animation once and then stop playing
    Once,

    /// Repeat the animation forever
    Repeat,

    /// Repeat the animation forever, going back and forth between
    /// the first and last frame.
    PingPong,
}

/// A single animation frame
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "unstable-load-from-file", derive(Deserialize))]
pub struct Frame {
    /// Index in the sprite atlas
    pub(crate) index: usize,
    /// How long should the frame be displayed
    #[cfg_attr(
        feature = "unstable-load-from-file",
        serde(deserialize_with = "deserialize_duration")
    )]
    pub(crate) duration: Duration,
}

#[cfg(feature = "unstable-load-from-file")]
fn deserialize_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_u64(DurationVisitor)
}

#[cfg(feature = "unstable-load-from-file")]
struct DurationVisitor;

#[cfg(feature = "unstable-load-from-file")]
impl<'de> de::Visitor<'de> for DurationVisitor {
    type Value = Duration;

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Duration::from_millis(v))
    }

    fn expecting(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "a positive integer")
    }
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

    /// Parse content of a yaml file representing the animation
    ///
    /// # Errors
    ///
    /// Returns an error if the content is not a valid yaml representation of an animation
    #[cfg(feature = "unstable-load-from-file")]
    pub fn from_yaml(yaml: &str) -> Result<Self, AnimationParseError> {
        serde_yaml::from_str(yaml).map_err(AnimationParseError)
    }
}

#[cfg(feature = "unstable-load-from-file")]
#[derive(Debug)]
#[non_exhaustive]
pub struct AnimationParseError(serde_yaml::Error);

#[cfg(feature = "unstable-load-from-file")]
impl Display for AnimationParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Animation format is invalid: {}", self.0)
    }
}

#[cfg(feature = "unstable-load-from-file")]
impl Error for AnimationParseError {}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "unstable-load-from-file", derive(Deserialize))]
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

#[allow(deprecated)]
impl Default for AnimationMode {
    #[inline]
    fn default() -> Self {
        Self::Repeat
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

    #[cfg(feature = "unstable-load-from-file")]
    mod yaml_parsing {
        use super::*;

        #[test]
        fn load_from_yaml() {
            // given
            let content = "
            frames:
              - index: 0
                duration: 100 # ms
              - index: 1
                duration: 100 # ms
              - index: 2
                duration: 120 # ms";

            // when
            let animation = SpriteSheetAnimation::from_yaml(content).unwrap();

            // then
            assert_eq!(animation.mode, Mode::RepeatFrom(0));
            assert_eq!(
                animation.frames,
                vec![
                    Frame::new(0, Duration::from_millis(100)),
                    Frame::new(1, Duration::from_millis(100)),
                    Frame::new(2, Duration::from_millis(120)),
                ]
            );
        }
    }
}
