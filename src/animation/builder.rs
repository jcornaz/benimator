use std::{ops::RangeInclusive, time::Duration};

use crate::{Animation, Frame};

#[derive(Debug, Clone)]
pub struct Builder {
    frame_duration: Duration,
    range: RangeInclusive<usize>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            frame_duration: Duration::ZERO,
            range: 0..=0,
        }
    }

    pub fn set_frame_duration(mut self, duration: Duration) -> Self {
        self.frame_duration = duration;
        self
    }

    pub fn add_frame_index_range(mut self, range: RangeInclusive<usize>) -> Self {
        self.range = range;
        self
    }

    pub fn build(self) -> Result<Animation, InvalidAnimation> {
        Ok(Animation::from_range(self.range, self.frame_duration))
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
    use super::*;

    #[rstest]
    fn empty_animation() {}

    #[rstest]
    fn global_frame_duration(duration: Duration, range: RangeInclusive<usize>) {
        let anim: Animation = Animation::builder()
            .set_frame_duration(duration)
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
