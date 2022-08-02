use std::{ops::RangeInclusive, time::Duration};

use crate::Animation;

#[derive(Debug, Clone)]
pub struct Builder {
    frame_duration: Duration,
    range: Option<RangeInclusive<usize>>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            frame_duration: Duration::ZERO,
            range: None,
        }
    }

    pub fn set_default_frame_duration(mut self, duration: Duration) -> Self {
        self.frame_duration = duration;
        self
    }

    pub fn add_frame_index_range(mut self, range: RangeInclusive<usize>) -> Self {
        self.range = Some(range);
        self
    }

    pub fn build(self) -> Result<Animation, InvalidAnimation> {
        if self.frame_duration.is_zero() {
            Err(InvalidAnimation)
        } else if let Some(range) = self.range {
            Ok(Animation::from_range(range, self.frame_duration))
        } else {
            Err(InvalidAnimation)
        }
    }
}

impl Animation {
    pub fn builder() -> Builder {
        Builder::new()
    }
}

impl TryFrom<Builder> for Animation {
    type Error = InvalidAnimation;

    fn try_from(builder: Builder) -> Result<Self, Self::Error> {
        builder.build()
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub struct InvalidAnimation;

#[cfg(test)]
mod tests {
    use crate::Frame;

    use super::*;

    #[rstest]
    fn empty_animation_is_err() {
        assert!(Animation::builder().build().is_err())
    }

    #[rstest]
    fn zero_frame_is_err(duration: Duration) {
        assert!(Animation::builder()
            .set_default_frame_duration(duration)
            .build()
            .is_err())
    }

    #[rstest]
    fn global_frame_duration(duration: Duration, range: RangeInclusive<usize>) {
        let anim: Animation = Animation::builder()
            .set_default_frame_duration(duration)
            .add_frame_index_range(range.clone())
            .try_into()
            .expect("Failed to build animation");

        assert_eq!(anim.mode, Animation::default().mode);
        assert_eq!(
            anim.frames,
            range.map(|i| Frame::new(i, duration)).collect::<Vec<_>>()
        );
    }

    #[fixture]
    fn duration() -> Duration {
        Duration::from_millis(42)
    }

    #[fixture]
    fn range() -> RangeInclusive<usize> {
        4..=8
    }
}
