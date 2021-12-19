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

// use covarage_grid_gen::CovarageGridGen;
// use kludgine::prelude::SingleWindowApplication;
use sample_cells::*;
use std::{fs, path::Path, thread, time::Duration};
use glam::DVec2;

pub struct CovarageGrid {
    cells: Vec<cell::Cell>,
    size: usize,
}
impl CovarageGrid {
    pub fn get_covarag_grid(
        size: usize,
        limit: usize,
        samples_per_cell: usize,
        sample_limit: usize,
    ) -> CovarageGrid {
        let file_name = CovarageGrid::get_file_name(size, limit, samples_per_cell);
        let path = Path::new(&file_name);

        if !path.exists() {
            CovarageGrid::gen_sample_cells(size, limit, samples_per_cell);
        }
        while !path.exists() {
            thread::sleep(Duration::from_millis(100));
        }

        let sampled_cells_data = fs::read(path).unwrap();
        let sample_cells: SampleCells = bincode::deserialize(&sampled_cells_data).unwrap();

        let cells = sample_cells.to_cells(sample_limit);

        CovarageGrid { cells, size }
    }
    pub fn gen_sample_cells(_size: usize, _limit: usize, _samples_per_cell: usize) {
        todo!();
        // let camera = CameraManger::new(
        //     true,
        //     CovarageGridGen::new(
        //         limit,
        //         samples_per_cell,
        //         size,
        //         CovarageGrid::get_file_name(size, limit, samples_per_cell),
        //     ),
        // );
        // SingleWindowApplication::run(camera);
        // let window_builder = WindowBuilder::default()
        //     .with_title("Genarating Covarage Grid")
        //     .with_size((WIDTH as u32, HEIGHT as u32).into());
        // Runtime::open_window(window_builder, camera);
    }
    pub fn get_file_name(size: usize, limit: usize, samples_per_cell: usize) -> String {
        format!(
            "./gridsize: {}, limit: {}, samples_per_cells: {}.cells",
            size, limit, samples_per_cell,
        )
    }
    pub fn gen_samples(&self, target: &mut Vec<DVec2>) {
        let rng = &mut rand::thread_rng();
        let new_samples = self.cells.iter().cycle().flat_map(|cell|
            {
                let poss_samples = [
                    cell.gen_point_inside(self.size, rng),
                    cell.gen_point_inside(self.size, rng),
                    cell.gen_point_inside(self.size, rng),
                    cell.gen_point_inside(self.size, rng),
                ];
                let iteraion_counts = iterate_points_dvec2(&poss_samples, 100);

                std::iter::zip(iteraion_counts, poss_samples).filter_map(|(iteration_count, point)| {
                    if 30 < iteration_count && iteration_count < 100 {
                        Some(point)
                    } else {
                        None
                    }
                })
            }
        );
        for (new_sample, old_sample) in new_samples.zip(target.iter_mut()) {
            *old_sample = new_sample;
        }
    }
}
