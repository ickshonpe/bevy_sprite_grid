use std::ops::Mul;
use bevy::math::vec2;
use bevy::prelude::*;
use crate::prelude::*;

pub fn pick_cell(
    grid: &SpriteGrid,
    transform: &GlobalTransform,
    point: Vec2,
) -> Option<[usize; 2]>{
    let alignment_translation = -grid.alignment.0 * grid.grid_size();
    let grid_transform =
        transform.mul(
        Transform {
            translation: alignment_translation.extend(0.0),
            ..Default::default()
        });
    let m = grid_transform.compute_matrix();
    let grid_point = m.inverse().transform_point3(point.extend(0.0)).truncate();
    if 0.0 <= grid_point.x && grid_point.x < grid.grid_size().x
    && 0.0 <= grid_point.y && grid_point.y < grid.grid_size().y {
        let cell = grid_point / grid.cell_size;
        let result = [cell.x as usize, cell.y as usize];
        result.into()
    } else {
        None
    }
}

pub fn pick_cell_unbounded(
    grid: &SpriteGrid,
    transform: &GlobalTransform,
    point: Vec2,
) -> [i64; 2]{
    let alignment_translation = -grid.alignment.0 * grid.grid_size();
    let grid_transform =
        transform.mul(
        Transform {
            translation: alignment_translation.extend(0.0),
            ..Default::default()
        });
    let m = grid_transform.compute_matrix();
    let grid_point = m.inverse().transform_point3(point.extend(0.0)).truncate();
    let cell = grid_point / grid.cell_size;
    let result = [cell.x.floor() as i64, cell.y.floor() as i64];
  
    result
}

pub fn pick_rect(
    grid: &SpriteGrid,
    transform: &GlobalTransform,
    rect_half_size: Vec2,
    rect_transform: &GlobalTransform,
) -> Option<[[usize; 2]; 2]> {
    if grid.x_len == 0 || grid.y_len == 0 {
        return None;
    }
    let cell_indices = [
        -rect_half_size,
        rect_half_size * vec2(-1.0, 1.0),
        rect_half_size,
        rect_half_size * vec2(1.0, -1.0),
    ]
    .map(|pos| {
        rect_transform.mul_vec3(pos.extend(0.0))
    })
    .map(|pos| {
        pick_cell_unbounded(grid, transform, pos.truncate())
    });
    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;
    let mut min_y = i64::MAX;
    let mut max_y = i64::MIN;
    for [x, y] in cell_indices {
        if x < min_x { min_x = x };
        if max_x < x { max_x = x };
        if y < min_y { min_y = y };
        if max_y < y { max_y = y };
    }
    
    if max_x < 0 { return None }
    if max_y < 0 { return None }
    if grid.x_len as i64 <= min_x { return None }
    if grid.y_len as i64 <= min_y { return None }
    let xs = [
        min_x.max(0) as usize,
        (max_x as usize).clamp(0, grid.x_len - 1)
    ];
    let ys = [
        min_y.max(0) as usize,
        (max_y as usize).clamp(0, grid.y_len - 1)
    ];
    Some([xs, ys])
}