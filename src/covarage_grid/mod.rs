pub mod cell;
mod covarage_grid_gen;
mod grid;
mod sample_cells;
mod sampled_cell;
mod worker;

use super::camera;
// use super::camera::*;
use super::mandel_iter;
use super::mandel_iter::*;

use glam::DVec2;
use sample_cells::*;
use std::{fs, path::Path, thread, time::Duration};


pub struct CovarageGrid {
    cells: Vec<cell::Cell>,
    gridsize: usize,
}
impl CovarageGrid {
    pub fn get_covarag_grid(
        grid_size: usize,
        limit: usize,
        samples_per_cell: usize,
        sample_limit: usize,
    ) -> Self {
        let file_name = Self::get_file_name(grid_size, limit, samples_per_cell);
        let path = Path::new(&file_name);

        if !path.exists() {
            Self::gen_sample_cells(grid_size, limit, samples_per_cell);
        }
        while !path.exists() {
            thread::sleep(Duration::from_millis(100));
        }

        let sampled_cells_data = fs::read(path).unwrap();
        let sample_cells: SampleCells = bincode::deserialize(&sampled_cells_data).unwrap();

        let cells = sample_cells.to_cells(sample_limit);
        println!("cells: {}", cells.len());

        Self {
            cells,
            gridsize: grid_size,
        }
    }
    pub fn gen_sample_cells(grid_size: usize, limit: usize, samples_per_cell: usize) {
        let covarage_grid_gen = covarage_grid_gen::CovarageGridGen::new(
            limit,
            samples_per_cell,
            grid_size,
            Self::get_file_name(grid_size, limit, samples_per_cell),
        );
        
        std::thread::spawn(move|| camera::CameraManger::start(true, covarage_grid_gen));
    }
    pub fn get_file_name(grid_size: usize, limit: usize, samples_per_cell: usize) -> String {
        format!(
            "./gridsize: {}, limit: {}, samples_per_cells: {}.cells",
            grid_size, limit, samples_per_cell,
        )
    }
    pub fn gen_samples(&self, target: &mut Vec<DVec2>) {
        let rng = &mut rand::thread_rng();
        let new_samples = self.cells.iter().cycle().flat_map(|cell| {
            let poss_samples = [
                cell.gen_point_inside(self.gridsize, rng),
                cell.gen_point_inside(self.gridsize, rng),
                cell.gen_point_inside(self.gridsize, rng),
                cell.gen_point_inside(self.gridsize, rng),
            ];
            let iteraion_counts = iterate_points_dvec2(&poss_samples, 100);

            std::iter::zip(iteraion_counts, poss_samples).filter_map(|(iteration_count, point)| {
                if 30 < iteration_count && iteration_count < 100 {
                    Some(point)
                } else {
                    None
                }
            })
        });
        for (new_sample, old_sample) in new_samples.zip(target.iter_mut()) {
            *old_sample = new_sample;
        }
    }
    pub const fn get_cells(&self) -> &Vec<cell::Cell> {
        &self.cells
    }
    pub const fn get_grid_size(&self) -> usize {
        self.gridsize
    }
}
