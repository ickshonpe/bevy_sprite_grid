use crate::prelude::*;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::render::texture::DEFAULT_IMAGE_HANDLE;
use bevy::render::Extract;
use bevy::render::RenderApp;
use bevy::render::RenderStage;
use bevy::sprite;
use bevy::sprite::ExtractedSprite;
use bevy::sprite::ExtractedSprites;
use bevy::sprite::SpriteSystem;
use copyless::VecHelper;
use std::ops::Mul;

fn extract_grid_sprites(
    cameras: Extract<Query<(&OrthographicProjection, &GlobalTransform), With<Camera2d>>>,
    mut extracted_sprites: ResMut<ExtractedSprites>,
    texture_atlases: Extract<Res<Assets<TextureAtlas>>>,
    sprite_grid_query: Extract<
        Query<(
            Entity,
            &SpriteGrid,
            &GlobalTransform,
            &ComputedVisibility,
            Option<&SpriteGridView>,
        )>,
    >,
) {
    let (projection, camera_transform) =
        if let Some((projection, camera_transform)) = cameras.iter().next() {
            (projection, camera_transform)
        } else {
            return;
        };
    let culling_rect_half_size = projection.right * Vec2::X + projection.top * Vec2::Y;

    for (entity, sprite_grid, global_transform, visibility, view) in sprite_grid_query.iter() {
        if !visibility.is_visible() {
            continue;
        }
        if sprite_grid.x_len == 0 || sprite_grid.y_len == 0 {
            continue;
        }
        let alignment_translation = -sprite_grid.alignment.0 * sprite_grid.grid_size();
        let grid_transform = global_transform.mul(Transform {
            translation: alignment_translation.extend(0.0),
            ..Default::default()
        });
        let view_rect = if let SpriteGridCulling::Enabled { margin } = sprite_grid.culling {
            if let Some(ranges) = pick_rect(
                sprite_grid,
                global_transform,
                culling_rect_half_size + margin,
                camera_transform,
            ) {
                if let Some(view) = view {
                    let ranges = ranges.intersect_with(view.0);
                    if ranges.is_none() {
                        continue;
                    }
                    ranges.unwrap()
                } else {
                    ranges
                }
            } else {
                continue;
            }
        } else if let Some(view) = view {
            view.0
        } else {
            SpriteGridRect {
                left: 0,
                right: sprite_grid.x_len,
                bottom: 0,
                top: sprite_grid.y_len,
            }
        };
        let mut count = 0;
        for ([x, y], sprite_cell) in sprite_grid.iter(view_rect.xs(), view_rect.ys()) {
            let grid_pos = (vec2(x as f32, y as f32) + 0.5 * Vec2::ONE) * sprite_grid.cell_size;
            let cell_transform = Transform {
                translation: grid_pos.extend(0.0),
                ..Default::default()
            };
            let transform = grid_transform.mul(cell_transform);
            let extracted_sprite = match sprite_cell {
                SpriteCell::Texture(cell) => {
                    let (image_handle_id, rect) = match &cell.texture {
                        TextureSource::Image { handle } => (handle.id, None),
                        TextureSource::Atlas { handle, index } => {
                            if let Some(texture_atlas) = texture_atlases.get(handle) {
                                (
                                    texture_atlas.texture.id,
                                    Some(texture_atlas.textures[*index].into()),
                                )
                            } else {
                                continue;
                            }
                        }
                    };
                    ExtractedSprite {
                        entity,
                        color: cell.color,
                        transform,
                        rect,
                        custom_size: cell.custom_size,
                        flip_x: cell.flip_x,
                        flip_y: cell.flip_y,
                        image_handle_id,
                        anchor: Vec2::ZERO,
                    }
                }
                &SpriteCell::Color(color) => ExtractedSprite {
                    entity,
                    color,
                    transform,
                    rect: None,
                    custom_size: Some(sprite_grid.cell_size),
                    flip_x: false,
                    flip_y: false,
                    image_handle_id: DEFAULT_IMAGE_HANDLE.id,
                    anchor: Vec2::ZERO,
                },
            };
            extracted_sprites.sprites.alloc().init(extracted_sprite);
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
            render_app.add_system_to_stage(
                RenderStage::Extract,
                extract_grid_sprites
                    .label(SpriteGridRenderSystem::ExtractTiles)
                    .after(SpriteSystem::ExtractSprites),
            );
        }
    }
}
