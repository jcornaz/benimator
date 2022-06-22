use super::SpriteSheetAnimation;
use bevy_asset::{AssetLoader, LoadContext, LoadedAsset};
use bevy_utils::BoxedFuture;

#[derive(Debug)]
pub(crate) struct SpriteSheetAnimationLoader {
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

impl AssetLoader for SpriteSheetAnimationLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext<'_>,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let custom_asset = match load_context.path().extension().unwrap().to_str().unwrap() {
                #[cfg(feature = "yaml")]
                "yaml" | "yml" => SpriteSheetAnimation::from_yaml_bytes(bytes)?,

                #[cfg(feature = "ron")]
                "ron" => SpriteSheetAnimation::from_ron_bytes(bytes)?,

                _ => unreachable!(),
            };
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &self.extensions
    }
}
