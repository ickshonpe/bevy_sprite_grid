use std::ops::Mul;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::render::RenderApp;
use bevy::render::RenderStage;
use bevy::render::RenderWorld;
use bevy::sprite::ExtractedSprite;
use bevy::sprite::ExtractedSprites;
use bevy::sprite::SpriteSystem;
use copyless::VecHelper;
use crate::prelude::*;

fn extract_tiles(
    mut render_world: ResMut<RenderWorld>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    sprite_grid_query: Query<(
        &SpriteGrid,
        &GlobalTransform,
        &Visibility,
        Option<&SpriteGridAlignment>,
    )>,
) {
    let mut extracted_sprites = render_world.get_resource_mut::<ExtractedSprites>().unwrap();
    for (sprite_grid, &global_transform, visibility, alignment_option) in sprite_grid_query.iter() {
        if !visibility.is_visible { continue }
        let alignment_translation =
            -alignment_option.cloned().unwrap_or_default().0 * sprite_grid.grid_size();
        let grid_transform =
            global_transform.mul(
            Transform {
                translation: alignment_translation.extend(0.0),
                scale: sprite_grid.cell_size.extend(1.0),
                ..Default::default()
            });

        for x in 0..sprite_grid.x_len {
            for y in 0..sprite_grid.y_len {
                let grid_pos = vec2(x as f32, y as f32) + 0.5 * Vec2::ONE;                
                let cell_transform = Transform {
                    translation: grid_pos.extend(0.0),
                    ..Default::default()
                };
                let transform = grid_transform.mul(cell_transform);
                match &sprite_grid[[x, y]] {
                    &SpriteCell::Sprite(CellSprite { ref image_handle, color, flip_x, flip_y, custom_size }) => {
                        extracted_sprites.sprites.alloc().init(ExtractedSprite {
                            color,
                            transform,
                            rect: None,
                            custom_size: custom_size.map(|custom_size| custom_size / sprite_grid.cell_size),
                            flip_x,
                            flip_y,
                            image_handle_id: image_handle.id,
                        });
                    }
                    &SpriteCell::AtlasSprite(CellAtlasSprite { ref atlas_handle, atlas_index, color, flip_x, flip_y, custom_size }) => {
                        if let Some(texture_atlas) = texture_atlases.get(atlas_handle) {
                            let rect = texture_atlas.textures[atlas_index].into();
                            extracted_sprites.sprites.alloc().init(ExtractedSprite {
                                color,
                                transform,
                                rect,
                                custom_size: custom_size.map(|custom_size| custom_size / sprite_grid.cell_size),
                                flip_x,
                                flip_y,
                                image_handle_id: texture_atlas.texture.id,
                            });
                        } 
                    },
                    _ => ()
                }
            }
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum SpriteGridRenderSystem {
    ExtractTiles,
}

pub(crate) struct RenderSpriteGridPlugin;

impl Plugin for RenderSpriteGridPlugin {
    fn build(&self, app: &mut App) {
        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app
            .add_system_to_stage(
                RenderStage::Extract,
                extract_tiles
                .label(SpriteGridRenderSystem::ExtractTiles)
                .after(SpriteSystem::ExtractSprites)
            );
        }
    }
}