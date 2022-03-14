use bevy::prelude::*;
use crate::prelude::*;

#[derive(Clone, Default)]
#[derive(Bundle)]
pub struct SpriteGridBundle {
    pub sprite_grid: SpriteGrid,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
}
