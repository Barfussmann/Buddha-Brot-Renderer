pub use glam::dvec2 as vec2;
pub use glam::DVec2 as Vec2;
pub use macroquad::color::*;
pub use rand::prelude::{Rng, ThreadRng};

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
struct MandelIterator {
    z: Vec2,
    z_squared: Vec2,
    c: Vec2,
    iteration: usize,
}
impl MandelIterator {
    fn new(starting_point: Vec2) -> MandelIterator {
        MandelIterator {
            z: starting_point,
            z_squared: starting_point * starting_point,
            c: starting_point,
            iteration: 1,
        }
    }
    fn next_iteration(&mut self) {
        self.z.y = 2. * self.z.x * self.z.y + self.c.y;
        self.z.x = self.z_squared.x - self.z_squared.y + self.c.x;
        self.z_squared = self.z * self.z;
        self.iteration += 1;
    }
    fn is_in_set(&self) -> bool {
        self.z_squared.x + self.z_squared.y < 4.
    }
}

// https://en.wikipedia.org/wiki/Plotting_algorithms_for_the_Mandelbrot_set#Optimized_escape_time_algorithms
/// iterates point on the mandelbrot set
///
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
