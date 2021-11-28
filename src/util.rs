use super::cell::*;
use super::mandel_iter::*;
pub use glam::dvec2 as vec2;
pub use glam::DVec2 as Vec2;
pub use macroquad::color::Color;
pub use macroquad::color::*;
pub use rand::prelude::{Rng, ThreadRng};

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
pub fn draw_square(corner: Vec2, side_length: f64, color: Color) {
    macroquad::prelude::draw_rectangle(
        corner.x as f32,
        corner.y as f32,
        side_length as f32,
        side_length as f32,
        color,
    )
}
pub fn draw_rect(corner: Vec2, width: f64, heigth: f64, color: Color) {
    macroquad::prelude::draw_rectangle(
        corner.x as f32,
        corner.y as f32,
        width as f32,
        heigth as f32,
        color,
    )
}
pub fn gen_point_in_square(corner: Vec2, side_length: f64, rng: &mut ThreadRng) -> Vec2 {
    let offset = vec2(
        rng.gen_range(0. ..side_length),
        rng.gen_range(0. ..side_length),
    );
    corner - offset
}
pub fn raw_quad_inside_test(cell: Cell, limit: usize, grid_size: usize, rng: &mut ThreadRng) -> ([bool;4], [bool;4]) {
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
    for _ in 0..128 {
        multi_mandel_iterator.next_iteration();
    }
    let set_limit_is_inside = multi_mandel_iterator.is_in_set();
    (limit_is_inside, set_limit_is_inside)
}
pub fn quad_inside_test(cell: Cell, limit: usize, grid_size: usize, rng: &mut ThreadRng) -> bool {
    let (inside_limit, inside_set) = raw_quad_inside_test(cell, limit, grid_size, rng);
    for (limit_is_inside, set_limit_is_inside) in
        inside_limit.iter().zip(inside_set.iter())
    {
        if *limit_is_inside && !set_limit_is_inside {
            return true;
        }
    }
    return false;
}
