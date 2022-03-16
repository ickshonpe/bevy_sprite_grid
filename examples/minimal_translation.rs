use bevy::prelude::*;
use bevy_sprite_grid::prelude::*;

fn spawn_grid(
    mut commands: Commands,
) {
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::NAVY,
            custom_size: Some(Vec2::splat(200.0)),
            ..Default::default()
        },
        ..Default::default()
    });
    let s = 100.0;
    let mut translation = Vec3::ZERO + 100.0 * Vec3::Z;
    for color in [Color::WHITE, Color::RED] {
        let cell_size = Vec2::splat(s);
        let mut sprite_grid = SpriteGrid::empty(([1, 1], cell_size, SpriteGridAlignment::top_right()));
        sprite_grid[[0, 0]] = Some(SpriteCell::color(color));     
        commands.spawn_bundle(SpriteGridBundle {
            transform: Transform::from_translation(translation),
            sprite_grid,
            ..Default::default()
        });
        translation += s * Vec3::X;
    }
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(SpriteGridPlugin)
    .add_startup_system(|mut commands: Commands| { commands.spawn_bundle(OrthographicCameraBundle::new_2d()); })
    .add_startup_system(spawn_grid)
    .run();
}