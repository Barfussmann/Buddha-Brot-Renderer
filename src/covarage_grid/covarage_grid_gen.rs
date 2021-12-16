use super::{camera::*, cell::*, grid::*, sample_cells::*, sampled_cell::*, worker::*};
use glam::IVec2;
use std::fs;
use std::{
    sync::{mpsc, Mutex},
    thread,
    time::Instant,
};

pub struct CovarageGridGen {
    inside_cells: Grid,
    grid_size: usize,
    cell_to_sample: Mutex<spmc::Sender<Cell>>,
    cell_that_are_inside: Mutex<mpsc::Receiver<SampledCell>>,
    saved_cells: Vec<SampledCell>,
    last_cell_count_change: Instant,
    file_name_to_save_to: String,
}

impl CovarageGridGen {
    pub fn new(limit: usize, samples_per_cell: usize, grid_size: usize, file_name: String) -> Self {
        let (mut cell_to_sample_sender, cell_to_sample_receiver) = spmc::channel();
        let (cell_that_are_inside_sender, cell_that_are_inside_receiver) = mpsc::channel();
        for _ in 0..16 {
            let receiver = cell_to_sample_receiver.clone();
            let sender = cell_that_are_inside_sender.clone();
            thread::spawn(move || {
                Worker::start(receiver, sender, limit, samples_per_cell, grid_size)
            });
        }
        let starting_x = (grid_size / 16) as i32;
        for x in starting_x..=starting_x + (grid_size / 100) as i32 {
            cell_to_sample_sender
                .send(Cell::new(IVec2::new(x, -1)))
                .unwrap();
        }
        CovarageGridGen {
            inside_cells: Grid::new(grid_size),
            grid_size,
            cell_to_sample: Mutex::new(cell_to_sample_sender),
            cell_that_are_inside: Mutex::new(cell_that_are_inside_receiver),
            saved_cells: Vec::new(),
            last_cell_count_change: Instant::now(),
            file_name_to_save_to: file_name,
        }
    }
    pub fn sample_neighbors(&mut self) {
        let mut had_change = false;
        let start = Instant::now();
        while start.elapsed().as_millis() < 300 {
            for save_cell in self
                .cell_that_are_inside
                .lock()
                .unwrap()
                .try_iter()
                .take(10_000)
            {
                for neighbor in save_cell.get_cell(self.grid_size).get_neighbors() {
                    if !self.chech_if_neighbor_is_new(neighbor) {
                        continue;
                    }
                    had_change = true;
                    self.cell_to_sample
                        .get_mut()
                        .unwrap()
                        .send(neighbor)
                        .unwrap();
                }
                self.inside_cells.insert(save_cell.get_cell(self.grid_size));
                self.saved_cells.push(save_cell);
            }
        }
        if had_change {
            self.last_cell_count_change = Instant::now();
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
    // pub fn is_fi
    pub fn to_complet_sampled_cells(&self) -> SampleCells {
        assert!(self.is_finished(), "CovarageGridGen is not finished");
        let mut sorterd_saved_cells = self.saved_cells.clone();
        sorterd_saved_cells.sort_unstable_by_key(|b| std::cmp::Reverse(b.get_highest_iteration()));
        SampleCells::new(sorterd_saved_cells, self.grid_size)
    }
    pub fn is_finished(&self) -> bool {
        self.last_cell_count_change.elapsed().as_millis() > 500
    }
}

impl Updateable for CovarageGridGen {
    fn update(&mut self) {
        self.sample_neighbors();
    }
    fn draw(&mut self, drawer: &mut Drawer) {
        self.inside_cells.draw(drawer);
    }
    fn is_finished(&self) -> bool {
        self.is_finished()
    }
    fn finish(&mut self) {
        let file_name = self.file_name_to_save_to.clone();
        let sample_cells = self.to_complet_sampled_cells();
        let data = bincode::serialize(&sample_cells).unwrap();
        fs::write(file_name, &data).unwrap();
    }
}
