use bevy::math::vec2;
use bevy::prelude::*;
use bevy_sprite_grid::prelude::*;

#[derive(Component)]
struct MyCamera;

fn update_camera_indicators(
    cameras: Query<(&OrthographicProjection, &mut GlobalTransform), With<Camera2d>>,
    mut sprite: Query<(&mut Sprite, &mut Transform), With<CameraNode>>,
    mut corner: Query<
        &mut Transform,
        (
            With<CameraCornerMarker>,
            Without<CameraNode>,
            Without<CameraCenterMarker>,
        ),
    >,
    mut center: Query<
        &mut Transform,
        (
            With<CameraCenterMarker>,
            Without<CameraNode>,
            Without<CameraCornerMarker>,
        ),
    >,
) {
    let (projection, camera_transform) =
        if let Some((projection, camera_transform)) = cameras.iter().next() {
            (projection, camera_transform)
        } else {
            return;
        };

    let w = projection.right - projection.left;
    let h = projection.top - projection.bottom;

    sprite.for_each_mut(|(mut sprite, mut transform)| {
        sprite.custom_size = Some(0.5 * Vec2::from([w, h]));
        *transform = Transform::from(*camera_transform);
    });

    corner.for_each_mut(|mut transform| {
        *transform = Transform::from(*camera_transform);
        transform.translation -=
            0.5 * Vec3::new(w, h, 0.0) - 20.0 * (Vec3::X + Vec3::Y) + 250.0 * Vec3::Z;
    });

    center.for_each_mut(|mut transform| {
        *transform = Transform::from(*camera_transform);
        transform.translation.z = 250.0;
    })
}

fn move_camera(
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &GlobalTransform), With<MyCamera>>,
) {
    query.for_each_mut(|(mut transform, global_transform)| {
        let mut m = Vec3::ZERO;
        if keyboard.pressed(KeyCode::A) {
            m -= Vec3::X
        }
        if keyboard.pressed(KeyCode::D) {
            m += Vec3::X
        }
        if keyboard.pressed(KeyCode::S) {
            m -= Vec3::Y
        }
        if keyboard.pressed(KeyCode::W) {
            m += Vec3::Y
        }
        if 0.0 < m.abs().length() - 0.01 {
            transform.translation += time.delta_seconds()
                * 100.0
                * global_transform
                    .compute_transform()
                    .rotation
                    .mul_vec3(m)
                    .normalize_or_zero();
        }

        let rotation_speed = 1.0;
        if keyboard.pressed(KeyCode::Left) {
            transform.rotate(Quat::from_rotation_z(rotation_speed * time.delta_seconds()));
        } else if keyboard.pressed(KeyCode::Right) {
            transform.rotate(Quat::from_rotation_z(
                -rotation_speed * time.delta_seconds(),
            ));
        }
    });
}

#[derive(Component)]
struct CameraNode;

#[derive(Component)]
struct CameraCornerMarker;

#[derive(Component)]
struct CameraCenterMarker;
fn camera_indicators(mut commands: Commands) {
    let w = 100.0;
    let h = 100.0;

    let mut color = Color::NAVY;
    color.set_a(0.35);
    color.set_r(1.0);
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(0.5 * Vec2::from([w, h])),
                ..Default::default()
            },
            transform: Transform::from_translation(100.0 * Vec3::Z),
            ..Default::default()
        })
        .insert(CameraNode);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::splat(10.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(CameraCornerMarker);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::splat(6.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(CameraCenterMarker);
}

fn draw_grid(mut commands: Commands) {
    commands.spawn_bundle(SpriteGridBundle {
        sprite_grid: SpriteGrid::from_fn(
            (
                [1000, 1000],
                vec2(40.0, 40.0),
                SpriteGridAlignment::center(),
            ),
            |[x, y]| {
                if (x + y) % 2 == 0 {
                    Color::BLACK
                } else {
                    Color::WHITE
                }
                .into()
            },
        ),
        transform: Transform {
            ..Default::default()
        },
        ..Default::default()
    });
}

fn select_cell(
    projections: Query<
        (&GlobalTransform, &OrthographicProjection),
        (With<MyCamera>, Changed<GlobalTransform>),
    >,
    mut grids: Query<(&GlobalTransform, &mut SpriteGrid), Without<MyCamera>>,
) {
    if let Ok((camera_transform, _projection)) = projections.get_single() {
        let picked = camera_transform.compute_transform().translation.truncate();
        grids.for_each_mut(|(transform, mut sprite_grid)| {
            let _ = pick_cell_unbounded(&sprite_grid, transform, picked);
            if let Some(cell) = pick_cell(&sprite_grid, transform, picked) {
                sprite_grid[cell] = Some(Color::GREEN.into());
            }
        });
    }
}

fn select_rect(
    projections: Query<
        (&GlobalTransform, &OrthographicProjection),
        (With<MyCamera>, Changed<GlobalTransform>),
    >,
    mut grids: Query<(&GlobalTransform, &mut SpriteGrid), Without<MyCamera>>,
) {
    if let Ok((camera_transform, projection)) = projections.get_single() {
        let half_size = 0.5 * vec2(projection.right, projection.top);
        grids.for_each_mut(|(transform, mut grid)| {
            let rect = pick_rect(&grid, transform, half_size, camera_transform);
            if let Some(rect) = rect {
                for x in 0..grid.x_len {
                    for y in 0..grid.y_len {
                        grid[[x, y]] = Some(
                            if rect.xs().contains(&x) && rect.ys().contains(&y) {
                                [Color::CYAN, Color::AQUAMARINE]
                            } else {
                                [Color::ORANGE, Color::ORANGE_RED]
                            }[(x + y) % 2]
                                .into(),
                        )
                    }
                }
            }
        });
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SpriteGridPlugin)
        .add_startup_system(|mut commands: Commands| {
            commands
                .spawn_bundle(Camera2dBundle::default())
                .insert(MyCamera);
        })
        .add_startup_system(camera_indicators)
        .add_startup_system(draw_grid)
        .add_system(move_camera)
        .add_system(select_rect.before("pick"))
        .add_system(select_cell.label("pick"))
        .add_system(update_camera_indicators.after("pick"))
        .run();
}
