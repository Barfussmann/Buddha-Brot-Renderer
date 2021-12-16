pub mod cell;
mod covarage_grid_gen;
mod grid;
mod sample_cells;
mod sampled_cell;
mod worker;

use super::{HEIGHT, WIDTH};

use super::camera;
use super::camera::*;
use super::mandel_iter;

use covarage_grid_gen::CovarageGridGen;
use kludgine::prelude::{Runtime, SingleWindowApplication, WindowBuilder};
use sample_cells::*;
use std::{fs, path::Path, thread, time::Duration};

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
            CovarageGrid::gen_sample_cells(size, limit, samples_per_cell)
        }
        while !path.exists() {
            thread::sleep(Duration::from_millis(100));
        }

        let sampled_cells_data = fs::read(path).unwrap();
        let sample_cells: SampleCells = bincode::deserialize(&sampled_cells_data).unwrap();

        let cells = sample_cells.to_cells(sample_limit);

        CovarageGrid { cells, size }
    }
    pub fn gen_sample_cells(size: usize, limit: usize, samples_per_cell: usize) {
        let camera = CameraManger::new(
            true,
            CovarageGridGen::new(
                limit,
                samples_per_cell,
                size,
                CovarageGrid::get_file_name(size, limit, samples_per_cell),
            ),
        );
        let window_builder = WindowBuilder::default()
            .with_title("Genarating Covarage Grid")
            .with_size((WIDTH as u32, HEIGHT as u32).into());
        println!("test");
        SingleWindowApplication::run(camera);
        // Runtime::open_window(window_builder, camera);
    }
    pub fn get_file_name(size: usize, limit: usize, samples_per_cell: usize) -> String {
        format!(
            "./gridsize: {}, limit: {}, samples_per_cells: {}.cells",
            size, limit, samples_per_cell,
        )
    }
}
