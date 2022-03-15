use bevy::math::vec2;
use bevy::prelude::*;
use bevy_sprite_grid::prelude::*;

fn spawn_tile_map(
    mut commands: Commands,
) {
    let cell_size = vec2(32.0, 32.0);
    let sprite_grid = 
        SpriteGrid::from_fn(
            ([5, 5], cell_size, SpriteGridAlignment::center()), 
            |[x, y]| match (x + y) % 3 {
                0 => Color::RED,
                1 => Color::MAROON,
                _ => Color::WHITE,
            }
        );
    commands.spawn_bundle(SpriteGridBundle {
        sprite_grid,
        ..Default::default()
    });
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(SpriteGridPlugin)
    .add_startup_system(|mut commands: Commands| { commands.spawn_bundle(OrthographicCameraBundle::new_2d()); })
    .add_startup_system(spawn_tile_map)
    .run();
}