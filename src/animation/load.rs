use std::{
    error::Error,
    fmt::{self, Display},
};

use crate::SpriteSheetAnimation;

/// Loader of animation file
///
/// It is not necessary to use this directly if you are using the bevy plugin,
/// as it is already registered as an asset loader.
#[cfg_attr(
    feature = "yaml",
    doc = "

# Yaml schema

```yaml
# The mode can be one of: 'once', 'repeat', 'ping-pong'
# or 'repeat-from: n' (where 'n' is the frame-index to repeat from)
# The default is 'Repeat'type
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
#[cfg_attr(
    feature = "ron",
    doc = "

# Ron Schema
```ron
(
  // The mode can be one of: 'Once', 'Repeat', 'PingPong'
  // or 'RepeatFrom(n)' (where 'n' is the frame-index to repeat from)
  // The default is 'Repeat'
  mode: PingPong,
  frames: [
    (
      index: 0, //index in the sprite sheet for that frame
      duration: Some(100), // duration of the frame in milliseconds
    ),
    (index: 1, duration: Some(100)),
    (index: 2, duration: Some(120)),
  ]
)
```

There is also a short-hand notation if all frames have the same duration:
```ron
(
  frame_duration: 100,
  frames: [0, 1, 2, 3, 4],
)
```
"
)]
#[derive(Debug)]
pub struct SpriteSheetAnimationLoader {
    extensions: Vec<&'static str>,
}

impl Default for SpriteSheetAnimationLoader {
    #[allow(clippy::vec_init_then_push)]
    fn default() -> Self {
        let mut extensions = Vec::with_capacity(3);

        #[cfg(feature = "yaml")]
        extensions.push("animation.yml");

        #[cfg(feature = "yaml")]
        extensions.push("animation.yaml");

        #[cfg(feature = "ron")]
        extensions.push("animation.ron");

        Self { extensions }
    }
}

impl SpriteSheetAnimationLoader {
    /// Returns supported extensions
    ///
    /// [`SpriteSheetAnimationLoader::load`] can only succeed one of the returned extensions
    #[must_use]
    pub fn supported_extensions(&self) -> &[&str] {
        &self.extensions
    }

    /// Load animation from file content
    ///
    /// # Errors
    ///
    /// Returns an error if the extension is not supported or if the data content is not valid for that extension
    #[allow(clippy::unused_self)]
    pub fn load(
        &self,
        extension: &str,
        data: &[u8],
    ) -> Result<SpriteSheetAnimation, AnimationParseError> {
        match extension {
            #[cfg(feature = "yaml")]
            "yaml" | "yml" => yaml::from_slice(data).map_err(AnimationParseError::new),

            #[cfg(feature = "ron")]
            "ron" => ron::Options::default()
                .with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME)
                .from_bytes(data)
                .map_err(AnimationParseError::new),

            _ => Err(AnimationParseError(UnexpectedExtension.into())),
        }
    }
}

/// Error when parsing an animation file content
#[derive(Debug)]
#[non_exhaustive]
pub struct AnimationParseError(anyhow::Error);

impl Display for AnimationParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Animation format is invalid: {}", self.0)
    }
}

impl Error for AnimationParseError {}

impl AnimationParseError {
    fn new(err: impl Error + Send + Sync + 'static) -> Self {
        Self(anyhow::Error::from(err))
    }
}

#[derive(Debug, Clone, Copy)]
struct UnexpectedExtension;

impl Display for UnexpectedExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unexpected extension")
    }
}

impl Error for UnexpectedExtension {}
