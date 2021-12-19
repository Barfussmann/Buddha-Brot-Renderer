use super::camera::*;
use super::covarage_grid::*;
use core::simd::*;
use glam::DVec2;

use super::{HEIGHT, WIDTH};

pub struct Buddha<'a> {
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
    x_screen_scale: f64x4,
    y_screen_scale: f64x4,
    value_to_replace: mask64x4,
    width: i64x4,
    pixels: Vec<u32>,
    sample_index: usize,
    samples: Vec<DVec2>,
    covarage_grid: &'a CovarageGrid,
}

#[allow(unused_variables)]
impl<'a> Buddha<'a> {
    pub fn new(max_iterations: u64, view_rect: ViewRect, covarage_grid: &'a CovarageGrid) -> Self {
        let zero = f64x4::splat(0.);
        let mut buddha = Self {
            z_x: zero,
            z_y: zero,
            z_squared_x: zero,
            z_squared_y: zero,
            iteration: u64x4::splat(max_iterations),
            max_iteration: u64x4::splat(max_iterations),
            c_x: zero,
            c_y: zero,
            x_lower_bound: zero,
            y_lower_bound: zero,
            x_upper_bound: zero,
            y_upper_bound: zero,
            x_screen_offset: zero,
            y_screen_offset: zero,
            x_screen_scale: zero,
            y_screen_scale: zero,
            width: i64x4::splat(WIDTH as i64),
            pixels: vec![0; WIDTH * HEIGHT],
            samples: vec![DVec2::ZERO; 100_000],
            sample_index: 0,
            value_to_replace: mask64x4::from_array([true, false, false, false]),
            covarage_grid,
        };
        buddha.replenish_samples();
        buddha.set_view_rect(view_rect);
        buddha
    }
    fn iterate_samples(&mut self) {
        while self.sample_index < self.samples.len() {
            self.iterate();
        }
    }
    fn iterate(&mut self) {
        let zero = f64x4::splat(0.);
        if self.sample_index == self.samples.len() {
            self.replenish_samples();
        }

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
                self.value_to_replace & (!inside | self.iteration.lanes_ge(self.max_iteration));
            // hacky method to rotate.
            self.value_to_replace = unsafe {
                mask64x4::from_int_unchecked(
                    self.value_to_replace.to_int().rotate_lanes_left::<1>(),
                )
            };
            let new_sample = self.samples[self.sample_index];
            // increase index when sample was used
            self.sample_index += value_to_replace.any() as usize;
            let new_c_x = f64x4::splat(new_sample.x);
            let new_c_y = f64x4::splat(new_sample.y);
            self.c_x = value_to_replace.select(new_c_x, self.c_x);
            self.c_y = value_to_replace.select(new_c_y, self.c_y);
            self.z_x = value_to_replace.select(zero, self.z_x);
            self.z_y = value_to_replace.select(zero, self.z_y);
            self.z_squared_x = value_to_replace.select(zero, self.z_squared_x);
            self.z_squared_y = value_to_replace.select(zero, self.z_squared_y);
            self.iteration = value_to_replace.select(u64x4::splat(0), self.iteration);
        }

        // compute index of pixel. 0 when outside.
        let index = {
            let x_screen = (self.z_x - self.x_screen_offset) * self.x_screen_scale;
            let y_screen = (self.z_y - self.y_screen_offset) * self.y_screen_scale;

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

        let indexs = index.as_array();
        let added_pixels = [
            self.pixels[indexs[0] as usize].saturating_add(1),
            self.pixels[indexs[1] as usize].saturating_add(1),
            self.pixels[indexs[2] as usize].saturating_add(1),
            self.pixels[indexs[3] as usize].saturating_add(1),
        ];
        self.pixels[indexs[0] as usize] = added_pixels[0];
        self.pixels[indexs[1] as usize] = added_pixels[1];
        self.pixels[indexs[2] as usize] = added_pixels[2];
        self.pixels[indexs[3] as usize] = added_pixels[3];
    }
    fn normalise_pixels(&self) -> Vec<u8> {
        let max = *self.pixels.iter().max().unwrap();
        let mut reduced_max = max;
        while self
            .pixels
            .iter()
            .filter(|&&pixel| pixel > reduced_max)
            .count()
            * 100
            < self.pixels.len()
            && reduced_max > 1
        {
            reduced_max /= 2;
        }
        println!("max: {}, reduced max: {}", max, reduced_max);
        let mul = u32::MAX.checked_div(reduced_max).unwrap_or_default();
        self.pixels
            .iter()
            .flat_map(|pixel| std::iter::repeat(((pixel.saturating_mul(mul)) >> 24) as u8).take(4))
            .collect()
    }
    fn replenish_samples(&mut self) {
        self.covarage_grid.gen_samples(&mut self.samples);
        self.sample_index = 0;
    }
    fn set_view_rect(&mut self, view_rect: ViewRect) {
        let lower_bound = view_rect.top_left_corner;
        let upper_bound = view_rect.get_bottom_right_corner();
        let screen_offset = lower_bound;
        let screen_scale = view_rect.get_screen_scale();
        self.x_lower_bound = f64x4::splat(lower_bound.x);
        self.y_lower_bound = f64x4::splat(lower_bound.y);
        self.x_upper_bound = f64x4::splat(upper_bound.x);
        self.y_upper_bound = f64x4::splat(upper_bound.y);
        self.x_screen_offset = f64x4::splat(screen_offset.x);
        self.y_screen_offset = f64x4::splat(screen_offset.y);
        self.x_screen_scale = f64x4::splat(screen_scale.x);
        self.y_screen_scale = f64x4::splat(screen_scale.y);
    }
}

impl Updateable for Buddha<'_> {
    fn update(&mut self) {
        for _ in 0..10_000_000 {
            self.iterate();
        }
        println!("sample index: {}", self.sample_index);
    }
    fn draw(&mut self, drawer: &mut Drawer) {
        drawer.draw_raw_pixels(self.normalise_pixels());
    }
    fn update_view_rect(&mut self, view_rect: ViewRect) {
        self.set_view_rect(view_rect);
        self.pixels = vec![0; WIDTH * HEIGHT];
    }
}
