use bevy::math::vec2;
use bevy::prelude::*;
use bevy_sprite_grid::prelude::*;

fn spawn_grid(mut commands: Commands) {
    let cell_size = vec2(100.0, 100.0);
    let mut sprite_grid = SpriteGrid::empty(([2, 2], cell_size));
    sprite_grid[[0, 0]] = Some(SpriteCell::color(Color::WHITE));
    sprite_grid[[1, 0]] = Some(SpriteCell::color(Color::RED));
    sprite_grid[[0, 1]] = Some(SpriteCell::color(Color::GREEN));
    sprite_grid[[1, 1]] = Some(SpriteCell::color(Color::BLUE));
    commands.spawn_bundle(SpriteGridBundle {
        sprite_grid,
        ..Default::default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SpriteGridPlugin)
        .add_startup_system(|mut commands: Commands| {
            commands.spawn_bundle(Camera2dBundle::default());
        })
        .add_startup_system(spawn_grid)
        .run();
}
