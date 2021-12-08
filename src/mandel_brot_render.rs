use super::mandel_iter::*;
use glam::DVec2 as Vec2;
use macroquad::color::*;
use macroquad::prelude::{draw_texture, Image, Texture2D};
use rayon::prelude::*;

pub struct MandelbrotRender {
    width: usize,
    height: usize,
    top_left_corner: Vec2,
    view_size: Vec2,
    pixel_colors: Vec<Color>,
    pixel_cords: (Vec<f64>, Vec<f64>),
    image: Image,
    texture: Texture2D,
}
impl MandelbrotRender {
    pub fn new(width: usize, height: usize, top_left_corner: Vec2, width_heigth: Vec2) -> Self {
        let image = Image::gen_image_color(width as u16, height as u16, WHITE);

        let mut render = Self {
            width,
            height,
            top_left_corner,
            view_size: width_heigth,
            pixel_colors: vec![BLACK; width * height],
            pixel_cords: (vec![0.; width * height], vec![0.; width * height]),
            texture: Texture2D::from_image(&image),
            image,
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
    pub fn set_camera_rect(&mut self, (top_left_corner, view_size): (Vec2, Vec2)) {
        self.top_left_corner = top_left_corner;
        self.view_size = view_size;
        self.calculate_pixel_cords();
        self.update_pixels();
        self.image.update(&self.pixel_colors);
        self.texture.update(&self.image);
    }
    pub fn draw(&self) {
        draw_texture(self.texture, 0., 0., Color::new(1., 1., 1., 1.));
    }
}
fn get_color(iterations: [i64; 4]) -> [Color; 4] {
    let mut colors = [BLACK; 4];
    for i in 0..4 {
        let color_value = 255 - ((iterations[i] as f32).sqrt() * 15.) as u8;
        colors[i] = Color::from_rgba(color_value, color_value, color_value, 255);
    }
    colors
}

fn iterat_points(x: [f64; 4], y: [f64; 4]) -> [i64; 4] {
    let mut iterator = MultiMandelIterator::new(x, y);
    unsafe { iterator.iterate(256) };
    iterator.get_iterations()
}
