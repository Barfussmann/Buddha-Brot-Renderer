use super::{camera::*, cell::*, grid::*, sample_cells::*, sampled_cell::*, worker::*};
use glam::IVec2;
use std::{sync::mpsc, thread, time::Instant};

pub struct CovarageGridGen {
    inside_cells: Grid,
    grid_size: usize,
    cell_to_sample: spmc::Sender<Cell>,
    cell_that_are_inside: mpsc::Receiver<SampledCell>,
    processed_cells_count: i64,
    saved_cells: Vec<SampledCell>,
    start_time: Instant,
}

impl CovarageGridGen {
    pub fn new(limit: usize, sample_per_cell: usize, grid_size: usize) -> Self {
        let (mut cell_to_sample_sender, cell_to_sample_receiver) = spmc::channel();
        let (cell_that_are_inside_sender, cell_that_are_inside_receiver) = mpsc::channel();
        for _ in 0..16 {
            let receiver = cell_to_sample_receiver.clone();
            let sender = cell_that_are_inside_sender.clone();
            thread::spawn(move || {
                Worker::start(receiver, sender, limit, sample_per_cell, grid_size)
            });
        }
        let starting_x = (grid_size / 16) as i32;
        for x in starting_x..=starting_x + (grid_size / 100) as i32 {
            cell_to_sample_sender
                .send(Cell::new(IVec2::new(x, 0)))
                .unwrap();
        }
        CovarageGridGen {
            inside_cells: Grid::new(grid_size),
            grid_size,
            cell_to_sample: cell_to_sample_sender,
            cell_that_are_inside: cell_that_are_inside_receiver,
            processed_cells_count: 0,
            saved_cells: Vec::new(),
            start_time: Instant::now(),
        }
    }
    pub fn sample_neighbors(&mut self) {
        let start = Instant::now();
        while Instant::now().duration_since(start).as_millis() < 50 {
            for save_cell in self.cell_that_are_inside.try_iter().take(10_000) {
                for neighbor in save_cell.get_cell(self.grid_size).get_neighbors() {
                    if !self.chech_if_neighbor_is_new(neighbor) {
                        continue;
                    }
                    self.processed_cells_count += 1;
                    self.cell_to_sample.send(neighbor).unwrap();
                }
                self.inside_cells.insert(save_cell.get_cell(self.grid_size));
                self.saved_cells.push(save_cell);
            }
        }
    }
    /// Has to be called before cell are inserted
    fn chech_if_neighbor_is_new(&self, cell: Cell) -> bool {
        for neighbor in cell.get_neighbors() {
            if self.inside_cells.is_activ(neighbor) {
                return false;
            }
        }
        true
    }
    pub fn is_finished(&self) -> bool {
        self.inside_cells
            .is_activ(Cell::new(IVec2::new(-((self.grid_size / 2) as i32) + 1, 0)))
    }
    pub fn to_complet_sampled_cells(&self) -> SampleCells {
        assert!(self.is_finished(), "CovarageGridGen is not finished");
        let mut sorterd_saved_cells = self.saved_cells.clone();
        sorterd_saved_cells.sort_unstable_by_key(|b| std::cmp::Reverse(b.get_highest_iteration()));
        SampleCells::new(sorterd_saved_cells, self.grid_size)
    }
    pub fn draw(&self, camera: &CameraManger) {
        self.inside_cells.draw(camera);
    }
    pub fn get_processed_cells_count(&self) -> usize {
        self.processed_cells_count as usize
    }
    pub fn get_cells_per_second(&self) -> f64 {
        self.processed_cells_count as f64 / self.start_time.elapsed().as_secs_f64()
    }
}
