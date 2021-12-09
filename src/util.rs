use super::cell::*;
use super::mandel_iter::*;
use core_simd::i64x4;
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
pub unsafe fn four_point_inside_tests(
    cells: [Cell; 4],
    limit: usize,
    grid_size: usize,
    iteration_depth: usize,
    rng: &mut ThreadRng,
) -> (i64x4, bool) {
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
    mandel_iter.iterate(iteration_depth);

    (
        mandel_iter.raw_get_iterations(),
        mandel_iter.is_inside(limit, iteration_depth).is_some(),
    )
}
