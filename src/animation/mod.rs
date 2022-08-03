use core::time::Duration;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde")]
mod dto;

/// Definition of an animation
#[cfg_attr(
    feature = "serde",
    doc = "

# Deserialization format
 
```yaml
# The mode can be one of: 'Once', 'Repeat', 'PingPong'
# or '!RepeatFrom: n' (where 'n' is the frame-index to repeat from)
# The default is 'Repeat'
mode: PingPong
frames:
  - index: 0 # index in the sprite sheet for that frame
    duration: 100 # duration of the frame in milliseconds
  - index: 1
    duration: 100
  - index: 2
    duration: 120
```

There is also a short-hand notation if all frames have the same duration:

```yaml
frame_duration: 100
frames: [0, 1, 2] # sequence of frame indices
```
"
)]
#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(try_from = "dto::AnimationDto", into = "dto::AnimationDto")
)]
pub struct Animation {
    /// Frames
    pub(crate) frames: Vec<Frame>,
    /// Animation mode
    pub(crate) mode: Mode,
}

/// A single animation frame
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Frame {
    /// Index in the sprite atlas
    pub(crate) index: usize,
    /// How long should the frame be displayed
    pub(crate) duration: Duration,
}

impl Animation {
    /// Create a new animation from frames
    #[must_use]
    pub fn from_frames(frames: Vec<Frame>) -> Self {
        Self {
            frames,
            mode: Mode::default(),
        }
    }

    /// Create a new animation from an index iterator, using the same frame duration for each frame.
    ///
    /// # Examples
    ///
    /// From an index range
    /// ```
    /// # use benimator::{Animation, FrameRate};
    /// # use std::time::Duration;
    /// let animation = Animation::from_indices(0..=5, FrameRate::from_fps(12.0));
    /// ```
    ///
    /// From an index array
    /// ```
    /// # use benimator::{Animation, FrameRate};
    /// # use std::time::Duration;
    /// let animation = Animation::from_indices([1, 2, 3, 4], FrameRate::from_fps(12.0));
    /// ```
    ///
    /// Reversed animation:
    /// ```
    /// # use benimator::{Animation, FrameRate};
    /// # use std::time::Duration;
    /// let animation = Animation::from_indices((0..5).rev(), FrameRate::from_fps(12.0));
    /// ```
    ///
    /// Chained ranges
    /// ```
    /// # use benimator::{Animation, FrameRate};
    /// # use std::time::Duration;
    /// let animation = Animation::from_indices((0..3).chain(10..15), FrameRate::from_fps(12.0));
    /// ```
    ///
    /// To use different non-uniform frame-duration, see [`from_frames`](Animation::from_frames)
    ///
    /// # Panics
    ///
    /// Panics if the duration is zero
    pub fn from_indices(indices: impl IntoIterator<Item = usize>, frame_rate: FrameRate) -> Self {
        indices
            .into_iter()
            .map(|index| Frame::new(index, frame_rate.frame_duration))
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

impl FromIterator<Frame> for Animation {
    fn from_iter<T: IntoIterator<Item = Frame>>(iter: T) -> Self {
        Self::from_frames(iter.into_iter().collect())
    }
}

impl Extend<Frame> for Animation {
    fn extend<T: IntoIterator<Item = Frame>>(&mut self, iter: T) {
        self.frames.extend(iter);
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

/// Frame-Rate definition
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[must_use]
pub struct FrameRate {
    frame_duration: Duration,
}

impl FrameRate {
    /// Frame rate defined by the FPS (Frame-Per-Second)
    ///
    /// # Panics
    ///
    /// This function will panic if `fps` is negative, zero or not finite.
    pub fn from_fps(fps: f64) -> Self {
        assert!(fps.is_finite() && fps > 0.0, "Invalid FPS: ${fps}");
        Self {
            frame_duration: Duration::from_secs(1).div_f64(fps),
        }
    }

    /// Frame rate defined by the duration of each frame
    pub fn from_frame_duration(duration: Duration) -> Self {
        Self {
            frame_duration: duration,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest]
    #[should_panic]
    fn invalid_frame_rate_panics(#[values(0.0, -1.0, f64::NAN, f64::INFINITY)] fps: f64) {
        let _ = FrameRate::from_fps(fps);
    }

    #[test]
    #[should_panic]
    fn panics_for_zero_duration() {
        let _ = Frame::new(0, Duration::ZERO);
    }

    #[test]
    fn extends() {
        let mut anim = Animation::from_indices(
            0..=0,
            FrameRate::from_frame_duration(Duration::from_secs(1)),
        );
        anim.extend([Frame::new(2, Duration::from_secs(2))]);
        assert_eq!(
            anim,
            Animation::from_frames(vec![
                Frame::new(0, Duration::from_secs(1)),
                Frame::new(2, Duration::from_secs(2))
            ])
        );
    }

    #[test]
    fn fps_frame_duration_equivalence() {
        assert_eq!(
            Animation::from_indices(1..=3, FrameRate::from_fps(10.0)),
            Animation::from_indices(
                1..=3,
                FrameRate::from_frame_duration(Duration::from_millis(100))
            ),
        );
    }
}
