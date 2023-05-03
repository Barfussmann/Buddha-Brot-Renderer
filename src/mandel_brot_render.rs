use crate::{camera::Updateable, iterat_point::{iterations_to_color, PointIter, TaylorApproximation, iterat_point}};

use super::camera::ViewRect;
use super::mandel_iter::*;
use glam::{dvec2, DVec2 as Vec2};
use num_complex::Complex;
use rayon::prelude::*;

use super::{HEIGHT, WIDTH};

pub struct MandelbrotRender {
    view_rect: ViewRect,
    pixels: Vec<u32>,
    pixel_cords: (Vec<f64>, Vec<f64>),
}
impl MandelbrotRender {
    pub fn new() -> Self {
        let mut me = Self {
            view_rect: ViewRect::default(),
            pixels: vec![0; WIDTH * HEIGHT],
            pixel_cords: (vec![0.; WIDTH * HEIGHT], vec![0.; WIDTH * HEIGHT]),
        };
        me.calculate_pixel_cords();
        me.update_pixels();
        me
    }
    fn calculate_pixel_cords(&mut self) {
        let delta_pixel = self.view_rect.view_size / Vec2::new(WIDTH as f64, HEIGHT as f64);
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let index = y * WIDTH + x;
                let cords =
                    self.view_rect.top_left_corner + Vec2::new(x as f64, y as f64) * delta_pixel;
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

            let start_pos = (view_rect.top_left_corner + view_rect.get_bottom_right_corner()) / 2.;

            let vec_pos = start_pos;
            self.pixels[view_rect.screen_index(start_pos)] = 0xff0000; // Red

            let mut pos = Complex::new(vec_pos.x, vec_pos.y);
            for _ in 0..10 {

                let mut iterator = PointIter::new(pos, 100);
                iterator.iterate();
                let values = iterator.values();
                let taylor_z = TaylorApproximation::<7>::new(pos ,values[..7].try_into().unwrap());
                let taylor_d_z = TaylorApproximation::<7>::new(pos, values[1..8].try_into().unwrap());
                

                for i in 0..15 {
                    
                    let direction = taylor_z.approximate_point(pos) / taylor_d_z.approximate_point(pos);
                    
                    pos -= direction / 10.;
                    let vec_pos = Vec2::new(pos.re, pos.im);

                    if view_rect.is_visable(vec_pos) {
                        let index = view_rect.screen_index(vec_pos);
                        if index < self.pixels.len() {
                            self.pixels[index] = 0x00ff00; // Green
                        }
                    }
                }
            }
            let mut pos = start_pos;

            for _ in 0..2000 {
                let (z, d_z, _) = iterat_point(pos, 1000);
                let i_z = Complex::new(z.x, z.y);
                let i_d_z = Complex::new(d_z.x, d_z.y);
                let i_direction = i_z / i_d_z;
                let direction = Vec2::new(i_direction.re, i_direction.im);
                pos -= direction / 10.;
                if view_rect.is_visable(pos) {
                    let index = view_rect.screen_index(pos);
                    if index < self.pixels.len() {
                        self.pixels[index] = 0xff0000; // Red
                    }
                }
            }
        }
    }
    pub fn get_raw_pixels(&mut self, new_view_rect: ViewRect) -> Vec<u32> {
        self.set_camera_rect(new_view_rect);
        self.pixels.clone()
    }
}

impl Updateable for MandelbrotRender {
    fn draw(&mut self, view: ViewRect) -> Vec<u32> {
        self.get_raw_pixels(view)
    }
    fn update(&mut self) {
        // self.update_pixels();
    }
    fn update_view_rect(&mut self, _view_rect: ViewRect) {}
}
