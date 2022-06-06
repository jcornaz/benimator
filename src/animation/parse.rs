use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    time::Duration,
};

use serde::{de, Deserialize, Deserializer};

use crate::SpriteSheetAnimation;

use super::Mode;

impl SpriteSheetAnimation {
    /// Parse content of a yaml string representing the animation
    ///
    /// # Yaml schema
    ///
    /// ```yaml
    /// # The mode can be one of: 'once', 'repeat', 'ping-pong'
    /// # or 'repeatFrom(n)' (where 'n' is the frame-index to repeat from)
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
    /// # or 'repeatFrom(n)' (where 'n' is the frame-index to repeat from)
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

pub(super) fn deserialize_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_u64(DurationVisitor)
}

struct DurationVisitor;

impl<'de> de::Visitor<'de> for DurationVisitor {
    type Value = Duration;

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v > 0 {
            Ok(Duration::from_millis(v))
        } else {
            Err(de::Error::invalid_value(de::Unexpected::Unsigned(v), &self))
        }
    }

    fn expecting(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "a positive integer")
    }
}

impl<'de> Deserialize<'de> for Mode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ModeVisitor)
    }
}

struct ModeVisitor;

impl<'de> de::Visitor<'de> for ModeVisitor {
    type Value = Mode;

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match s {
            "ping-pong" => Ok(Mode::PingPong),
            "repeat" => Ok(Mode::RepeatFrom(0)),
            "once" => Ok(Mode::Once),
            _ => {
                match s
                    .strip_prefix("repeat-from(")
                    .and_then(|s| s.strip_suffix(')'))
                    .and_then(|s| s.parse::<usize>().ok())
                {
                    Some(index) => Ok(Mode::RepeatFrom(index)),
                    None => Err(de::Error::invalid_value(de::Unexpected::Str(s), &self)),
                }
            }
        }
    }

    fn expecting(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "one of: 'repeat', 'once', 'ping-pong', 'repeat-from(n)'"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Frame, SpriteSheetAnimation};

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
            mode: repeat-from(1)
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
}
