use super::mandel_iter::*;
use glam::DVec2 as Vec2;
use rayon::prelude::*;
use kludgine::core::image::*;

pub struct MandelbrotRender {
    width: usize,
    height: usize,
    top_left_corner: Vec2,
    view_size: Vec2,
    pixels: Vec<u8>,
    pixel_cords: (Vec<f64>, Vec<f64>),
}
impl MandelbrotRender {
    pub fn new() -> Self {
        let width = 1024 as usize;
        let height = (1024 as f64 * (2.64 / 3.0)) as usize;

        Self {
            width,
            height,
            top_left_corner: Vec2::ZERO,
            view_size: Vec2::ZERO,
            pixels: vec![0; width * height * 4],
            pixel_cords: (vec![0.; width * height], vec![0.; width * height]),
        }
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
    fn update_pixels(&mut self) {
        self.pixels
            .array_chunks_mut::<16>()
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
    fn set_camera_rect(&mut self, (top_left_corner, view_size): (Vec2, Vec2)) {
        self.top_left_corner = top_left_corner;
        self.view_size = view_size;
        self.calculate_pixel_cords();
        self.update_pixels();
    }
    pub fn to_image(&mut self, new_view_rect: Option<(Vec2, Vec2)>) -> RgbaImage {
        if let Some(view_rect) = new_view_rect {
            self.set_camera_rect(view_rect);
        }
        RgbaImage::from_raw(self.width as u32, self.height as u32, self.pixels.clone()).unwrap()
    }
    // pub fn draw(&mut self, new_view_rect: Option<(Vec2, Vec2)>, graphics: &mut Graphics2D) {
    //     if let Some(view_rect) = new_view_rect {
    //         self.set_camera_rect(view_rect);
    //     }
    //     let image = graphics.create_image_from_raw_pixels(
    //         speedy2d::image::ImageDataType::RGB,
    //         speedy2d::image::ImageSmoothingMode::NearestNeighbor,
    //         (self.width as u32, self.height as u32),
    //         &self.pixels,
    //     ).unwrap();
    //     graphics.draw_image((0., 0.), &image);
    // }
}
fn iterations_to_color(iterations: [i64; 4]) -> [u8; 16] {
    let mut colors = [0; 16];
    for i in 0..4 {
        let color_value = 255 - ((iterations[i] as f32).sqrt() * 15.) as u8;
        colors[i * 4] = color_value;
        colors[i * 4 + 1] = color_value;
        colors[i * 4 + 2] = color_value;
        colors[i * 4 + 3] = 255;
    }
    colors
}
