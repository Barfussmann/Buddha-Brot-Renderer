use super::camera::CameraManger;
use super::mandel_iter::*;
use glam::DVec2 as Vec2;
use speedy2d::Graphics2D;
use rayon::prelude::*;

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
            pixels: vec![0; width * height * 3],
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
            .array_chunks_mut::<12>()
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
    pub fn draw(&mut self, camera: &CameraManger, graphics: &mut Graphics2D) {
        if camera.had_change() {
            self.set_camera_rect(camera.get_view_rect())
        }
        let image = graphics.create_image_from_raw_pixels(
            speedy2d::image::ImageDataType::RGB,
            speedy2d::image::ImageSmoothingMode::NearestNeighbor,
            (self.width as u32, self.height as u32),
            &self.pixels,
        ).unwrap();
        graphics.draw_image((0., 0.), &image);
    }
}
fn iterations_to_color(iterations: [i64; 4]) -> [u8; 12] {
    let mut colors = [0; 12];
    for i in 0..4 {
        let color_value = 255 - ((iterations[i] as f32).sqrt() * 15.) as u8;
        colors[i * 3] = color_value;
        colors[i * 3 + 1] = color_value;
        colors[i * 3 + 2] = color_value;
    }
    colors
}
