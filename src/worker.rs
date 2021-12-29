use flume::{Sender, Receiver};
use glam::DVec2;
use super::mandel_iter::iterate_points_dvec2;
use super::{MAX_ITER, MIN_ITER};


pub enum PointType {
    New(Vec<DVec2>),
    Mutated(Vec<DVec2>),
}

pub enum WorkerMessage {
    CheckPoints(PointType),

}

pub struct Worker {
    work: Receiver<WorkerMessage>,
    checked_samples: Sender<PointType>,
}

impl Worker {
    pub fn start(work: Receiver<WorkerMessage>, checked_samples: Sender<PointType>) {
        Self {
            work,
            checked_samples,
        }.work();
    }
    fn work(&mut self) {
        for work in self.work.iter() {
            match work {
                WorkerMessage::CheckPoints(points) => self.check_points(points),
            }
        }

    }
    fn check_points(&self, points: PointType) {
        let samples = match points {
            PointType::New(ref samples) => samples,
            PointType::Mutated(ref samples) => samples,
        };
        let mut checked_samples = Vec::with_capacity(samples.len());
        for samples in samples.array_chunks::<4>() {
            let iteration_counts = iterate_points_dvec2(samples, MAX_ITER);
            for (sample, iteration_count) in std::iter::zip(samples, iteration_counts) {
                if (MIN_ITER as i64) < iteration_count && iteration_count < MAX_ITER as i64 {
                    checked_samples.push(*sample);
                    let y_fliped = DVec2::new(sample.x, -sample.y);
                    checked_samples.push(y_fliped);
                }
            }
        }
        match points {
            PointType::New(_) => self.checked_samples.send(PointType::New(checked_samples)),
            PointType::Mutated(_) => self.checked_samples.send(PointType::Mutated(checked_samples)),
        }.unwrap();
    }
}

