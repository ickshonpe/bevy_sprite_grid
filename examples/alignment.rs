use bevy::math::vec2;
use bevy::prelude::*;
use bevy_sprite_grid::prelude::*;

fn spawn_tile_map(
    mut commands: Commands,
) {
    for (z, (alignment, color)) in [
        (SpriteGridAlignment::bottom_right(), Color::YELLOW),
        (SpriteGridAlignment::bottom_left(), Color::RED),
        (SpriteGridAlignment::top_left(), Color::GREEN),
        (SpriteGridAlignment::top_right(), Color::BLUE),
        (SpriteGridAlignment::center(), Color::BLACK),
    ].into_iter().enumerate() {    
        let grid_size = vec2(32.0, 32.0);
        let mut sprite_grid = SpriteGrid::empty([5, 5], grid_size);
        for i in 0..5 {
            for j in 0..5 {
                let cell_color = if (i + j + z) % 2 == 0 {
                    color
                } else {
                    Color::NAVY
                };
                sprite_grid[[i, j]] = SpriteCell::color(cell_color);
            }
        }
        commands.spawn_bundle(SpriteGridBundle {
            sprite_grid,
            transform: Transform::from_translation(z as f32 * Vec3::Z),
            ..Default::default()
        })
        .insert(alignment); 
    }
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(SpriteGridPlugin)
    .add_startup_system(|mut commands: Commands| { commands.spawn_bundle(OrthographicCameraBundle::new_2d()); })
    .add_startup_system(spawn_tile_map)
    .run();
}