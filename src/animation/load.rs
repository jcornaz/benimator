use std::{error::Error, fmt::Display};

use crate::SpriteSheetAnimation;

use super::AnimationParseError;

/// Loader of animation file
///
/// It is not necessary to use this directly if you are using the bevy plugin,
/// as it is already registered as an asset loader.
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
            "yaml" | "yml" => SpriteSheetAnimation::from_yaml_bytes(data),

            #[cfg(feature = "ron")]
            "ron" => SpriteSheetAnimation::from_ron_bytes(data),

            _ => Err(AnimationParseError(UnexpectedExtension.into())),
        }
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
