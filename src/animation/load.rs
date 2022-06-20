use super::SpriteSheetAnimation;
use bevy_asset::{AssetLoader, LoadContext, LoadedAsset};
use bevy_utils::BoxedFuture;

#[derive(Debug, Default)]
pub(crate) struct SpriteSheetAnimationLoader;

impl AssetLoader for SpriteSheetAnimationLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext<'_>,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let custom_asset = match load_context.path().extension().unwrap().to_str().unwrap() {
                "yaml" | "yml" => SpriteSheetAnimation::from_yaml_bytes(bytes)?,
                "ron" => SpriteSheetAnimation::from_ron_bytes(bytes)?,
                _ => unreachable!(),
            };
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["animation.yml", "animation.yaml", "animation.ron"]
    }
}
