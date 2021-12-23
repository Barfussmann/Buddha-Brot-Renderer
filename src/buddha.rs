use super::camera::*;
use super::covarage_grid::*;
use super::sample_gen::SampleGen;
use super::sample_mutator::SampleMutator;
use core::simd::*;
use flume::{Receiver, Sender};
use glam::DVec2;
use rand::Rng;
use std::time::Instant;

use super::{HEIGHT, WIDTH};

pub struct Buddha {
    mandel_iter: MandelIter,
    max_iteration: u64x4,
    iterations_on_screen: i64x4,
    value_to_replace: mask64x4,
    pixels: Vec<u32>,
    samples: Vec<DVec2>,
    view: View,
    used_samples: Sender<Vec<DVec2>>,
    new_samples: Receiver<Vec<DVec2>>,
    mutated_samples: Receiver<Vec<DVec2>>,
    interesting_samples: Sender<Vec<DVec2>>,
    current_interesting_samples: Vec<DVec2>,
    using_mutated_samples: bool,
}

#[allow(unused_variables)]
impl Buddha {
    pub fn new(
        max_iterations: u64,
        view_rect: ViewRect,
        covarage_grid: &'static CovarageGrid,
    ) -> Self {
        let zero = f64x4::splat(0.);
        let (used_samples_tx, used_samples_rx) = flume::unbounded();
        let (new_samples_tx, new_samples_rx) = flume::unbounded();

        let iter = covarage_grid.get_cells().iter().cycle();
        for _ in 0..16 {
            let used_samples_rx = used_samples_rx.clone();
            let new_samples_tx = new_samples_tx.clone();
            let iter = iter.clone();
            std::thread::spawn(move || {
                SampleGen::start_working(
                    iter,
                    used_samples_rx,
                    new_samples_tx,
                    covarage_grid.get_grid_size(),
                )
            });
        }
        for _ in 0..48 {
            used_samples_tx.send(Vec::with_capacity(1024)).unwrap();
        }

        let (mutated_samples_tx, mutated_samples_rx) = flume::unbounded();
        let (interesting_samples_tx, interesting_samples_rx) = flume::unbounded();

        for _ in 0..16 {
            let mutated_samples_tx = mutated_samples_tx.clone();
            let interesting_samples_rx = interesting_samples_rx.clone();
            std::thread::spawn(move || {
                SampleMutator::start_working(
                    mutated_samples_tx,
                    interesting_samples_rx,
                    covarage_grid.get_grid_size(),
                )
            });
        }

        let mut buddha = Self {
            mandel_iter: MandelIter::new(),
            max_iteration: u64x4::splat(max_iterations),
            iterations_on_screen: i64x4::splat(0),
            pixels: vec![0; WIDTH * HEIGHT],
            samples: Vec::new(),
            value_to_replace: mask64x4::from_array([true, false, false, false]),
            view: View::new(ViewRect::default()),
            used_samples: used_samples_tx,
            new_samples: new_samples_rx,
            mutated_samples: mutated_samples_rx,
            interesting_samples: interesting_samples_tx,
            current_interesting_samples: Vec::new(),
            using_mutated_samples: false,
        };
        buddha.replenish_samples();
        buddha.set_view_rect(view_rect);
        buddha
    }
    fn iterate(&mut self) {
        let zero = f64x4::splat(0.);

        self.mandel_iter.next_iteration();

        if self.compute_is_in_view_rect() {
            self.add_pixels();
        }
    }
    fn is_inside(&self) -> mask64x4 {
        let abs = self.mandel_iter.z_squared_x + self.mandel_iter.z_squared_y;
        abs.lanes_le(f64x4::splat(4.))
    }
    fn compute_is_in_view_rect(&mut self) -> bool {
        self.view.is_inside(self.mandel_iter.get_z()).any()
    }
    fn add_pixels(&mut self) {
        self.iterations_on_screen -= self.view.in_view.to_int();
        let screen_index = self.view.screen_index(self.mandel_iter.get_z());
        let indexs = screen_index.as_array();
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

        let value_to_replace = self.value_to_replace
            & (!self.is_inside() | self.mandel_iter.iteration.lanes_ge(self.max_iteration));
        // hacky method to rotate.
        self.value_to_replace = unsafe {
            mask64x4::from_int_unchecked(self.value_to_replace.to_int().rotate_lanes_left::<1>())
        };
        if value_to_replace.any() {
            let new_sample = self.samples.pop().unwrap();
            if self.samples.is_empty() {
                self.replenish_samples();
            }
            let iter_on_screen_of_removed_sample = value_to_replace
                .select(self.iterations_on_screen, i64x4::splat(0))
                .horizontal_sum();
            if iter_on_screen_of_removed_sample != 0 {
                self.send_interesting_sample(value_to_replace)
            }

            self.iterations_on_screen =
                value_to_replace.select(i64x4::splat(0), self.iterations_on_screen);
            self.mandel_iter.try_replace(value_to_replace, new_sample);
        }
    }
    fn send_interesting_sample(&mut self, value: mask64x4) {
        if self.using_mutated_samples {
            return;
        }
        let x = value
            .select(self.mandel_iter.get_c().0, f64x4::splat(0.))
            .horizontal_sum();
        let y = value
            .select(self.mandel_iter.get_c().1, f64x4::splat(0.))
            .horizontal_sum();
        self.current_interesting_samples.push(DVec2::new(x, y));
        if self.current_interesting_samples.len() > 128 {
            self.interesting_samples
                .send(std::mem::replace(
                    &mut self.current_interesting_samples,
                    Vec::with_capacity(128),
                ))
                .unwrap();
        }
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
        if let Ok(mutated_sapmles) = self.mutated_samples.try_recv() {
            self.samples = mutated_sapmles;
            self.using_mutated_samples = true;
            print!("|")
        } else {
            let new_samples = self.new_samples.recv().unwrap();
            let used_samples = std::mem::replace(&mut self.samples, new_samples);
            self.used_samples.send(used_samples).unwrap();
            self.using_mutated_samples = false;
            print!(".")
        }
    }
    fn set_view_rect(&mut self, view_rect: ViewRect) {
        self.view = View::new(view_rect);
    }
}

impl Updateable for Buddha {
    fn update(&mut self) {
        let instant = Instant::now();
        let samples = 10_000_000;
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
    in_view: mask64x4,
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
            in_view: mask64x4::splat(true),
        }
    }
    #[inline(always)]
    fn is_inside(&mut self, (x, y): (f64x4, f64x4)) -> mask64x4 {
        let x_inside = x.lanes_ge(self.x_lower_bound) & x.lanes_le(self.x_upper_bound);
        let y_inside = y.lanes_ge(self.y_lower_bound) & y.lanes_le(self.y_upper_bound);
        self.in_view = x_inside & y_inside;
        self.in_view
    }
    #[inline(always)]
    fn screen_index(&self, (x, y): (f64x4, f64x4)) -> i64x4 {
        let x_screen = (x - self.x_lower_bound) * self.x_screen_scale;
        let y_screen = (y - self.y_lower_bound) * self.y_screen_scale;

        let inside_x = self.in_view.select(x_screen, f64x4::splat(0.));
        let inside_y = self.in_view.select(y_screen, f64x4::splat(0.));
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
    const fn get_c(&self) -> (f64x4, f64x4) {
        (self.c_x, self.c_y)
    }
}
