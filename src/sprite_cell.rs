use bevy::prelude::*;

#[derive(Clone, Debug)]
pub enum TextureSource {
    Image { handle: Handle<Image> },
    Atlas { handle: Handle<TextureAtlas>, index: usize },
}

impl Default for TextureSource {
    fn default() -> Self {
        Self::Image { handle: Default::default() }
    }
}

impl From<Handle<Image>> for TextureSource {
    fn from(handle: Handle<Image>) -> Self {
        Self::Image { handle }
    }
}

impl From<(Handle<TextureAtlas>, usize)> for TextureSource {
    fn from((handle, index): (Handle<TextureAtlas>, usize)) -> Self {
        Self::Atlas { handle, index }
    }
}

#[derive(Clone, Debug, Default)]
pub struct TexturedCell {
    pub texture: TextureSource,
    pub color: Color,
    pub flip_x: bool,
    pub flip_y: bool,
    pub custom_size: Option<Vec2>,
}

impl TexturedCell {
    fn new(texture: impl Into<TextureSource>) -> Self {
        Self {
            texture: texture.into(),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug)]
#[derive(Component)]
pub enum SpriteCell {
    Texture(TexturedCell),
    Color(Color),
}

impl <T> From<T> for SpriteCell where T: Into<TextureSource> {
    fn from(source: T) -> Self {
        Self::Texture(TexturedCell::new(source))
    }
}

impl From<TexturedCell> for SpriteCell {
    fn from(tile_sprite: TexturedCell) -> Self {
        SpriteCell::Texture(tile_sprite)
    }
}

impl From<Color> for SpriteCell {
    fn from(color: Color) -> Self {
        SpriteCell::Color(color)
    }
}

impl SpriteCell {
    pub fn texture(texture: impl Into<TextureSource>) -> Self {
        Self::from(texture)
    }

    pub fn color(color: Color) -> Self {
        Self::Color(color)
    }
}
