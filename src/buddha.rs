use super::camera::*;
use super::covarage_grid::*;
use super::histogram::Histogram;
use super::pixels::Pixels;
use super::sample_checker::SampleChecker;
// use super::sample_mutator::SampleMutator;
use super::sample_gen::SampleGen;
use core::simd::*;
use flume::{Receiver, Sender};
use glam::DVec2;
use std::time::Instant;

use super::{HEIGHT, WIDTH};

pub struct Buddha {
    mandel_iter: MandelIter,
    max_iteration: u64x4,
    iterations_on_screen: i64x4,
    pixels: Pixels,
    samples: Vec<DVec2>,
    view: View,
    new_samples: Receiver<Vec<DVec2>>,
    mutated_samples: Receiver<Vec<DVec2>>,
    interesting_samples: Sender<Vec<DVec2>>,
    current_interesting_samples: Vec<DVec2>,
    using_mutated_samples: bool,
    histogram: Histogram,
}

#[allow(unused_variables)]
impl Buddha {
    pub fn new(
        max_iterations: u64,
        view_rect: ViewRect,
        covarage_grid: &'static CovarageGrid,
    ) -> Self {
        let zero = f64x4::splat(0.);

        let (new_samples_tx, new_samples_rx) = flume::bounded(64);

        std::thread::spawn(move || {
            SampleGen::start(
                covarage_grid.get_cells().clone(),
                covarage_grid.get_grid_size(),
                new_samples_tx,
            );
        });

        let (checked_samples_tx, checked_samples_rx) = flume::bounded(64);

        for _ in 0..16 {
            let new_samples_rx = new_samples_rx.clone();
            let checked_samples_tx = checked_samples_tx.clone();
            std::thread::spawn(move || {
                SampleChecker::start_working(new_samples_rx, checked_samples_tx)
            });
        }

        let (mutated_samples_tx, mutated_samples_rx) = flume::unbounded();
        let (interesting_samples_tx, interesting_samples_rx) = flume::unbounded();

        // for _ in 0..16 {
        //     let mutated_samples_tx = mutated_samples_tx.clone();
        //     let interesting_samples_rx = interesting_samples_rx.clone();
        //     std::thread::spawn(move || {
        //         SampleMutator::start_working(
        //             mutated_samples_tx,
        //             interesting_samples_rx,
        //             covarage_grid.get_grid_size(),
        //         )
        //     });
        // }

        let mut buddha = Self {
            mandel_iter: MandelIter::new(),
            max_iteration: u64x4::splat(max_iterations),
            iterations_on_screen: i64x4::splat(0),
            pixels: Pixels::new(WIDTH, HEIGHT),
            samples: Vec::new(),
            view: View::new(ViewRect::default()),
            new_samples: checked_samples_rx,
            mutated_samples: mutated_samples_rx,
            interesting_samples: interesting_samples_tx,
            current_interesting_samples: Vec::new(),
            using_mutated_samples: false,
            histogram: Histogram::new(WIDTH, HEIGHT, ViewRect::default()),
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
        self.pixels.quad_add_one(screen_index);
    }
    fn try_replace_samples(&mut self) {
        let values_to_replace =
            !self.is_inside() | self.mandel_iter.iteration.lanes_ge(self.max_iteration);

        if values_to_replace.any() {
            let shifted = (values_to_replace.to_int().rotate_lanes_right::<1>()
                & i64x4::from_array([0, -1, -1, -1]))
                | (values_to_replace.to_int().rotate_lanes_right::<2>()
                    & i64x4::from_array([0, 0, -1, -1]))
                | (values_to_replace.to_int().rotate_lanes_right::<3>()
                    & i64x4::from_array([0, 0, 0, -1]));
            let singel_value_to_replace =
                values_to_replace & !unsafe { mask64x4::from_int_unchecked(shifted) };

            let new_sample = self.samples.pop().unwrap();
            self.histogram.add_point(new_sample);
            if self.samples.is_empty() {
                self.replenish_samples();
            }
            let iter_on_screen_of_removed_sample = singel_value_to_replace
                .select(self.iterations_on_screen, i64x4::splat(0))
                .horizontal_sum();
            // if iter_on_screen_of_removed_sample >= 2 {
            //     self.send_interesting_sample(singel_value_to_replace)
            // }

            self.iterations_on_screen =
                singel_value_to_replace.select(i64x4::splat(0), self.iterations_on_screen);
            self.mandel_iter
                .replace(singel_value_to_replace, new_sample);
        }
    }
    fn send_interesting_sample(&mut self, value: mask64x4) {
        // return;
        if self.using_mutated_samples {
            return;
        }
        let x = value
            .select(self.mandel_iter.get_c().0, f64x4::splat(0.))
            .horizontal_sum();
        let y = value
            .select(self.mandel_iter.get_c().1, f64x4::splat(0.))
            .horizontal_sum();
        let iteresting_sample = DVec2::new(x, y);
        self.histogram.add_point(iteresting_sample);
        self.current_interesting_samples.push(iteresting_sample);
        if self.current_interesting_samples.len() > 128 {
            self.interesting_samples
                .send(std::mem::replace(
                    &mut self.current_interesting_samples,
                    Vec::with_capacity(128),
                ))
                .unwrap();
        }
    }
    fn normalise_pixels(&mut self) -> Vec<u32> {
        self.pixels.noramalise()
    }
    fn replenish_samples(&mut self) {
        if let Ok(mutated_sapmles) = self.mutated_samples.try_recv() {
            self.samples = mutated_sapmles;
            self.using_mutated_samples = true;
        } else {
            let new_samples = self.new_samples.recv().unwrap();
            let used_samples = std::mem::replace(&mut self.samples, new_samples);
            // self.used_samples.send(used_samples).unwrap();
            self.using_mutated_samples = false;
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
        for _ in 0..samples {
            self.iterate();
            self.try_replace_samples();
        }
        // println!(
        //     "Iteration per ys: {} ",
        //     samples as f64 / instant.elapsed().as_micros() as f64
        // );
    }
    fn draw(&mut self, _view: ViewRect) -> Vec<u32> {
        self.histogram.draw();
        self.normalise_pixels()
    }
    fn update_view_rect(&mut self, view_rect: ViewRect) {
        self.set_view_rect(view_rect);
        self.pixels.clear();
        self.histogram.clear();
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
    /// only one value can be replaced. The one that is true in value_to_replace.
    #[inline(always)]
    fn replace(&mut self, value_to_replace: mask64x4, new_sample: DVec2) {
        let zero = f64x4::splat(0.);
        let new_c_x = f64x4::splat(new_sample.x);
        let new_c_y = f64x4::splat(new_sample.y);

        assert!(new_sample.x > -3.);

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
