use bevy::prelude::*;

/// Determines the alignment of the 
/// sprite grid in relation to it's global transform
/// and it's point of rotation.
/// By default the bottom left corner of the grid maps
/// to the SpriteGrid's position 
#[derive(Copy, Clone, Debug)]
#[derive(Component)]
pub struct SpriteGridAlignment(pub Vec2);

impl SpriteGridAlignment {
    pub fn center() -> Self {
        Self(0.5 * Vec2::ONE)
    }

    pub fn bottom_left() -> Self {
        Self([0.0, 0.0].into())
    }

    pub fn bottom_right() -> Self {
        Self([1.0, 0.0].into())
    }

    pub fn top_left() -> Self {
        Self([0.0, 1.0].into())
    }

    pub fn top_right() -> Self {
        Self([1.0, 1.0].into())
    }
}

impl Default for SpriteGridAlignment {
    fn default() -> Self {
        Self::bottom_left()
    }
}
