use std::ops::Index;
use std::ops::IndexMut;

use bevy::math::vec2;
use bevy::prelude::*;
use bevy::render::texture::DEFAULT_IMAGE_HANDLE;

#[derive(Clone, Debug, Default)]
pub struct CellSprite {
    pub image_handle: Handle<Image>,
    pub color: Color,
    pub flip_x: bool,
    pub flip_y: bool,
    pub custom_size: Option<Vec2>,
}

impl CellSprite {
    fn new(image_handle: Handle<Image>) -> Self {
        Self {
            image_handle,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct CellAtlasSprite {
    pub atlas_handle: Handle<TextureAtlas>,
    pub atlas_index: usize,
    pub color: Color,
    pub flip_x: bool,
    pub flip_y: bool,
    pub custom_size: Option<Vec2>,
}

impl CellAtlasSprite {
    fn new(atlas_handle: Handle<TextureAtlas>, atlas_index: usize) -> Self {
        Self {
            atlas_handle,
            atlas_index,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug)]
#[derive(Component)]
pub enum SpriteCell {
    Sprite(CellSprite),
    AtlasSprite(CellAtlasSprite),
    Color(Color),
    Empty
}

impl From<CellSprite> for SpriteCell {
    fn from(tile_sprite: CellSprite) -> Self {
        SpriteCell::Sprite(tile_sprite)
    }
}

impl From<CellAtlasSprite> for SpriteCell {
    fn from(tile_atlas_sprite: CellAtlasSprite) -> Self {
        SpriteCell::AtlasSprite(tile_atlas_sprite)
    }
}

impl From<Color> for SpriteCell {
    fn from(color: Color) -> Self {
        SpriteCell::Color(color)
    }
}

impl Default for SpriteCell {
    fn default() -> Self {
        SpriteCell::Empty
    }
}

impl SpriteCell {
    pub fn sprite(image_handle: Handle<Image>) -> Self {
        Self::Sprite(CellSprite::new(image_handle))
    }

    pub fn atlas_sprite(atlas_handle: Handle<TextureAtlas>, atlas_index: usize) -> Self {        
        Self::AtlasSprite(CellAtlasSprite::new(atlas_handle, atlas_index))
    }

    pub fn color(color: Color) -> Self {
        Self::Color(color)
    }
}

#[derive(Clone, Default, Component)]
pub struct SpriteGrid {
    pub sprite_cells: Vec<Vec<SpriteCell>>,
    pub cell_transforms: Vec<Vec<Transform>>,
    pub x_len: usize,
    pub y_len: usize,
    pub cell_size: Vec2,
}

impl SpriteGrid {
    pub fn empty([x_len, y_len]: [usize; 2], cell_size: Vec2) -> Self {
        let cells: Vec<Vec<SpriteCell>> =
            (0..x_len)
            .map(|_| vec![SpriteCell::Empty; y_len])
            .collect();
        let cell_transforms = 
            (0..x_len)
            .map(|_| vec![Transform::default(); y_len])
            .collect();
        Self {
            sprite_cells: cells,
            cell_transforms,
            x_len,
            y_len,
            cell_size,
        }
    }

    pub fn from_cell([x_len, y_len]: [usize; 2], cell_size: Vec2, sprite_cell: impl Into<SpriteCell>) -> Self {
        let cell = sprite_cell.into();
        let cells: Vec<Vec<SpriteCell>> =
            (0..x_len)
            .map(|_| vec![cell.clone(); y_len])
            .collect();
        let cell_transforms = 
            (0..x_len)
            .map(|_| vec![Transform::default(); y_len])
            .collect();
            Self {
                sprite_cells: cells,
                cell_transforms,
                x_len,
                y_len,
                cell_size,
            }
    }

    pub fn from_fn<I>([x_len, y_len]: [usize; 2], cell_size: Vec2, mut c: impl FnMut([usize; 2]) -> I) -> Self 
    where 
        I: Into<SpriteCell>
    {
        let cells: Vec<Vec<SpriteCell>> =
            (0..x_len)
            .map(|x| 
                (0..y_len).map(|y| c([x, y]).into()).collect()
            )
            .collect();
        let cell_transforms = 
            (0..x_len)
            .map(|_| vec![Transform::default(); y_len])
            .collect();
            Self {
                sprite_cells: cells,
                cell_transforms,
                x_len,
                y_len,
                cell_size,
            }
    }

    pub fn grid_size(&self) -> Vec2 {
       vec2(self.x_len as f32, self.y_len as f32) * self.cell_size
    }
    
    pub fn set(&mut self, [x, y]: [usize; 2], cell: impl Into<SpriteCell>) {
        self.sprite_cells[x][y] = cell.into();
    }
}

impl Index<[usize; 2]> for SpriteGrid {
    type Output=SpriteCell;

    fn index(&self, [x, y]: [usize; 2]) -> &Self::Output {
        &self.sprite_cells[x][y]
    }
}

impl IndexMut<[usize; 2]> for SpriteGrid {
    fn index_mut(&mut self, [x, y]: [usize; 2]) -> &mut Self::Output {
        &mut self.sprite_cells[x][y]
    }
}