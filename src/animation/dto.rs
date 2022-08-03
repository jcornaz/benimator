use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    time::Duration,
};

use serde::{
    de::{self, value::MapAccessDeserializer, MapAccess, Unexpected},
    Deserialize, Serialize,
};

use crate::FrameRate;

use super::{Animation, Frame, Mode};

#[derive(Serialize, Deserialize)]
pub(super) struct AnimationDto {
    #[serde(default)]
    mode: ModeDto,
    #[serde(default, skip_serializing)]
    rate: Option<FrameRateDto>,
    frames: Vec<FrameDto>,
}

#[derive(Serialize, Deserialize)]
enum ModeDto {
    Repeat,
    RepeatFrom(usize),
    Once,
    PingPong,
}

impl Default for ModeDto {
    fn default() -> Self {
        ModeDto::Repeat
    }
}

#[derive(Serialize)]
struct FrameDto {
    index: usize,
    duration: Option<u64>,
}

impl<'de> Deserialize<'de> for FrameDto {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct FrameDtoMap {
            index: usize,
            duration: Option<u64>,
        }

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = FrameDto;

            fn expecting(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
                write!(formatter, "either a frame index, or a frame-index with a")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                v.try_into()
                    .map(|index| FrameDto {
                        index,
                        duration: None,
                    })
                    .map_err(|_| de::Error::invalid_value(Unexpected::Unsigned(v), &self))
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let FrameDtoMap { index, duration } =
                    FrameDtoMap::deserialize(MapAccessDeserializer::new(map))?;
                Ok(FrameDto { index, duration })
            }
        }
        deserializer.deserialize_any(Visitor)
    }
}

impl From<Frame> for FrameDto {
    fn from(frame: Frame) -> Self {
        Self {
            duration: Some(frame.duration.as_millis().try_into().unwrap()),
            index: frame.index,
        }
    }
}

#[derive(Copy, Clone, Deserialize)]
enum FrameRateDto {
    Fps(f64),
    FrameDuration(u64),
    TotalDuration(u64),
}

impl TryFrom<FrameRateDto> for FrameRate {
    type Error = InvalidAnimation;

    fn try_from(value: FrameRateDto) -> Result<Self, Self::Error> {
        match value {
            FrameRateDto::Fps(fps) => Ok(Self::from_fps(fps)),
            FrameRateDto::FrameDuration(millis) => {
                Ok(Self::from_frame_duration(Duration::from_millis(millis)))
            }
            FrameRateDto::TotalDuration(millis) => {
                Ok(Self::from_total_duration(Duration::from_millis(millis)))
            }
        }
    }
}

impl From<Animation> for AnimationDto {
    fn from(animation: Animation) -> Self {
        Self {
            rate: None,
            mode: match animation.mode {
                Mode::Once => ModeDto::Once,
                Mode::RepeatFrom(0) => ModeDto::Repeat,
                Mode::RepeatFrom(i) => ModeDto::RepeatFrom(i),
                Mode::PingPong => ModeDto::PingPong,
            },
            frames: animation.frames.into_iter().map(FrameDto::from).collect(),
        }
    }
}

impl TryFrom<AnimationDto> for Animation {
    type Error = InvalidAnimation;

    fn try_from(animation: AnimationDto) -> Result<Self, Self::Error> {
        let frame_rate: Option<FrameRate> = match animation.rate.map(FrameRate::try_from) {
            Some(result) => Some(result?),
            None => None,
        };
        let mut frames = AnimationDto::build_frames(animation.frames, frame_rate)?;
        if let Some(duration) =
            frame_rate.and_then(|r| r.frame_duration_from_frame_count(frames.len()))
        {
            for frame in &mut frames {
                frame.duration = duration;
            }
        }
        Ok(Self {
            frames,
            mode: match animation.mode {
                ModeDto::Repeat => Mode::RepeatFrom(0),
                ModeDto::RepeatFrom(f) => Mode::RepeatFrom(f),
                ModeDto::Once => Mode::Once,
                ModeDto::PingPong => Mode::PingPong,
            },
        })
    }
}

impl AnimationDto {
    fn build_frames(
        frames: impl IntoIterator<Item = FrameDto>,
        frame_rate: Option<FrameRate>,
    ) -> Result<Vec<Frame>, InvalidAnimation> {
        frames
            .into_iter()
            .map(|FrameDto { index, duration }| {
                duration
                    .map(Duration::from_millis)
                    .or_else(|| frame_rate.map(|r| r.frame_duration))
                    .filter(|d| !d.is_zero())
                    .map(|duration| Frame::new(index, duration))
                    .ok_or(InvalidAnimation::ZeroDuration)
            })
            .collect()
    }
}

#[derive(Debug)]
pub(super) enum InvalidAnimation {
    ZeroDuration,
}

impl Display for InvalidAnimation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            InvalidAnimation::ZeroDuration => write!(f, "invalid duration, must be > 0"), /*  */
        }
    }
}

