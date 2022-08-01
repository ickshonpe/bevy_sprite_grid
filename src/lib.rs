pub mod bundles;
pub mod picking;
pub mod render;
pub mod sprite_cell;
pub mod sprite_grid;
pub mod view;

pub mod prelude {
    pub use crate::bundles::*;
    pub use crate::picking::*;
    pub use crate::sprite_cell::*;
    pub use crate::sprite_grid::*;
    pub use crate::view::*;
    pub use crate::SpriteGridPlugin;
}

use bevy::prelude::*;

pub struct SpriteGridPlugin;

impl Plugin for SpriteGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(render::RenderSpriteGridPlugin);
    }
}
