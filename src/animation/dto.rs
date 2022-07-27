use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    time::Duration,
};

use serde::{
    de::{self, value::MapAccessDeserializer, MapAccess, Unexpected},
    Deserialize,
};

use super::{Frame, Mode, SpriteSheetAnimation};

#[derive(Deserialize)]
pub(super) struct AnimationDto {
    #[serde(default)]
    mode: ModeDto,
    #[serde(default)]
    frame_duration: Option<u64>,
    frames: Vec<FrameDto>,
}

#[derive(Deserialize)]
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

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
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

impl TryFrom<AnimationDto> for SpriteSheetAnimation {
    type Error = InvalidAnimation;

    fn try_from(animation: AnimationDto) -> Result<Self, Self::Error> {
        Ok(Self {
            frames: animation
                .frames
                .into_iter()
                .map(|FrameDto { index, duration }| {
                    match duration.or(animation.frame_duration).filter(|d| *d > 0) {
                        Some(duration) => Ok(Frame::new(index, Duration::from_millis(duration))),
                        None => Err(InvalidAnimation::ZeroDuration),
                    }
                })
                .collect::<Result<_, _>>()?,
            mode: match animation.mode {
                ModeDto::Repeat => Mode::RepeatFrom(0),
                ModeDto::RepeatFrom(f) => Mode::RepeatFrom(f),
                ModeDto::Once => Mode::Once,
                ModeDto::PingPong => Mode::PingPong,
            },
        })
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

/// Error when parsing an animation file content
#[derive(Debug)]
#[non_exhaustive]
pub struct AnimationParseError(pub(crate) anyhow::Error);

impl AnimationParseError {
    pub(super) fn new(err: impl Error + Send + Sync + 'static) -> Self {
        Self(anyhow::Error::from(err))
    }
}

impl Display for AnimationParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Animation format is invalid: {}", self.0)
    }
}

impl Error for AnimationParseError {}
