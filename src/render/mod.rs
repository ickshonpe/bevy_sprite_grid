

use std::ops::Mul;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::render::RenderApp;
use bevy::render::RenderStage;
use bevy::render::RenderWorld;
use bevy::render::camera::ActiveCamera;
use bevy::render::camera::ActiveCameras;
use bevy::render::camera::CameraPlugin;
use bevy::render::texture::DEFAULT_IMAGE_HANDLE;
use bevy::sprite::ExtractedSprite;
use bevy::sprite::ExtractedSprites;
use bevy::sprite::SpriteSystem;
use copyless::VecHelper;
use crate::prelude::*;

fn extract_tiles(
    active_cameras: Res<ActiveCameras>,
    cameras: Query<(&OrthographicProjection, &GlobalTransform)>,
    mut render_world: ResMut<RenderWorld>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    sprite_grid_query: Query<(
        &SpriteGrid,
        &GlobalTransform,
        &Visibility,
    )>,
) {
    let mut extracted_sprites = render_world.get_resource_mut::<ExtractedSprites>().unwrap();
    let (projection, camera_transform) = 
        if let Some(ActiveCamera { entity: Some(entity), .. }) = active_cameras.get(CameraPlugin::CAMERA_2D) {
            if let Ok((projection, camera_transform)) = cameras.get(*entity) {
                (projection, camera_transform)
            } else {
                return;
            }
        } else {
            return;
        };
    let culling_rect_half_size = projection.right * Vec2::X + projection.top * Vec2::Y;

    for (sprite_grid, global_transform, visibility) in sprite_grid_query.iter() {
        if !visibility.is_visible { continue }
        if sprite_grid.x_len == 0 || sprite_grid.y_len == 0 { continue }
        let alignment_translation =
            -sprite_grid.alignment.0 * sprite_grid.grid_size();
        let grid_transform =
            global_transform.mul(
            Transform {
                translation: alignment_translation.extend(0.0),
                ..Default::default()
            });
        let [xs, ys] = if let SpriteGridCulling::Enabled { margin } = sprite_grid.culling {
            if let Some(ranges) = pick_rect(
                sprite_grid,
                global_transform,
                culling_rect_half_size + margin,
                camera_transform,
            ) {
                ranges
            } else {
                continue;
            }
        } else {
            [[0, sprite_grid.x_len - 1], [0, sprite_grid.y_len - 1]]
        };
        
        for x in xs[0]..=xs[1] {
            for y in ys[0]..=ys[1] {
                let grid_pos = (vec2(x as f32, y as f32) + 0.5 * Vec2::ONE) * sprite_grid.cell_size;                 
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
                            custom_size,
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
                                custom_size,
                                flip_x,
                                flip_y,
                                image_handle_id: texture_atlas.texture.id,
                            });
                        } 
                    },
                    &SpriteCell::Color(color) => {
                        extracted_sprites.sprites.alloc().init(ExtractedSprite {
                            color,
                            transform,
                            rect: None,
                            custom_size: Some(sprite_grid.cell_size),
                            flip_x: false,
                            flip_y: false,
                            image_handle_id: DEFAULT_IMAGE_HANDLE.id,
                        });
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