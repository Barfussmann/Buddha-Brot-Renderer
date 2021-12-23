use super::mandel_iter::*;
use flume::{Receiver, Sender};
use glam::DVec2;
use rand::{prelude::ThreadRng, Rng};

pub struct SampleMutator {
    mutated_samples: Sender<Vec<DVec2>>,
    interesting_samples: Receiver<Vec<DVec2>>,
    grid_size: usize,
}
impl SampleMutator {
    pub fn start_working(
        mutated_samples: Sender<Vec<DVec2>>,
        interesting_samples: Receiver<Vec<DVec2>>,
        grid_size: usize,
    ) {
        Self {
            mutated_samples,
            interesting_samples,
            grid_size,
        }
        .work();
    }
    fn work(&mut self) {
        let rng = &mut rand::thread_rng();
        let mut new_samples = Vec::new();
        for interesting_samples in self.interesting_samples.iter() {
            for interesting_sample in interesting_samples {
                for _ in 0..4 {
                    let poss_samples = [
                        Self::mutate_point(interesting_sample, rng, self.grid_size),
                        Self::mutate_point(interesting_sample, rng, self.grid_size),
                        Self::mutate_point(interesting_sample, rng, self.grid_size),
                        Self::mutate_point(interesting_sample, rng, self.grid_size),
                    ];
                    let iteration_counts = iterate_points_dvec2(&poss_samples, 100);
                    for (sample, iteration_count) in std::iter::zip(poss_samples, iteration_counts)
                    {
                        if 30 < iteration_count && iteration_count < 100 {
                            new_samples.push(sample);
                        }
                    }
                    if new_samples.len() > 1020 {
                        self.mutated_samples
                            .send(std::mem::replace(
                                &mut new_samples,
                                Vec::with_capacity(1024),
                            ))
                            .unwrap();
                    }
                }
            }
        }
    }
    fn mutate_point(point: DVec2, rng: &mut ThreadRng, grid_size: usize) -> DVec2 {
        let grid_len = 4.0 / (grid_size as f64);

        let len = rng.gen_range(0_f64..1.) * grid_len;
        let angle = rng.gen_range(0_f64..std::f64::consts::PI * 2.);
        DVec2::new(
            len.mul_add(angle.cos(), point.x),
            len.mul_add(angle.sin(), point.y),
        )
    }
}