impl Error for InvalidAnimation {}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{animation::Mode, Frame, FrameRate};

    use super::*;

    #[rstest]
    fn deserialize_serialize(
        #[values(
            Animation::from_indices(0..=2, FrameRate::from_fps(2.0)),
            Animation::from_indices(0..=2, FrameRate::from_frame_duration(Duration::from_secs(1))),
            Animation::from_indices(0..=2, FrameRate::from_fps(2.0)).once(),
            Animation::from_indices(0..=2, FrameRate::from_fps(2.0)).repeat(),
            Animation::from_indices(0..=2, FrameRate::from_fps(2.0)).repeat_from(1),
            Animation::from_indices(0..=2, FrameRate::from_fps(2.0)).ping_pong(),
        )]
        animation: Animation,
    ) {
        let serialized: String = serde_yaml::to_string(&animation).unwrap();
        let deserialized: Animation = serde_yaml::from_str(&serialized).unwrap();
        assert_eq!(animation, deserialized);
    }

    #[test]
    fn parse() {
        // given
        let content = "
            mode: PingPong
            frames:
              - index: 0 # index in the sprite sheet for that frame
                duration: 100 # duration of the frame in milliseconds
              - index: 1
                duration: 100
              - index: 2
                duration: 120";

        // when
        let animation: Animation = serde_yaml::from_str(content).unwrap();

        // then
        assert_eq!(animation.mode, Mode::PingPong);
        assert_eq!(
            animation.frames,
            vec![
                Frame::new(0, Duration::from_millis(100)),
                Frame::new(1, Duration::from_millis(100)),
                Frame::new(2, Duration::from_millis(120)),
            ]
        );
    }

    #[test]
    fn default_mode() {
        // given
        let content = "
            frames:
              - index: 0
                duration: 100";

        // when
        let animation: Animation = serde_yaml::from_str(content).unwrap();

        // then
        assert_eq!(animation.mode, Mode::RepeatFrom(0));
    }

    #[test]
    fn repeat() {
        // given
        let content = "
            mode: Repeat
            frames:
              - index: 0
                duration: 100";

        // when
        let animation: Animation = serde_yaml::from_str(content).unwrap();

        // then
        assert_eq!(animation.mode, Mode::RepeatFrom(0));
    }

    #[test]
    fn once() {
        // given
        let content = "
            mode: Once
            frames:
              - index: 0
                duration: 100";

        // when
        let animation: Animation = serde_yaml::from_str(content).unwrap();

        // then
        assert_eq!(animation.mode, Mode::Once);
    }

    #[test]
    fn repeat_from() {
        // given
        let content = "
            mode: !RepeatFrom 1
            frames:
              - index: 0
                duration: 100
              - index: 1
                duration: 100";

        // when
        let animation: Animation = serde_yaml::from_str(content).unwrap();

        // then
        assert_eq!(animation.mode, Mode::RepeatFrom(1));
    }

    #[test]
    fn zero_duration() {
        // given
        let content = "
            frames:
              - index: 0
                duration: 0";

        // when
        let animation: Result<Animation, _> = serde_yaml::from_str(content);

        // then
        assert!(animation.is_err());
    }

    #[test]
    fn same_duration_for_all_frames() {
        // given
        let content = "
            rate: !FrameDuration 100
            frames:
              - index: 0
              - index: 1
              - index: 2
        ";

        // when
        let animation: Animation = serde_yaml::from_str(content).unwrap();

        // then
        assert_eq!(
            animation,
            Animation::from_indices(
                0..=2,
                FrameRate::from_frame_duration(Duration::from_millis(100))
            )
        );
    }

    #[test]
    fn same_duration_for_all_frames_short_hand() {
        // given
        let content = "
            rate: !FrameDuration 100
            frames: [0, 1, 2]
        ";

        // when
        let animation: Animation = serde_yaml::from_str(content).unwrap();

        // then
        assert_eq!(
            animation,
            Animation::from_indices(
                0..=2,
                FrameRate::from_frame_duration(Duration::from_millis(100))
            )
        );
    }

    #[test]
    fn defined_via_fps() {
        // given
        let content = "
            rate: !Fps 10.0
            frames: [0, 1, 2]
        ";

        // when
        let animation: Animation = serde_yaml::from_str(content).unwrap();

        // then
        assert_eq!(
            animation,
            Animation::from_frames([
                Frame::new(0, Duration::from_millis(100)),
                Frame::new(1, Duration::from_millis(100)),
                Frame::new(2, Duration::from_millis(100)),
            ])
        );
    }

    #[test]
    fn defined_via_total_duration() {
        // given
        let content = "
            rate: !TotalDuration 600
            frames: [0, 1, 2]
        ";

        // when
        let animation: Animation = serde_yaml::from_str(content).unwrap();

        // then
        assert_eq!(
            animation,
            Animation::from_frames([
                Frame::new(0, Duration::from_millis(200)),
                Frame::new(1, Duration::from_millis(200)),
                Frame::new(2, Duration::from_millis(200)),
            ])
        );
    }
}
