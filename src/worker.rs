use super::cell::*;
use super::util::four_point_inside_tests;
use rand::prelude::{thread_rng, ThreadRng};
use spmc;
use std::sync::mpsc;

pub struct Worker {
    cell_to_sample: spmc::Receiver<Vec<Cell>>,
    cell_that_are_inside: mpsc::Sender<Vec<Cell>>,
    cells_to_work_on: Vec<Cell>,
    cells_to_send: Vec<Cell>,
    current_cells: [Cell; 4],
    current_not_inside_count: [usize; 4],
    sampels: usize,
    grid_size: usize,
    limit: usize,
    rng: ThreadRng,
}
impl Worker {
    pub fn new(
        cell_to_work_on: spmc::Receiver<Vec<Cell>>,
        cell_that_are_inside: mpsc::Sender<Vec<Cell>>,
        limit: usize,
        sampels: usize,
        grid_size: usize,
    ) {
        let mut worker = Self {
            cells_to_work_on: cell_to_work_on.recv().unwrap(),
            cell_to_sample: cell_to_work_on,
            cell_that_are_inside,
            cells_to_send: Vec::new(),
            current_cells: [Cell::dummy(); 4],
            current_not_inside_count: [sampels, sampels - 1, sampels - 2, sampels - 3],
            sampels,
            grid_size,
            limit,
            rng: thread_rng(),
        };
        worker.work()
    }
    fn replace_current_cells(&mut self, cell_to_replace: [bool; 4]) {
        for i in 0..4 {
            if cell_to_replace[i] {
                self.cells_to_send.push(self.current_cells[i]);
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
        if self.cells_to_work_on.is_empty() {
            self.get_next_cells();
        }
        self.current_cells[index] = self.cells_to_work_on.pop().unwrap();
        self.current_not_inside_count[index] = 0;
    }
    fn get_next_cells(&mut self) {
        while self.cells_to_work_on.is_empty() {
            self.cell_that_are_inside
                .send(std::mem::replace(
                    &mut self.cells_to_send,
                    std::mem::replace(
                        &mut self.cells_to_work_on,
                        self.cell_to_sample.recv().unwrap(),
                    ),
                ))
                .unwrap();
        }
    }
    pub fn work(&mut self) {
        loop {
            let cell_is_inside = unsafe {
                four_point_inside_tests(
                    self.current_cells,
                    self.limit,
                    self.grid_size,
                    &mut self.rng,
                )
            };
            for count in self.current_not_inside_count.iter_mut() {
                *count += 1;
            }
            if cell_is_inside.is_none() {
                self.replace_old_cells();
                continue;
            } else {
                self.replace_current_cells(cell_is_inside.unwrap());
                self.replace_old_cells();
            }
        }
    }
}
