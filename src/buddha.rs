use super::camera::*;
use super::covarage_grid::*;
use core::simd::*;
use glam::DVec2;
use std::time::Instant;

use super::{HEIGHT, WIDTH};

pub struct Buddha<'a> {
    mandel_iter: MandelIter,
    inside: mask64x4,
    inside_view_rect: mask64x4,
    index: i64x4,
    max_iteration: u64x4,
    value_to_replace: mask64x4,
    pixels: Vec<u32>,
    sample_index: usize,
    samples: Vec<DVec2>,
    covarage_grid: &'a CovarageGrid,
    // sample_gen_iter: 
    view: View,
}

#[allow(unused_variables)]
impl<'a> Buddha<'a> {
    pub fn new(max_iterations: u64, view_rect: ViewRect, covarage_grid: &'a CovarageGrid) -> Self {
        let zero = f64x4::splat(0.);
        let mut buddha = Self {
            mandel_iter: MandelIter::new(),
            max_iteration: u64x4::splat(max_iterations),
            inside: mask64x4::splat(false),
            inside_view_rect: mask64x4::splat(false),
            index: i64x4::splat(0),
            pixels: vec![0; WIDTH * HEIGHT],
            samples: vec![DVec2::ZERO; 1_000_000],
            sample_index: 0,
            value_to_replace: mask64x4::from_array([true, false, false, false]),
            covarage_grid,
            view: View::new(ViewRect::default()),
        };
        buddha.replenish_samples();
        buddha.set_view_rect(view_rect);
        buddha
    }
    fn iterate(&mut self) {
        let zero = f64x4::splat(0.);
        if self.sample_index == self.samples.len() {
            self.replenish_samples();
        }

        self.mandel_iter.next_iteration();
        self.inside = self.is_inside();

        self.compute_is_in_view_rect();
        if self.inside_view_rect.any() {
            self.compute_index();
            self.add_pixels();
        }
    }
    fn is_inside(&self) -> mask64x4 {
        let abs = self.mandel_iter.z_squared_x + self.mandel_iter.z_squared_y;
        abs.lanes_le(f64x4::splat(4.))
    }
    fn compute_is_in_view_rect(&mut self) {
        self.inside_view_rect = self.view.is_inside(self.mandel_iter.get_z());
    }
    fn compute_index(&mut self) {
        self.index = self
            .view
            .screen_index(self.mandel_iter.get_z(), self.inside_view_rect);
    }
    fn add_pixels(&mut self) {
        let indexs = self.index.as_array();
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
    fn try_replace_samples(&mut self) {
        let zero = f64x4::splat(0.);

        let value_to_replace =
            self.value_to_replace & (!self.inside | self.mandel_iter.iteration.lanes_ge(self.max_iteration));
        // hacky method to rotate.
        self.value_to_replace = unsafe {
            mask64x4::from_int_unchecked(self.value_to_replace.to_int().rotate_lanes_left::<1>())
        };
        let new_sample = self.samples[self.sample_index];
        // increase index when sample was used
        self.sample_index += value_to_replace.any() as usize;

        self.mandel_iter.try_replace(value_to_replace, new_sample);

    }
    fn normalise_pixels(&mut self) -> Vec<u8> {
        self.pixels[0] = 0;
        let max = *self.pixels.iter().max().unwrap();
        let mut reduced_max = max;
        while self
            .pixels
            .iter()
            .filter(|&&pixel| pixel > reduced_max)
            .count()
            * 1_000
            < self.pixels.len()
            && reduced_max > 1
        {
            reduced_max /= 2;
        }
        let mul = u32::MAX.checked_div(reduced_max).unwrap_or_default();
        self.pixels
            .iter()
            // .flat_map(|pixel| std::iter::repeat(*pixel).take(4))
            .flat_map(|pixel| std::iter::repeat((pixel.saturating_mul(mul) >> 24) as u8).take(4))
            .collect()
    }
    fn replenish_samples(&mut self) {
        self.sample_index = 0;
        self.covarage_grid.gen_samples(&mut self.samples);
    }
    fn set_view_rect(&mut self, view_rect: ViewRect) {
        self.view = View::new(view_rect);
    }
}

impl Updateable for Buddha<'_> {
    fn update(&mut self) {
        let instant = Instant::now();
        let samples = 1_000_000;
        for _ in 0..samples / 4 {
            for _ in 0..4 {
                self.iterate();
            }
            self.try_replace_samples();
        }
        println!(
            "Iteration per ys: {} ",
            samples as f64 / instant.elapsed().as_micros() as f64
        );
    }
    fn draw(&mut self, drawer: &mut Drawer) {
        drawer.draw_raw_pixels(self.normalise_pixels());
    }
    fn update_view_rect(&mut self, view_rect: ViewRect) {
        self.set_view_rect(view_rect);
        self.pixels = vec![0; WIDTH * HEIGHT];
    }
}

