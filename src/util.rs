pub use glam::dvec2 as vec2;
pub use glam::DVec2 as Vec2;
pub use macroquad::color::*;
pub use rand::prelude::{Rng, ThreadRng};
use super::grid_bound::Cell;
use core_simd::*;

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

struct MultiMandelIterator {
    z_x: f64x4,
    z_y: f64x4,
    z_squared_x: f64x4,
    z_squared_y: f64x4,
    c_x: f64x4,
    c_y: f64x4,
    iteration: usize,
}
impl MultiMandelIterator {
    fn new(x: [f64; 4], y: [f64; 4]) -> Self {
        let z_x = f64x4::from_array(x);
        let z_y = f64x4::from_array(y);
        Self {
            z_x,
            z_y,
            z_squared_x: z_x * z_x,
            z_squared_y: z_y * z_y,
            c_x: z_x,
            c_y: z_y,
            iteration: 1,
        }
    }
    fn next_iteration(&mut self) {

        self.z_y = 2. * self.z_x * self.z_y + self.c_y;
        self.z_x = self.z_squared_x - self.z_squared_y + self.c_x;
        self.z_squared_x = self.z_x * self.z_x;
        self.z_squared_y = self.z_y * self.z_y;
        self.iteration += 1;
    }
    fn is_in_set(&self) -> [bool; 4] {
        let abs = self.z_squared_x + self.z_squared_y;
        abs.lanes_le(f64x4::splat(4.)).to_array()
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
    for _ in 0..1024 {
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn multi_mandel_iterator_same_to_mandel_iterator() {
        let x = [0.3, 1.1, 1.0, 1.0];
        let y = [0.1, 1.2, 1.1, -1.0];
        let mut multi_mandel_iterator = MultiMandelIterator::new(x, y);
        let mut mandel_iterators = Vec::new();
        for (x, y) in x.iter().zip(y.iter()) {
            mandel_iterators.push(MandelIterator::new(vec2(*x, *y)));
        }
        for _ in 0..1000 {
            multi_mandel_iterator.next_iteration();
            for mandel_iterator in mandel_iterators.iter_mut() {
                mandel_iterator.next_iteration();
            }
            let x = multi_mandel_iterator.z_x.to_array();
            let y = multi_mandel_iterator.z_y.to_array();
            for ((x, y), mandel_iterator) in x.iter().zip(y.iter()).zip(mandel_iterators.iter()) {
                let x_ordering = x.partial_cmp(&mandel_iterator.z.x).unwrap_or(std::cmp::Ordering::Equal);
                let y_ordering = y.partial_cmp(&mandel_iterator.z.y).unwrap_or(std::cmp::Ordering::Equal);
                assert_eq!(x_ordering, std::cmp::Ordering::Equal);
                assert_eq!(y_ordering, std::cmp::Ordering::Equal);
            }
        }
        
    }
}
