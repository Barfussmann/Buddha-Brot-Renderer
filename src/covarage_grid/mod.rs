pub mod cell;
pub mod covarage_grid_gen;
mod grid;
mod sample_cells;
mod sampled_cell;
mod worker;

use super::{WIDTH, HEIGHT};

use super::camera;
use super::camera::*;
use super::mandel_iter;

use covarage_grid_gen::CovarageGridGen;
use sample_cells::*;
use std::fs;

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
        let sample_cells: SampleCells = if let Ok(sampled_cells_data) = fs::read(file_name.clone())
        {
            bincode::deserialize(&sampled_cells_data).unwrap()
        } else {
            let sample_cells = CovarageGrid::gen_sample_cells(size, limit, samples_per_cell);
            let data = bincode::serialize(&sample_cells).unwrap();
            fs::write(file_name, &data).unwrap();
            sample_cells
        };
        let cells = sample_cells.to_cells(sample_limit);

        CovarageGrid { cells, size }
    }
    pub fn gen_sample_cells(size: usize, limit: usize, samples_per_cell: usize) -> SampleCells {
        // let camera = CameraManger::new(true, Box::new(CovarageGridGen::new(limit, samples_per_cell, size)));
        // let window = Window::new_centered("Mandelbrot", (WIDTH as u32, HEIGHT as u32)).unwrap();
        // window.run_loop(camera);
        todo!()
    }
    // pub fn draw(&self, rect_drawer: &mut RectDrawer) {
    //     for cell in self.cells.iter() {
    //         cell.draw(self.size, rect_drawer);
    //     }
    // }
    fn get_file_name(size: usize, limit: usize, samples_per_cell: usize) -> String {
        format!(
            "./gridsize: {}, limit: {}, samples_per_cells: {}.cells",
            size.to_string(),
            limit.to_string(),
            samples_per_cell.to_string(),
        )
    }
}
