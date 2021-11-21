pub use glam::dvec2 as vec2;
pub use glam::DVec2 as Vec2;
pub use macroquad::color::*;
pub use rand::prelude::{Rng, ThreadRng};
use super::grid_bound::Cell;
use super::mandel_iter::*;
pub const MAX_ITERATIONS: usize = 100_000;

pub fn draw_point(point: &Vec2, color: Color, line_width: f32) {
    macroquad::prelude::draw_line(
        point.x as f32,
        point.y as f32,
        point.x as f32 + line_width,
        point.y as f32,
        line_width,
        color,
    );
}
pub fn draw_line(p1: &Vec2, p2: &Vec2, color: Color, line_width: f32) {
    macroquad::prelude::draw_line(
        p1.x as f32,
        p1.y as f32,
        p2.x as f32,
        p2.y as f32,
        line_width,
        color,
    );
}
pub fn draw_square(center: Vec2, side_length: f64, color: Color) {
    macroquad::prelude::draw_rectangle(
        (center.x - side_length / 2.0) as f32,
        (center.y - side_length / 2.0) as f32,
        side_length as f32,
        side_length as f32,
        color,
    )
}
pub fn gen_point_in_square(center: Vec2, side_length: f64, rng: &mut ThreadRng) -> Vec2 {
    let offset = vec2(
        rng.gen_range((-side_length / 2.)..(side_length / 2.)),
        rng.gen_range((-side_length / 2.)..(side_length / 2.)),
    );
    center + offset
}

/// returns number of iterations. None when bigger then limit
pub fn iterate_point(point: &Vec2, limit: usize) -> Option<usize> {
    let mut mandel_iterator = MandelIterator::new(point.clone());

    while mandel_iterator.is_in_set() && mandel_iterator.iteration < limit {
        mandel_iterator.next_iteration();
    }
    if mandel_iterator.iteration == limit {
        None
    } else {
        Some(mandel_iterator.iteration)
    }
}
pub fn is_inside(point: &Vec2, limit: usize) -> bool {
    if let Some(iterations) = iterate_point(point, 10000) {
        iterations >= limit
    } else {
        false
    }
}
pub fn quad_inside_test(cell: Cell, limit: usize, rng: &mut ThreadRng) -> bool {
    let mut x = [0.; 4];
    let mut y = [0.; 4];
    for i in 0..4 {
        let point = cell.gen_point_inside(rng);
        x[i] = point.x;
        y[i] = point.y;
    }
    let mut multi_mandel_iterator = MultiMandelIterator::new(x, y);
    for _ in 0..limit {
        multi_mandel_iterator.next_iteration();
    }
    let limit_is_inside = multi_mandel_iterator.is_in_set();
    for _ in 0..128 {
        multi_mandel_iterator.next_iteration();
    }
    let set_limit_is_inside = multi_mandel_iterator.is_in_set();
    for (limit_is_inside, set_limit_is_inside) in limit_is_inside.iter().zip(set_limit_is_inside.iter()) {
        if *limit_is_inside && !set_limit_is_inside {
            return true;
        }
    }
    return false;
}