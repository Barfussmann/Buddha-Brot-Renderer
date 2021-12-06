use super::cell::*;
use super::util::four_point_inside_tests;
use rand::prelude::{thread_rng, ThreadRng};
use spmc;
use std::sync::mpsc;

pub struct Worker {
    cell_to_sample: spmc::Receiver<Cell>,
    cell_that_are_inside: mpsc::Sender<Cell>,
    current_cells: [Cell; 4],
    current_not_inside_count: [usize; 4],
    sampels: usize,
    grid_size: usize,
    limit: usize,
    rng: ThreadRng,
}
impl Worker {
    pub fn new(
        cell_to_work_on: spmc::Receiver<Cell>,
        cell_that_are_inside: mpsc::Sender<Cell>,
        limit: usize,
        sampels: usize,
        grid_size: usize,
    ) {
        let mut worker = Self {
            cell_to_sample: cell_to_work_on,
            cell_that_are_inside,
            current_cells: [Cell::dummy(); 4],
            current_not_inside_count: [0; 4],
            sampels,
            grid_size,
            limit,
            rng: thread_rng(),
        };
        worker.replace_current_cells([true; 4]);
        worker.work()
    }
    fn send_current_cells(&mut self, cell_to_send: [bool; 4]) {
        for i in 0..4 {
            if cell_to_send[i] {
                self.cell_that_are_inside.send(self.current_cells[i]).unwrap();
            }
        }
    }
    fn replace_current_cells(&mut self, cell_to_replace: [bool; 4]) {
        for i in 0..4 {
            if cell_to_replace[i] {
                self.replace_cell(i);
            }
        }
    }
    fn replace_old_cells(&mut self) {
        for i in 0..4 {
            if self.current_not_inside_count[i] > self.sampels {
                self.replace_cell(i);
            }
        }
    }
    fn replace_cell(&mut self, index: usize) {
        self.current_cells[index] = self.cell_to_sample.recv().unwrap();
        self.current_not_inside_count[index] = 0;
    }
    pub fn work(&mut self) {
        loop {
            let cell_is_inside = unsafe {
                four_point_inside_tests(self.current_cells, self.limit, self.grid_size, &mut self.rng)
            };
            for count in self.current_not_inside_count.iter_mut() {
                *count += 1;
            }
            self.replace_old_cells();
            if cell_is_inside.is_none() {
                continue;
            } else {
                self.send_current_cells(cell_is_inside.unwrap());
                self.replace_current_cells(cell_is_inside.unwrap());
            }
        }
    }
}
