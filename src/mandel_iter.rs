use core_simd::*;
use glam::DVec2 as Vec2;
// https://en.wikipedia.org/wiki/Plotting_algorithms_for_the_Mandelbrot_set#Optimized_escape_time_algorithms

pub struct MandelIterator {
    z: Vec2,
    z_squared: Vec2,
    c: Vec2,
    pub iteration: usize,
}
impl MandelIterator {
    pub fn new(starting_point: Vec2) -> MandelIterator {
        MandelIterator {
            z: starting_point,
            z_squared: starting_point * starting_point,
            c: starting_point,
            iteration: 1,
        }
    }
    pub fn next_iteration(&mut self) {
        self.z.y = 2. * self.z.x * self.z.y + self.c.y;
        self.z.x = self.z_squared.x - self.z_squared.y + self.c.x;
        self.z_squared = self.z * self.z;
        self.iteration += 1;
    }
    pub fn is_in_set(&self) -> bool {
        self.z_squared.x + self.z_squared.y < 4.
    }
}

pub struct MultiMandelIterator {
    z_x: f64x4,
    z_y: f64x4,
    z_squared_x: f64x4,
    z_squared_y: f64x4,
    c_x: f64x4,
    c_y: f64x4,
    iteration: i64x4,
}
impl MultiMandelIterator {
    pub fn new(x: [f64; 4], y: [f64; 4]) -> Self {
        let z_x = f64x4::from_array(x);
        let z_y = f64x4::from_array(y);
        Self {
            z_x,
            z_y,
            z_squared_x: z_x * z_x,
            z_squared_y: z_y * z_y,
            c_x: z_x,
            c_y: z_y,
            iteration: i64x4::splat(1),
        }
    }
    #[inline(always)]
    fn next_iteration(&mut self) {
        self.z_y = 2. * self.z_x * self.z_y + self.c_y;
        self.z_x = self.z_squared_x - self.z_squared_y + self.c_x;
        self.z_squared_x = self.z_x * self.z_x;
        self.z_squared_y = self.z_y * self.z_y;
        let abs = self.z_squared_x + self.z_squared_y;
        let new_iterations = abs.lanes_le(f64x4::splat(4.)).to_int();
        self.iteration -= new_iterations; // lane is -1 when abs < 4
    }
    // returns none when no points is inside otherwise returns corresponding bools
    pub fn is_inside(&self, limit: usize, iteration_depth: usize) -> Option<[bool; 4]> {
        let over_limit = self.iteration.lanes_ge(i64x4::splat(limit as i64));
        let under_set_limit = self.iteration.lanes_lt(i64x4::splat(iteration_depth as i64));
        let in_set = over_limit.to_int().lanes_eq(under_set_limit.to_int());
        if in_set.any() {
            Some(in_set.to_array())
        } else {
            None
        }
    }
    pub fn get_iterations(&self) -> [i64; 4] {
        self.iteration.to_array()
    }
    #[target_feature(enable = "fma")]
    #[target_feature(enable = "avx2")]
    pub unsafe fn iterate(&mut self, iteration_depth: usize) {
        for _ in 0..iteration_depth {
            self.next_iteration();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::dvec2 as vec2;

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
