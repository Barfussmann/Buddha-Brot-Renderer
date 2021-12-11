use core_simd::*;
// https://en.wikipedia.org/wiki/Plotting_algorithms_for_the_Mandelbrot_set#Optimized_escape_time_algorithms


pub fn iterat_points(x: [f64; 4], y: [f64; 4], max_inarations: usize) -> i64x4 {
    let zero = f64x4::splat(0.0);
    let mut z_x = zero;
    let mut z_y = zero;
    let mut z_squared_x = zero;
    let mut z_squared_y = zero;
    let c_x = f64x4::from_array(x);
    let c_y = f64x4::from_array(y);
    let mut iterations = i64x4::splat(0);
    for _ in 0..max_inarations {
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