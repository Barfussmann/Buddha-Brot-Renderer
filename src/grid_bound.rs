use super::camera::*;
use super::cell::*;
use super::draw_manager::DrawManager;
use super::grid::*;
use super::util::*;
use super::worker::*;
use glam::IVec2;
use spmc;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub struct CovarageGrid {
    pub inside_cells: Grid,
    limit: usize,
    grid_size: usize,
    cell_to_sample: spmc::Sender<Cell>,
    cell_that_are_inside: mpsc::Receiver<Cell>,
    processed_cells: i64,
}

impl CovarageGrid {
    pub fn new(limit: usize, sample_per_cell: usize, grid_size: usize) -> Self {
        let (mut cell_to_sample_sender, cell_to_sample_receiver) = spmc::channel();
        let (cell_that_are_inside_sender, cell_that_are_inside_receiver) = mpsc::channel();
        for _ in 0..16 {
            let receiver = cell_to_sample_receiver.clone();
            let sender = cell_that_are_inside_sender.clone();
            thread::spawn(move || Worker::new(receiver, sender, limit, sample_per_cell, grid_size));
        }
        let starting_x = (grid_size / 16) as i32;
        for x in starting_x..=starting_x + (grid_size / 100) as i32 {
            cell_to_sample_sender
                .send(Cell::new(IVec2::new(x, 0)))
                .unwrap();
        }
        CovarageGrid {
            inside_cells: Grid::new(grid_size),
            limit,
            grid_size,
            cell_to_sample: cell_to_sample_sender,
            cell_that_are_inside: cell_that_are_inside_receiver,
            processed_cells: 0,
        }
    }
    pub fn sample_neighbors(&mut self) {
        for _ in 0..10_000 {
            match self.cell_that_are_inside.try_recv() {
                Ok(inside_cell) => {
                    self.add_inside_cell(inside_cell);
                }
                Err(_) => break,
            }
        }
    }
    #[inline(always)]
    fn add_inside_cell(&mut self, cell: Cell) {
        for neighbor in cell.get_neighbors() {
            if !self.chech_if_neighbor_is_new(neighbor) {
                continue;
            }
            self.processed_cells += 1;
            self.cell_to_sample.send(neighbor).unwrap();
        }
        self.inside_cells.insert(cell);
    }
    /// Has to be called before cell are inserted
    fn chech_if_neighbor_is_new(&self, cell: Cell) -> bool {
        for neighbor in cell.get_neighbors() {
            if self.inside_cells.is_activ(neighbor) {
                return false;
            }
        }
        return true;
    }
    pub fn area(&self) -> f64 {
        self.inside_cells.activ_count() as f64 * Cell::area(self.grid_size)
    }
    pub fn real_covered_area(&self) -> f64 {
        todo!()
    }
    pub fn draw(&self, draw_manager: &DrawManager, camera: &CameraManger) {
        if draw_manager.get_draw_inside_cells() {
            self.inside_cells.draw(GREEN, camera);
        }
    }
    pub fn get_inside_cell_count(&self) -> usize {
        self.inside_cells.activ_count()
    }
    pub fn get_processed_cells(&self) -> usize {
        self.processed_cells as usize
    }
}
