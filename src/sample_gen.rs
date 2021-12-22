use std::iter::Cycle;
use std::slice::Iter;

use super::covarage_grid::*;
use super::mandel_iter::*;
use flume::{Receiver, Sender};
use glam::DVec2;

struct SampleGen {
    cell_iter: Cycle<Iter<'static, cell::Cell>>,
    used_samples: Receiver<Vec<DVec2>>,
    new_samples: Sender<Vec<DVec2>>,
    rng: rand::rngs::ThreadRng,
    size: usize,
}
impl SampleGen {
    fn start_working(
        cell_iter: Cycle<Iter<'static, cell::Cell>>,
        used_samples: Receiver<Vec<DVec2>>,
        new_samples: Sender<Vec<DVec2>>,
        size: usize,
    ) {
        Self {
            cell_iter,
            used_samples,
            new_samples,
            rng: rand::thread_rng(),
            size,
        }
        .work();
    }
    fn work(&mut self) {
        for mut used_samples in self.used_samples.iter() {
            while used_samples.len() < 1020 {
                let cell = self.cell_iter.next().unwrap();

                let poss_samples = [
                    cell.gen_point_inside(self.size, &mut self.rng),
                    cell.gen_point_inside(self.size, &mut self.rng),
                    cell.gen_point_inside(self.size, &mut self.rng),
                    cell.gen_point_inside(self.size, &mut self.rng),
                ];
                let iteration_counts = iterate_points_dvec2(&poss_samples, 100);
                for (sample, iteration_count) in std::iter::zip(poss_samples, iteration_counts) {
                    if 30 < iteration_count && iteration_count < 100 {
                        used_samples.push(sample);
                    }
                }
            }
            self.new_samples.send(used_samples).unwrap();
        }
    }
}