struct View {
    x_lower_bound: f64x4,
    y_lower_bound: f64x4,
    x_upper_bound: f64x4,
    y_upper_bound: f64x4,
    x_screen_scale: f64x4,
    y_screen_scale: f64x4,
}
impl View {
    #[inline(always)]
    fn new(view_rect: ViewRect) -> Self {
        let lower_bound = view_rect.top_left_corner;
        let upper_bound = view_rect.get_bottom_right_corner();
        let screen_scale = view_rect.get_screen_scale();
        Self {
            x_lower_bound: f64x4::splat(lower_bound.x),
            y_lower_bound: f64x4::splat(lower_bound.y),
            x_upper_bound: f64x4::splat(upper_bound.x),
            y_upper_bound: f64x4::splat(upper_bound.y),
            x_screen_scale: f64x4::splat(screen_scale.x),
            y_screen_scale: f64x4::splat(screen_scale.y),
        }
    }
    #[inline(always)]
    fn is_inside(&self, (x, y): (f64x4, f64x4)) -> mask64x4 {
        let x_inside = x.lanes_ge(self.x_lower_bound) & x.lanes_le(self.x_upper_bound);
        let y_inside = y.lanes_ge(self.y_lower_bound) & y.lanes_le(self.y_upper_bound);
        x_inside & y_inside
    }
    #[inline(always)]
    fn screen_index(&self, (x, y): (f64x4, f64x4), in_view_rect: mask64x4) -> i64x4 {
        let x_screen = (x - self.x_lower_bound) * self.x_screen_scale;
        let y_screen = (y - self.y_lower_bound) * self.y_screen_scale;

        let inside_x = in_view_rect.select(x_screen, f64x4::splat(0.));
        let inside_y = in_view_rect.select(y_screen, f64x4::splat(0.));
        let x_index = unsafe { inside_x.to_int_unchecked() };
        let y_index = unsafe { inside_y.to_int_unchecked() };
        x_index + y_index * i64x4::splat(WIDTH as i64)
    }
}
struct MandelIter {
    z_x: f64x4,
    z_y: f64x4,
    z_squared_x: f64x4,
    z_squared_y: f64x4,
    c_x: f64x4,
    c_y: f64x4,
    iteration: u64x4,
}
impl MandelIter {
    const fn new() -> Self {
        let zero = f64x4::splat(0.);
        Self {
            z_x: zero,
            z_y: zero,
            z_squared_x: zero,
            z_squared_y: zero,
            c_x: zero,
            c_y: zero,
            iteration: u64x4::splat(1_000_000),
        }
    }
    #[inline(always)]
    fn next_iteration(&mut self) {
        self.z_y = f64x4::splat(2.) * self.z_x * self.z_y + self.c_y;
        self.z_x = self.z_squared_x - self.z_squared_y + self.c_x;
        self.z_squared_x = self.z_x * self.z_x;
        self.z_squared_y = self.z_y * self.z_y;
        self.iteration += u64x4::splat(1);
    }
    #[inline(always)]
    fn try_replace(&mut self, value_to_replace: mask64x4, new_sample: DVec2) {
        let zero = f64x4::splat(0.);
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
    #[inline(always)]
    const fn get_z(&self) -> (f64x4, f64x4) {
        (self.z_x, self.z_y)
    }
}