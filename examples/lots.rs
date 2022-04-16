use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy_sprite_grid::prelude::*;

#[derive(Component)]
struct Center;

fn spawn_grid(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let center = commands.spawn_bundle((
            Center,
            Transform::default(),
            GlobalTransform::default()
        )).id();
    let cell_size = vec2(8.0, 8.0);
    
    let sprite_grid = SpriteGrid::from_fn(
        ([100, 100], cell_size, SpriteGridAlignment::center()), 
        |[x, y]| TexturedCell {
            texture: asset_server.load("sprite.png").into(),
            color: if (x + y) % 2 == 0 {
                Color::GREEN
            } else {
                Color::WHITE
            },
            flip_x: false,
            flip_y: (x + y) % 2 == 0,
            custom_size: Some(cell_size),
            }.into()
    );

    let sprite_grid = commands.spawn_bundle(SpriteGridBundle {
        sprite_grid,
        ..Default::default()
    })
    .id(); 
    commands.entity(center).add_child(sprite_grid);
    
}

fn rotate(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Center>>,
) {
    let rotation = 0.125 * std::f32::consts::PI * time.delta_seconds();
    query.for_each_mut(|mut transform| {
        transform.rotation = transform.rotation.mul_quat(Quat::from_rotation_z(rotation));
    });
}

fn main() {
    App::new()
    .insert_resource(WindowDescriptor {
        present_mode: bevy::window::PresentMode::Immediate,
        mode: bevy::window::WindowMode::Fullscreen,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(SpriteGridPlugin)
    .add_plugin(LogDiagnosticsPlugin::default())
    .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
    .add_startup_system(|mut commands: Commands| { commands.spawn_bundle(OrthographicCameraBundle::new_2d()); })
    .add_startup_system(spawn_grid)
    .add_system(rotate)
    .run();
}