use core_simd::*;
use glam::DVec2;
// https://en.wikipedia.org/wiki/Plotting_algorithms_for_the_Mandelbrot_set#Optimized_escape_time_algorithms

pub fn iterat_points(x: [f64; 4], y: [f64; 4], max_iterations: usize) -> i64x4 {
    let zero = f64x4::splat(0.0);
    let mut z_x = zero;
    let mut z_y = zero;
    let mut z_squared_x = zero;
    let mut z_squared_y = zero;
    let c_x = f64x4::from_array(x);
    let c_y = f64x4::from_array(y);
    let mut iterations = i64x4::splat(0);
    for _ in 0..max_iterations {
        z_y = 2. * z_x * z_y + c_y;
        z_x = z_squared_x - z_squared_y + c_x;
        z_squared_x = z_x * z_x;
        z_squared_y = z_y * z_y;
        let abs = z_squared_x + z_squared_y;
        let new_iterations = abs.lanes_le(f64x4::splat(4.)).to_int();
        iterations -= new_iterations; // lane is -1 when abs < 4
    }
    iterations
}
pub fn iterate_points_dvec2(points: &[DVec2; 4], max_interations: usize) -> [i64; 4] {
    let x = [points[0].x, points[1].x, points[2].x, points[3].x];
    let y = [points[0].y, points[1].y, points[2].y, points[3].y];
    iterat_points(x, y, max_interations).to_array()
}
