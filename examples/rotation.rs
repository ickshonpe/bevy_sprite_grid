use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy_sprite_grid::prelude::*;

#[derive(Component)]
struct Center;

fn spawn_grid(mut commands: Commands) {
    let center = commands
        .spawn_bundle(SpatialBundle::default())
        .insert(Center)
        .id();
    let cell_size = vec2(32.0, 32.0);
    for (alignment, color) in [
        (SpriteGridAlignment::bottom_left(), Color::RED),
        (SpriteGridAlignment::bottom_right(), Color::YELLOW),
        (SpriteGridAlignment::top_left(), Color::GREEN),
        (SpriteGridAlignment::top_right(), Color::BLUE),
    ] {
        let sprite_grid = SpriteGrid::from_fn(([5, 5], cell_size, alignment), |[x, y]| {
            if (x + y) % 2 == 0 { color } else { Color::NAVY }.into()
        });

        let sprite_grid = commands
            .spawn_bundle(SpriteGridBundle {
                sprite_grid,
                ..Default::default()
            })
            .id();
        commands.entity(center).add_child(sprite_grid);
    }
}

fn rotate(time: Res<Time>, mut query: Query<&mut Transform, With<Center>>) {
    let rotation = 0.125 * std::f32::consts::PI * time.delta_seconds();
    query.for_each_mut(|mut transform| {
        transform.rotation = transform.rotation.mul_quat(Quat::from_rotation_z(rotation));
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SpriteGridPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(|mut commands: Commands| {
            commands.spawn_bundle(Camera2dBundle::default());
        })
        .add_startup_system(spawn_grid)
        .add_system(rotate)
        .run();
}
