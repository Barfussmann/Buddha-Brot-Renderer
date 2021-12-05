use super::camera::*;
use super::cell::*;
use super::draw_manager::DrawManager;
use super::grid::*;
use super::util::*;
use super::worker::*;
use glam::IVec2;
use std::thread;
// use rand::thread_rng;
// use rayon::prelude::*;
use spmc;
use std::sync::mpsc;

pub struct CovarageGrid {
    pub inside_cells: Grid,
    limit: usize,
    grid_size: usize,
    cell_to_sample: spmc::Sender<[Cell; 4]>,
    cell_that_are_inside: mpsc::Receiver<Cell>,
    left_over_cells: Vec<Cell>,
}

impl CovarageGrid {
    pub fn new(limit: usize, sample_per_cell: usize, grid_size: usize) -> Self {
        let (cell_to_sample_sender, cell_to_sample_receiver) = spmc::channel();
        let (cell_that_are_inside_sender, cell_that_are_inside_receiver) = mpsc::channel();
        for _ in 0..16 {
            let receiver = cell_to_sample_receiver.clone();
            let sender = cell_that_are_inside_sender.clone();
            thread::spawn(move || Worker::new(receiver, sender, limit, sample_per_cell, grid_size));
        }
        let mut left_over_cells = Vec::with_capacity(4);
        let starting_x = (grid_size / 16) as i32;
        for x in starting_x..=starting_x + (grid_size / 100) as i32 {
            left_over_cells.push(Cell::new(IVec2::new(x, 0)));
        }
        CovarageGrid {
            inside_cells: Grid::new(grid_size),
            limit,
            grid_size,
            cell_to_sample: cell_to_sample_sender,
            cell_that_are_inside: cell_that_are_inside_receiver,
            left_over_cells,
        }
    }
    pub fn sample_neighbors(&mut self) {
        for _ in 0..1000 {
            self.try_send_cell();
            let try_inside_cell = self.cell_that_are_inside.try_recv();
            if try_inside_cell.is_err() {
                break;
            }
            let inside_cell = try_inside_cell.unwrap();
            self.add_inside_cell(inside_cell);
        }
    }
    fn try_send_cell(&mut self) {
        if self.left_over_cells.len() < 4 {
            return;
        }
        match self.cell_to_sample.send([
            self.left_over_cells.pop().unwrap(),
            self.left_over_cells.pop().unwrap(),
            self.left_over_cells.pop().unwrap(),
            self.left_over_cells.pop().unwrap(),
        ]) {
            Ok(_) => {}
            Err(err) => {
                println!("{err}");
            }
        }
    }
    #[inline(always)]
    fn add_inside_cell(&mut self, cell: Cell) {
        for neighbor in cell.get_neighbors() {
            if !self.chech_if_neighbor_is_new(neighbor) {
                continue;
            }
            self.left_over_cells.push(neighbor);
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
}
