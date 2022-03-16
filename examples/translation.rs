use bevy::prelude::*;
use bevy_sprite_grid::prelude::*;

fn spawn_tile_map(
    mut commands: Commands,
) {
    for (i, color) in [
         Color::RED,
        Color::YELLOW,
        Color::GREEN,
         Color::BLUE,
    ].into_iter().enumerate() {    
        let cell_size = Vec2::splat(32.0);
        let mut sprite_grid = SpriteGrid::empty(([5, 5], cell_size, SpriteGridAlignment::center()));
        for i in 0..5 {
            for j in 0..5 {
                let cell_color = if (i + j) % 2 == 0 {
                    color
                } else {
                    Color::NAVY
                };
                sprite_grid[[i, j]] = SpriteCell::color(cell_color).into();
            }
        }
        let translation = (i as f32 - 1.5) * sprite_grid.grid_size().x * Vec3::X;
        commands.spawn_bundle(SpriteGridBundle {
            transform: Transform::from_translation(translation),
            sprite_grid,
            ..Default::default()
        }); 
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