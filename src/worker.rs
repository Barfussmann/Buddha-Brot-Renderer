use spmc;
use std::sync::mpsc;
use super::cell::*;
use rand::prelude::{ThreadRng, thread_rng};
use super::util::four_point_inside_tests;


pub struct Worker {
    cell_to_sample: spmc::Receiver<[Cell; 4]>,
    cell_that_are_inside: mpsc::Sender<Cell>,
    sampels: usize,
    grid_size: usize,
    limit: usize,
    rng: ThreadRng,
}
impl Worker {
    pub fn new(cell_to_work_on: spmc::Receiver<[Cell;4]>, cell_that_are_inside: mpsc::Sender<Cell>, limit: usize, sampels: usize, grid_size: usize){
        Self {
            cell_to_sample: cell_to_work_on,
            cell_that_are_inside,
            sampels,
            grid_size,
            limit,
            rng: thread_rng(),
        }.work()
    }
    pub fn work(&mut self) {
        loop {
            let cells_result = self.cell_to_sample.recv();
            if cells_result.is_err() {
                return;
            }
            let cells = cells_result.unwrap();
            let mut are_any_inside = [false; 4];
            for _ in 0..self.sampels {
                let are_these_inside  = four_point_inside_tests(cells, self.limit, self.grid_size, &mut self.rng);
                for (is_any_inside, is_this_inside) in std::iter::zip(&mut are_any_inside, are_these_inside) {
                    *is_any_inside |= is_this_inside;
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