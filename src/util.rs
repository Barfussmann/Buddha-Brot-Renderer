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
// https://en.wikipedia.org/wiki/Triangle#Using_coordinates
pub fn triangle_area(p1: Vec2, p2: Vec2, p3: Vec2) -> f64 {
    0.5 * ((p2.x - p1.x) * (p3.y - p1.y) - (p3.x - p1.x) * (p2.y - p1.y)).abs()
}

pub fn gen_point_in_triangle(p1: Vec2, p2: Vec2, p3: Vec2, rng: &mut ThreadRng) -> Vec2 {
    let mut a = rng.gen::<f64>();
    let mut b = rng.gen::<f64>();

    while a + b > 1. {
        a = rng.gen::<f64>();
        b = rng.gen::<f64>();
    }

    let delta_a = vec2((p2.x - p1.x) * a, (p2.y - p1.y) * a);
    let delta_b = vec2((p3.x - p1.x) * b, (p3.y - p1.y) * b);

    let point = vec2(p1.x + delta_a.x + delta_b.x, p1.y + delta_a.y + delta_b.y);

    point
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
pub fn cycle_detection(point: Vec2, cycle_iter_limit: usize) -> Option<usize> {
    let mut mandel_iterator = MandelIterator::new(point);
    if !is_any_cycle(point) {
        return None;
    }

    'outer: while mandel_iterator.is_in_set() && mandel_iterator.iteration < MAX_ITERATIONS {
        let last_cycle_start = mandel_iterator.z;
        for _ in 0..cycle_iter_limit {
            mandel_iterator.next_iteration();
            if last_cycle_start == mandel_iterator.z {
                break 'outer;
            }
        }
    }
    if !mandel_iterator.is_in_set() || mandel_iterator.iteration >= MAX_ITERATIONS {
        return None;
    }

    let start = mandel_iterator.z;

    for i in 0..cycle_iter_limit {
        mandel_iterator.next_iteration();
        if (start - mandel_iterator.z).length_squared() < f64::EPSILON {
            // if start == mandel_iterator.c {
            return Some(i);
        }
    }
    return None;
}

pub fn is_any_cycle(point: Vec2) -> bool {
    let mut mandel_iter = MandelIterator::new(point);
    let mut derivativ = Vec2::ONE;
    while mandel_iter.is_in_set() && mandel_iter.iteration < MAX_ITERATIONS {
        derivativ = derivativ * mandel_iter.z * 2.;
        if derivativ.length_squared() < f64::EPSILON {
            return true;
        }
        mandel_iter.next_iteration();
    }
    false
}

pub fn is_cycle(point: Vec2, cycle_length: usize) -> bool {
    if !is_any_cycle(point) {
        return false;
    }
    if let Some(found_cycle) = cycle_detection(point, 100) {
        found_cycle == cycle_length
    } else {
        false
    }
}

pub fn get_cycle_boundary_point(p1: Vec2, p2: Vec2, cycle_length: usize) -> Vec2 {
    assert!(is_cycle(p1, cycle_length) != is_cycle(p2, cycle_length));
    let (mut inside, mut outside) = if is_cycle(p1, cycle_length) {
        (p1, p2)
    } else {
        (p2, p1)
    };
    let mut middle = inside.lerp(outside, 0.5);
    let mut last_middle = Vec2::new(1000., 1000.); // cant be outside of the set
    while middle != last_middle {
        if is_cycle(middle, cycle_length) {
            inside = middle;
        } else {
            outside = middle;
        }
        last_middle = middle;
        middle = inside.lerp(outside, 0.5);
    }
    middle
}

pub fn is_inside(point: &Vec2, limit: usize) -> bool {
    iterate_point(point, limit).is_none()
}

pub fn get_boundary_point(p1: Vec2, p2: Vec2, limit: usize) -> Vec2 {
    assert!(
        is_inside(&p1, limit) != is_inside(&p2, limit),
        "There is no boundary between the points"
    );

    let mut inside;
    let mut outside;
    if is_inside(&p1, limit) {
        inside = p1;
        outside = p2;
    } else {
        inside = p2;
        outside = p1;
    }
    let mut middle = inside.lerp(outside, 0.5);
    let mut last_middle = Vec2::ZERO;
    while middle != last_middle {
        if is_inside(&middle, limit) {
            inside = middle;
        } else {
            outside = middle;
        }
        last_middle = middle;
        middle = inside.lerp(outside, 0.5);
    }
    middle
}

pub trait Rotate {
    fn rotate(&self, angle: f64) -> Self;
}

impl Rotate for Vec2 {
    fn rotate(&self, angle: f64) -> Self {
        let (sin, cos) = angle.sin_cos();
        vec2(self.x * cos - self.y * sin, self.x * sin + self.y * cos)
    }
}
