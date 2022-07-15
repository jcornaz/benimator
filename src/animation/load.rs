use crate::SpriteSheetAnimation;

use super::AnimationParseError;

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
    pub fn supported_extensions(&self) -> &[&str] {
        &self.extensions
    }

    #[allow(clippy::unused_self)]
    pub fn load(
        &self,
        extension: &str,
        bytes: &[u8],
    ) -> Result<SpriteSheetAnimation, AnimationParseError> {
        match extension {
            #[cfg(feature = "yaml")]
            "yaml" | "yml" => SpriteSheetAnimation::from_yaml_bytes(bytes),

            #[cfg(feature = "ron")]
            "ron" => SpriteSheetAnimation::from_ron_bytes(bytes),

            _ => unreachable!(),
        }
    }
}
