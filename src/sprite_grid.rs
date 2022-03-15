use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Range;
use bevy::math::vec2;
use bevy::prelude::*;

#[derive(Copy, Clone, Debug)]
pub enum SpriteGridCulling {
    Enabled { margin: Vec2, },
    Disabled,
}

impl Default for SpriteGridCulling {
    fn default() -> Self {
        Self::Enabled { margin: Vec2::splat(16.0) }
    }
}

/// Determines the alignment of the 
/// sprite grid in relation to it's global transform
/// and it's point of rotation.
/// By default the bottom left corner of the grid maps
/// to the SpriteGrid's position 
#[derive(Copy, Clone, Debug)]
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

pub struct SpriteGridGeometry {
    grid_size: [usize; 2],
    cell_size: Vec2,
    alignment: SpriteGridAlignment
}

impl From<([usize; 2], Vec2)> for SpriteGridGeometry {
    fn from((grid_size, cell_size): ([usize; 2], Vec2)) -> Self {
        Self {
            grid_size,
            cell_size,
            alignment: SpriteGridAlignment::default()
        }
    }
}

impl From<([usize; 2], Vec2, SpriteGridAlignment)> for SpriteGridGeometry {
    fn from((grid_size, cell_size, alignment): ([usize; 2], Vec2, SpriteGridAlignment)) -> Self {
        Self {
            grid_size,
            cell_size,
            alignment
        }
    }
}

#[derive(Clone, Default, Component)]
pub struct SpriteGrid {
    pub sprite_cells: Vec<Vec<SpriteCell>>,
    pub cell_transforms: Vec<Vec<Transform>>,
    pub alignment: SpriteGridAlignment,
    pub x_len: usize,
    pub y_len: usize,
    pub cell_size: Vec2,
    pub culling: SpriteGridCulling,
}

impl SpriteGrid {
    pub fn empty(geometry: impl Into<SpriteGridGeometry>) -> Self {
        let geometry = geometry.into();
        let [x_len, y_len] = geometry.grid_size;
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
            alignment: geometry.alignment,
            cell_transforms,
            x_len,
            y_len,
            cell_size: geometry.cell_size,
            culling: SpriteGridCulling::Enabled { margin: 1.5 * geometry.cell_size },
        }
    }

    pub fn from_cell(geometry: impl Into<SpriteGridGeometry>, sprite_cell: impl Into<SpriteCell>) -> Self {
        let geometry = geometry.into();
        let [x_len, y_len] = geometry.grid_size;
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
                alignment: geometry.alignment,
                cell_transforms,
                x_len,
                y_len,
                cell_size: geometry.cell_size,
                culling: SpriteGridCulling::Enabled { margin: 1.5 * geometry.cell_size },
            }
    }

    pub fn from_fn<I>(geometry: impl Into<SpriteGridGeometry>, mut c: impl FnMut([usize; 2]) -> I) -> Self 
    where 
        I: Into<SpriteCell>
    {
        let geometry = geometry.into();
        let [x_len, y_len] = geometry.grid_size;
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
                alignment: geometry.alignment,
                cell_transforms,
                x_len,
                y_len,
                cell_size: geometry.cell_size,
                culling: SpriteGridCulling::Enabled { margin: 1.5 * geometry.cell_size },
            }
    }

    pub fn grid_size(&self) -> Vec2 {
       vec2(self.x_len as f32, self.y_len as f32) * self.cell_size
    }
    
    pub fn set(&mut self, [x, y]: [usize; 2], cell: impl Into<SpriteCell>) {
        self.sprite_cells[x][y] = cell.into();
    }

    pub fn iter(&self, xs: Range<usize>, ys: Range<usize>) -> impl Iterator<Item=([usize; 2], &SpriteCell)> {
        xs.flat_map(move |x| ys.clone().map(move |y| ([x, y], &self[[x, y]])))
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