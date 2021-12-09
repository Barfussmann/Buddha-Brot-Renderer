use super::cell::*;
use super::save_cell::SaveCell;
use super::util::four_point_inside_tests;
use core_simd::i64x4;
use rand::prelude::{thread_rng, ThreadRng};
use std::sync::mpsc;

pub struct Worker {
    cell_to_sample: spmc::Receiver<Cell>,
    cell_that_are_inside: mpsc::Sender<SaveCell>,
    current_cells: [Cell; 4],
    sampels: usize,
    grid_size: usize,
    limit: usize,
    rng: ThreadRng,
}
impl Worker {
    pub fn start(
        cell_to_work_on: spmc::Receiver<Cell>,
        cell_that_are_inside: mpsc::Sender<SaveCell>,
        limit: usize,
        sampels: usize,
        grid_size: usize,
    ) {
        let mut worker = Self {
            cell_to_sample: cell_to_work_on,
            cell_that_are_inside,
            current_cells: [Cell::dummy(); 4],
            sampels,
            grid_size,
            limit,
            rng: thread_rng(),
        };
        worker.work()
    }
    fn replace_current_cells(&mut self) {
        let new_cell_with_negativ_y = loop {
            let new_cell = self.cell_to_sample.recv().unwrap();
            if new_cell.is_y_negativ() {
                break new_cell;
            }
        };
        self.current_cells = [new_cell_with_negativ_y; 4];
    }
    fn send_current_cells(&mut self, max_iteration: i64) {
        self.cell_that_are_inside
            .send(SaveCell::new(self.current_cells[0], max_iteration as u16))
            .unwrap();
    }
    pub fn work(&mut self) {
        let mut max_iterations = i64x4::splat(0);
        let mut current_iterations = 0;
        let mut any_inside = false;
        loop {
            let (iterations, is_inside) = four_point_inside_tests(
                self.current_cells,
                self.limit,
                self.grid_size,
                1024,
                &mut self.rng,
            );
            max_iterations = max_iterations.max(iterations);
            any_inside |= is_inside;
            current_iterations += 1;
            if current_iterations == self.sampels {
                if any_inside {
                    self.send_current_cells(max_iterations.horizontal_max());
                }
                self.replace_current_cells();
                max_iterations = i64x4::splat(0);
                current_iterations = 0;
                any_inside = false;
            }
        }
    }
}
