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
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub(super) struct AnimationDto {
    #[serde(default)]
    mode: ModeDto,
    #[serde(default)]
    frame_duration: Option<u64>,
    frames: Vec<FrameDto>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
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

impl SpriteSheetAnimation {
    /// Parse content of a yaml string representing the animation
    ///
    /// # Yaml schema
    ///
    /// ```yaml
    /// # The mode can be one of: 'once', 'repeat', 'ping-pong'
    /// # or 'repeat-from: n' (where 'n' is the frame-index to repeat from)
    /// # The default is 'repeat'
    /// mode: ping-pong
    /// frames:
    ///   - index: 0 # index in the sprite sheet for that frame
    ///     duration: 100 # duration of the frame in milliseconds
    ///   - index: 1
    ///     duration: 100
    ///   - index: 2
    ///     duration: 120
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the content is not a valid yaml representation of an animation
    #[cfg(feature = "unstable-load-from-file")]
    pub fn from_yaml_str(yaml: &str) -> Result<Self, AnimationParseError> {
        serde_yaml::from_str(yaml).map_err(AnimationParseError)
    }

    /// Parse content of a yaml bytes representing the animation
    ///
    /// # Yaml schema
    ///
    /// ```yaml
    /// # The mode can be one of: 'once', 'repeat', 'ping-pong'
    /// # or 'repeat-from: n' (where 'n' is the frame-index to repeat from)
    /// # The default is 'repeat'
    /// mode: ping-pong
    /// frames:
    ///   - index: 0 # index in the sprite sheet for that frame
    ///     duration: 100 # duration of the frame in milliseconds
    ///   - index: 1
    ///     duration: 100
    ///   - index: 2
    ///     duration: 120
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the content is not a valid yaml representation of an animation
    #[cfg(feature = "unstable-load-from-file")]
    pub fn from_yaml_bytes(yaml: &[u8]) -> Result<Self, AnimationParseError> {
        serde_yaml::from_slice(yaml).map_err(AnimationParseError)
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub struct AnimationParseError(pub(super) serde_yaml::Error);

impl Display for AnimationParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Animation format is invalid: {}", self.0)
    }
}

impl Error for AnimationParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_yaml() {
        // given
        let content = "
            mode: ping-pong
            frames:
              - index: 0 # index in the sprite sheet for that frame
                duration: 100 # duration of the frame in milliseconds
              - index: 1
                duration: 100
              - index: 2
                duration: 120";

        // when
        let animation = SpriteSheetAnimation::from_yaml_str(content).unwrap();

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
    fn parse_yaml_default_mode() {
        // given
        let content = "
            frames:
              - index: 0
                duration: 100";

        // when
        let animation = SpriteSheetAnimation::from_yaml_str(content).unwrap();

        // then
        assert_eq!(animation.mode, Mode::RepeatFrom(0));
    }

    #[test]
    fn parse_yaml_repeat() {
        // given
        let content = "
            mode: repeat
            frames:
              - index: 0
                duration: 100";

        // when
        let animation = SpriteSheetAnimation::from_yaml_str(content).unwrap();

        // then
        assert_eq!(animation.mode, Mode::RepeatFrom(0));
    }

    #[test]
    fn parse_yaml_once() {
        // given
        let content = "
            mode: once
            frames:
              - index: 0
                duration: 100";

        // when
        let animation = SpriteSheetAnimation::from_yaml_str(content).unwrap();

        // then
        assert_eq!(animation.mode, Mode::Once);
    }

    #[test]
    fn parse_yaml_repeat_from() {
        // given
        let content = "
            mode:
              repeat-from: 1
            frames:
              - index: 0
                duration: 100
              - index: 1
                duration: 100";

        // when
        let animation = SpriteSheetAnimation::from_yaml_str(content).unwrap();

        // then
        assert_eq!(animation.mode, Mode::RepeatFrom(1));
    }

    #[test]
    fn parse_yaml_zero_duration() {
        // given
        let content = "
            frames:
              - index: 0
                duration: 0";

        // when
        let animation = SpriteSheetAnimation::from_yaml_str(content);

        // then
        assert!(animation.is_err());
    }

    #[test]
    fn parse_yaml_same_duraton_for_all_frames() {
        // given
        let content = "
            frame-duration: 100
            frames:
              - index: 0
              - index: 1
              - index: 2
                duration: 200
        ";

        // when
        let animation = SpriteSheetAnimation::from_yaml_str(content).unwrap();

        // then
        assert_eq!(
            animation.frames,
            vec![
                Frame::new(0, Duration::from_millis(100)),
                Frame::new(1, Duration::from_millis(100)),
                Frame::new(2, Duration::from_millis(200)),
            ]
        );
    }

    #[test]
    fn parse_yaml_same_duraton_for_all_frames_short_hand() {
        // given
        let content = "
            frame-duration: 100
            frames: [0, 1, 2]
        ";

        // when
        let animation = SpriteSheetAnimation::from_yaml_str(content).unwrap();

        // then
        assert_eq!(
            animation.frames,
            vec![
                Frame::new(0, Duration::from_millis(100)),
                Frame::new(1, Duration::from_millis(100)),
                Frame::new(2, Duration::from_millis(100)),
            ]
        );
    }
}
