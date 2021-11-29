use core_simd::*;
use glam::DVec2 as Vec2;
use macroquad::color::*;
use rayon::prelude::*;

pub struct MandelbrotRender {
    width: usize,
    height: usize,
    top_left_corner: Vec2,
    view_size: Vec2,
    pixel_colors: Vec<Color>,
    pixel_cords: (Vec<f64>, Vec<f64>),
}
impl MandelbrotRender {
    pub fn new(width: usize, height: usize, top_left_corner: Vec2, width_heigth: Vec2) -> Self {
        let mut render = Self {
            width,
            height,
            top_left_corner,
            view_size: width_heigth,
            pixel_colors: vec![BLACK; width * height],
            pixel_cords: (vec![0.; width * height], vec![0.; width * height]),
        };
        render.calculate_pixel_cords();
        render
    }
    fn calculate_pixel_cords(&mut self) {
        let delta_pixel = self.view_size / Vec2::new(self.width as f64, self.height as f64);
        for x in 0..self.width {
            for y in 0..self.height {
                let index = y * self.width + x;
                let cords = self.top_left_corner + Vec2::new(x as f64, y as f64) * delta_pixel;
                self.pixel_cords.0[index] = cords.x;
                self.pixel_cords.1[index] = cords.y;
            }
        }
    }
    pub fn update_pixels(&mut self) {
        self.pixel_colors
            .array_chunks_mut::<4>()
            .zip(
                self.pixel_cords
                    .0
                    .array_chunks::<4>()
                    .zip(self.pixel_cords.1.array_chunks::<4>()),
            )
            .par_bridge()
            .for_each(|(pixel_colors, (x, y))| {
                *pixel_colors = get_color(iterat_points(*x, *y));
            });
    }
    pub fn get_colors(&self) -> &[Color] {
        &self.pixel_colors
    }
    pub fn set_camera_rect(&mut self, (top_left_corner, view_size): (Vec2, Vec2)) {
        let old_top_left = self.top_left_corner;
        let old_width_heigth = self.view_size;
        self.top_left_corner = top_left_corner;
        self.view_size = view_size;
        if self.top_left_corner != old_top_left || self.view_size != old_width_heigth {
            println!("recalculate");
            self.calculate_pixel_cords();
            self.update_pixels();
        }
    }
}
fn get_color(iterations: [i64; 4]) -> [Color; 4] {
    let mut colors = [BLACK; 4];
    for i in 0..4 {
        colors[i] = if iterations[i] != 129 {
            let r = (iterations[i] as f32 * 0.5).sin() * 0.5 + 0.5;
            let g = (iterations[i] as f32 * 0.5).cos() * 0.5 + 0.5;
            let b = (iterations[i] as f32 * 0.5).tan() * 0.5 + 0.5;
            Color::new(r, g, b, 1.)
        } else {
            Color::new(0.2, 0.2, 0.2, 1.)
        };
    }
    colors
}

fn iterat_points(x: [f64; 4], y: [f64; 4]) -> [i64; 4] {
    let mut iterator = MultiMandelIterator::new(x, y);
    unsafe {
        iterator.iterate()
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
    // #[target_feature(enable = "avx2")]
    unsafe fn next_iteration(&mut self) {
        self.z_y = 2. * self.z_x * self.z_y + self.c_y;
        self.z_x = self.z_squared_x - self.z_squared_y + self.c_x;
        self.z_squared_x = self.z_x * self.z_x;
        self.z_squared_y = self.z_y * self.z_y;
        let abs = self.z_squared_x + self.z_squared_y;
        let new_iterations = abs.lanes_le(f64x4::splat(4.)).to_int();
        self.iteration -= new_iterations; // lane is -1 when abs < 4
    }
    fn get_iterations(&self) -> [i64; 4] {
        self.iteration.to_array()
    }
    #[target_feature(enable = "fma")]
    #[target_feature(enable = "avx2")]
    unsafe fn iterate(&mut self) -> [i64; 4] {
        for _ in 0..128 {
            self.next_iteration();
        }
        self.get_iterations()
    }
}
