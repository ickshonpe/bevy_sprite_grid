use crate::prelude::*;
use bevy::prelude::*;

#[derive(Clone, Default, Bundle)]
pub struct SpriteGridBundle {
    pub sprite_grid: SpriteGrid,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}
