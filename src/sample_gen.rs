use super::covarage_grid::*;
use flume::{Receiver, Sender};
use glam::DVec2;

struct SampleGen {
    cell_iter: std::slice::Iter<'static, cell::Cell>,
    used_samples: Receiver<Vec<DVec2>>,
    new_samples: Sender<Vec<DVec2>>,
}
impl SampleGen {
    fn new(
        cell_iter: std::slice::Iter<'static, cell::Cell>,
        used_samples: Receiver<Vec<DVec2>>,
        new_samples: Sender<Vec<DVec2>>,
    ) -> Self {
        Self {
            cell_iter,
            used_samples,
            new_samples,
        }
    }
    fn work(&mut self) {
        for used_samples in self.used_samples.iter() {

        }
    }
    fn replace_samples(&self, samples: Vec<DVec2>) -> Vec<DVec2> {
        let mut new_samples = vec![cell::Cell::dummy()];

        todo!()
    }
}
