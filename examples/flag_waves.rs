use bevy::prelude::*;
use bevy_sprite_grid::prelude::*;

fn spawn_grid(
    mut commands: Commands,
) {
    let s = 50.0;
    let cell_size = Vec2::splat(s);

    for t in [
        Vec3::ZERO,
        Vec3::X,
        -Vec3::X,
        Vec3::Y,
        -Vec3::Y,
    ] {
        let mut sprite_grid = SpriteGrid::empty([2, 2], cell_size);
        sprite_grid[[0, 0]] = SpriteCell::solid_color(Color::WHITE);    
        sprite_grid[[1, 0]] = SpriteCell::solid_color(Color::RED);
        sprite_grid[[0, 1]] = SpriteCell::solid_color(Color::GREEN);
        sprite_grid[[1, 1]] = SpriteCell::solid_color(Color::BLUE);
        commands.spawn_bundle(SpriteGridBundle {
            sprite_grid,
            transform: Transform::from_translation(3.0 * s * t),
            ..Default::default()
        })
        .insert(SpriteGridAlignment::top_left());
    }

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::ORANGE,
            custom_size: Some(2.0 * 1.1 * cell_size),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn rotate(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<SpriteGrid>>,
) {
    let rotation = std::f32::consts::PI * time.seconds_since_startup().sin() as f32;
    query.for_each_mut(|mut transform| {
        transform.rotation = Quat::from_rotation_z(rotation);
    });
}

fn scale(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<SpriteGrid>>,
) {
    let scalar = 0.75 + 0.25 * time.seconds_since_startup().sin() as f32;
    query.for_each_mut(|mut transform| {
        transform.scale = [scalar, scalar, 1.0].into();
    });
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(SpriteGridPlugin)
    .add_startup_system(|mut commands: Commands| { commands.spawn_bundle(OrthographicCameraBundle::new_2d()); })
    .add_startup_system(spawn_grid)
    .add_system(rotate)
    .add_system(scale)
    .run();
}