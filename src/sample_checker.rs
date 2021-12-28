use super::mandel_iter::*;
use super::{MAX_ITER, MIN_ITER};
use flume::{Receiver, Sender};
use glam::DVec2;

pub struct SampleChecker {
    new_samples: Receiver<Vec<DVec2>>,
    checked_samples: Sender<Vec<DVec2>>,
}
impl SampleChecker {
    pub fn start_working(new_samples: Receiver<Vec<DVec2>>, checked_samples: Sender<Vec<DVec2>>) {
        Self {
            new_samples,
            checked_samples,
        }
        .work();
    }
    fn work(&mut self) {
        let mut ckecked_samples = Vec::with_capacity(1024);
        for new_sampels in self.new_samples.iter() {
            for new_samples in new_sampels.array_chunks::<4>() {
                let iteration_counts = iterate_points_dvec2(new_samples, MAX_ITER);
                for (sample, iteration_count) in std::iter::zip(new_samples, iteration_counts) {
                    if (MIN_ITER as i64) < iteration_count && iteration_count < MAX_ITER as i64 {
                        ckecked_samples.push(*sample);
                        let y_fliped = DVec2::new(sample.x, -sample.y);
                        ckecked_samples.push(y_fliped);
                    }
                }
            }
            if self.checked_samples
                .send(std::mem::replace(
                    &mut ckecked_samples,
                    Vec::with_capacity(1024),
                )).is_err() {
                    break;
                }
        }
    }
}
