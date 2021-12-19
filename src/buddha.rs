use super::camera::*;
use core::simd::*;
use glam::DVec2;

use super::{HEIGHT, WIDTH};

pub struct Buddha {
    z_x: f64x4,
    z_y: f64x4,
    z_squared_x: f64x4,
    z_squared_y: f64x4,
    c_x: f64x4,
    c_y: f64x4,
    iteration: u64x4,
    max_iteration: u64x4,
    x_lower_bound: f64x4,
    y_lower_bound: f64x4,
    x_upper_bound: f64x4,
    y_upper_bound: f64x4,
    x_screen_offset: f64x4,
    y_screen_offset: f64x4,
    screen_scale: f64x4,
    width: i64x4,
    pixels: Vec<u32>,
    samples: Vec<DVec2>,
    sample_index: usize,
    value_to_replace: mask64x4,
}

#[allow(unused_variables)]
impl Buddha {
    pub fn new(samples: Vec<DVec2>, max_iterations: u64, view_rect: ViewRect) -> Self {
        let zero = f64x4::splat(0.);
        let lower_bound = view_rect.top_left_corner;    
        let upper_bound = view_rect.get_bottom_right_corner();
        let screen_offset = lower_bound;
        let screen_scale = view_rect.get_screen_scale().x;
        Self {
            z_x: zero,
            z_y: zero,
            z_squared_x: zero,
            z_squared_y: zero,
            iteration: u64x4::splat(0),
            max_iteration: u64x4::splat(max_iterations),
            c_x: zero,
            c_y: zero,
            x_lower_bound: f64x4::splat(lower_bound.x),
            y_lower_bound: f64x4::splat(lower_bound.y),
            x_upper_bound: f64x4::splat(upper_bound.x),
            y_upper_bound: f64x4::splat(upper_bound.y),
            x_screen_offset: f64x4::splat(screen_offset.x),
            y_screen_offset: f64x4::splat(screen_offset.y),
            screen_scale: f64x4::splat(screen_scale),
            width: i64x4::splat(WIDTH as i64),
            pixels: vec![0; WIDTH * HEIGHT],
            samples,
            sample_index: 0,
            value_to_replace: mask64x4::from_array([true, false, false, false]),
        }
    }
    fn iterate_samples(&mut self) {
        while self.sample_index < self.samples.len() {
            self.iterate();
        }
    }
    fn iterate(&mut self) {
        let zero = f64x4::splat(0.);
        let dummy = zero;

        // computes next interation and return if they are still inside
        let inside = {
            self.z_y = f64x4::splat(2.) * self.z_x * self.z_y + self.c_y;
            self.z_x = self.z_squared_x - self.z_squared_y + self.c_x;
            self.z_squared_x = self.z_x * self.z_x;
            self.z_squared_y = self.z_y * self.z_y;
            self.iteration += u64x4::splat(1);
            let abs = self.z_squared_x + self.z_squared_y;

            abs.lanes_le(f64x4::splat(4.))
        };

        // replace outside samples or to old samples
        // doesn't have to run every loop
        {
            let value_to_replace =
                self.value_to_replace & (inside | self.iteration.lanes_ge(self.max_iteration));
            // hacky method to rotate.
            self.value_to_replace = unsafe {
                mask64x4::from_int_unchecked(
                    self.value_to_replace.to_int().rotate_lanes_left::<1>(),
                )
            };
            let false_to_keep = self.value_to_replace & inside;
            let new_sample = self.samples[self.sample_index];
            // increase index when sample was used
            self.sample_index += false_to_keep.any() as usize;
            let new_x = f64x4::splat(new_sample.x);
            let new_y = f64x4::splat(new_sample.y);
            self.z_y = false_to_keep.select(new_x, self.z_x);
            self.z_y = false_to_keep.select(new_y, self.z_y);
            self.iteration = false_to_keep.select(u64x4::splat(0), self.iteration);
        }

        // compute index of pixel. 0 when outside.
        let index = {
            let x_screen = (self.z_x + self.x_screen_offset) * self.screen_scale;
            let y_screen = (self.z_y + self.y_screen_offset) * self.screen_scale;

            let x_inside =
                self.z_x.lanes_ge(self.x_lower_bound) & self.z_x.lanes_le(self.x_upper_bound);
            let y_inside =
                self.z_y.lanes_ge(self.y_lower_bound) & self.z_y.lanes_le(self.y_upper_bound);
            let both_inside = x_inside & y_inside;
            let inside_x = both_inside.select(x_screen, zero);
            let inside_y = both_inside.select(y_screen, zero);
            let x_index = unsafe { inside_x.to_int_unchecked() };
            let y_index = unsafe { inside_y.to_int_unchecked() };
            x_index + y_index * self.width
        };

        for index in index.to_array() {
            self.pixels[index as usize] += 1;
        }
    }
    fn normalise_pixels(&self) -> Vec<u8> {
        let max = self.pixels.iter().max().unwrap();
        let mul = u32::MAX.checked_div(*max).unwrap_or_default();
        self.pixels
            .iter()
            .map(|pixel| ((pixel * mul) >> 24) as u8)
            .collect()
    }
}

impl Updateable for Buddha {
    fn update(&mut self) {
        for _ in 0..1 {
            self.iterate();
        }
    }
    fn draw(&mut self, drawer: &mut Drawer) {
        drawer.draw_raw_pixels(self.normalise_pixels());
    }
}
