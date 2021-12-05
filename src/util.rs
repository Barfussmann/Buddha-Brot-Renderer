use super::cell::*;
use super::mandel_iter::*;
pub use glam::dvec2 as vec2;
pub use glam::DVec2 as Vec2;
pub use macroquad::color::Color;
pub use macroquad::color::*;
pub use rand::prelude::{Rng, ThreadRng};

pub fn gen_point_in_square(corner: Vec2, side_length: f64, rng: &mut ThreadRng) -> Vec2 {
    let offset = vec2(
        rng.gen_range(0. ..side_length),
        rng.gen_range(0. ..side_length),
    );
    corner - offset
}
#[target_feature(enable = "avx2")]
#[target_feature(enable = "fma")]
pub unsafe fn raw_quad_inside_test(cell: Cell, limit: usize, grid_size: usize, rng: &mut ThreadRng) -> ([bool;4], [bool;4]) {
    let mut x = [0.; 4];
    let mut y = [0.; 4];
    for i in 0..4 {
        let point = cell.gen_point_inside(grid_size, rng);
        x[i] = point.x;
        y[i] = point.y;
    }
    let mut multi_mandel_iterator = MultiMandelIterator::new(x, y);
    for _ in 0..limit {
        multi_mandel_iterator.next_iteration();
    }
    let limit_is_inside = multi_mandel_iterator.is_in_set();
    for _ in 0..1024 {
        multi_mandel_iterator.next_iteration();
    }
    let set_limit_is_inside = multi_mandel_iterator.is_in_set();
    (limit_is_inside, set_limit_is_inside)
}
pub fn quad_inside_test(cell: Cell, limit: usize, grid_size: usize, rng: &mut ThreadRng) -> bool {
    let (inside_limit, inside_set) = unsafe{raw_quad_inside_test(cell, limit, grid_size, rng)};
    for (limit_is_inside, set_limit_is_inside) in
        inside_limit.iter().zip(inside_set.iter())
    {
        if *limit_is_inside && !set_limit_is_inside {
            return true;
        }
    }
    return false;
}
pub fn four_point_inside_tests(cells: [Cell; 4], limit: usize, grid_size: usize, rng: &mut ThreadRng) -> [bool; 4] {
    let inside_points = [
        cells[0].gen_point_inside(grid_size, rng),
        cells[1].gen_point_inside(grid_size, rng),
        cells[2].gen_point_inside(grid_size, rng),
        cells[3].gen_point_inside(grid_size, rng),
    ];
    let x = [
        inside_points[0].x,
        inside_points[1].x,
        inside_points[2].x,
        inside_points[3].x,
    ];
    let y = [
        inside_points[0].y,
        inside_points[1].y,
        inside_points[2].y,
        inside_points[3].y,
    ];
    let mut mandel_iter = MultiMandelIterator::new(x, y);
    for _ in 0..limit {
        mandel_iter.next_iteration();
    }
    let limit_is_inside = mandel_iter.is_in_set();
    for _ in 0..1024 {
        mandel_iter.next_iteration();
    }
    let set_limit_is_inside = mandel_iter.is_in_set();
    [
        limit_is_inside[0] && ! set_limit_is_inside[0],
        limit_is_inside[1] && ! set_limit_is_inside[1],
        limit_is_inside[2] && ! set_limit_is_inside[2],
        limit_is_inside[3] && ! set_limit_is_inside[3],
    ]
}
