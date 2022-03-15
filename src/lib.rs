pub mod bundles;
pub mod render;
pub mod sprite_grid;
pub mod picking;
pub mod prelude {
    pub use crate::bundles::*;
    pub use crate::sprite_grid::*;
    pub use crate::SpriteGridPlugin;
    pub use crate::picking::*;
}

use bevy::prelude::*;

pub struct SpriteGridPlugin;

impl Plugin for SpriteGridPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(render::RenderSpriteGridPlugin);
        
    }
}