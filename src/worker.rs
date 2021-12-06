use super::cell::*;
use super::util::four_point_inside_tests;
use rand::prelude::{thread_rng, ThreadRng};
use spmc;
use std::sync::mpsc;

pub struct Worker {
    cell_to_sample: spmc::Receiver<[Cell; 4]>,
    cell_that_are_inside: mpsc::Sender<Cell>,
    sampels: usize,
    grid_size: usize,
    limit: usize,
    rng: ThreadRng,
}
impl Worker {
    pub fn new(
        cell_to_work_on: spmc::Receiver<[Cell; 4]>,
        cell_that_are_inside: mpsc::Sender<Cell>,
        limit: usize,
        sampels: usize,
        grid_size: usize,
    ) {
        Self {
            cell_to_sample: cell_to_work_on,
            cell_that_are_inside,
            sampels,
            grid_size,
            limit,
            rng: thread_rng(),
        }
        .work()
    }
    pub fn work(&mut self) {
        loop {
            let cells_result = self.cell_to_sample.recv();
            let cells = cells_result.unwrap();
            let mut are_any_inside = [false; 4];
            for _ in 0..self.sampels {
                let are_these_inside = unsafe {
                    four_point_inside_tests(cells, self.limit, self.grid_size, &mut self.rng)
                };
                for (is_any_inside, is_this_inside) in
                    std::iter::zip(&mut are_any_inside, are_these_inside)
                {
                    *is_any_inside |= is_this_inside;
                }
                if are_any_inside.iter().all(|a|*a) {
                    break;
                }
            }
            for (cell, is_inside) in std::iter::zip(cells, are_any_inside) {
                if is_inside {
                    self.cell_that_are_inside.send(cell).unwrap();
                }
            }
        }
    }
}
