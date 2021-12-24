use super::mandel_iter::*;
use super::camera::ViewRect;
use glam::DVec2 as Vec2;
use rayon::prelude::*;

use super::{HEIGHT, WIDTH};

pub struct MandelbrotRender {
    view_rect: ViewRect,
    pixels: Vec<u32>,
    pixel_cords: (Vec<f64>, Vec<f64>),
}
impl MandelbrotRender {
    pub fn new() -> Self {
        Self {
            view_rect: ViewRect::default(),
            pixels: vec![0; WIDTH * HEIGHT],
            pixel_cords: (vec![0.; WIDTH * HEIGHT], vec![0.; WIDTH * HEIGHT]),
        }
    }
    fn calculate_pixel_cords(&mut self) {
        let delta_pixel = self.view_rect.view_size / Vec2::new(WIDTH as f64, HEIGHT as f64);
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let index = y * WIDTH + x;
                let cords = self.view_rect.top_left_corner + Vec2::new(x as f64, y as f64) * delta_pixel;
                self.pixel_cords.0[index] = cords.x;
                self.pixel_cords.1[index] = cords.y;
            }
        }
    }
    fn update_pixels(&mut self) {
        self.pixels
            .array_chunks_mut::<4>()
            .zip(
                self.pixel_cords
                    .0
                    .array_chunks::<4>()
                    .zip(self.pixel_cords.1.array_chunks::<4>()),
            )
            .par_bridge()
            .for_each(|(pixel_colors, (x, y))| {
                *pixel_colors = iterations_to_color(iterat_points(*x, *y, 256).to_array());
            });
    }
    fn set_camera_rect(&mut self, view_rect: ViewRect) {
        if self.view_rect != view_rect {
            self.view_rect = view_rect;
            self.calculate_pixel_cords();
            self.update_pixels();
        }
    }
    pub fn get_raw_pixels(&mut self, new_view_rect: ViewRect) -> Vec<u32> {
        self.set_camera_rect(new_view_rect);
        self.pixels.clone()
    }
}
fn iterations_to_color(iterations: [i64; 4]) -> [u32; 4] {
    let mut colors = [0; 4];
    for i in 0..4 {
        let color_value = 255 - ((iterations[i] as f32).sqrt() * 15.) as u32;
        colors[i] = color_value + (color_value << 8) + (color_value << 16);
    }
    colors
}
