use bevy_ecs::prelude::*;
use bevy_log::prelude::*;

use crate::SpriteSheetAnimation;

pub(crate) fn systems() -> SystemSet {
    SystemSet::new().with_system(report_animation_used_as_component.system())
}

fn report_animation_used_as_component(query: Query<'_, Entity, Added<SpriteSheetAnimation>>) {
    if query.iter().next().is_some() {
        warn!("A SpriteSheetAnimation was inserted as a component. That has no effect. The animations should be added to `Assets<SpriteSheetAnimation>` and the asset-handle inserted as a component.");
    }
}
