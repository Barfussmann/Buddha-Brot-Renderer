pub mod cell;
pub mod covarage_grid_gen;
mod grid;
mod sample_cells;
mod sampled_cell;
mod worker;

use super::camera;
use super::mandel_brot_render::MandelbrotRender;
use super::mandel_iter;

use covarage_grid_gen::CovarageGridGen;
use sample_cells::*;
use std::fs;
use speedy2d::Graphics2D;

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
        graphics: &mut Graphics2D,
    ) -> CovarageGrid {
        let file_name = CovarageGrid::get_file_name(size, limit, samples_per_cell);
        let sample_cells: SampleCells = if let Ok(sampled_cells_data) = fs::read(file_name.clone())
        {
            bincode::deserialize(&sampled_cells_data).unwrap()
        } else {
            let sample_cells = CovarageGrid::gen_sample_cells(size, limit, samples_per_cell, graphics);
            let data = bincode::serialize(&sample_cells).unwrap();
            fs::write(file_name, &data).unwrap();
            sample_cells
        };
        let cells = sample_cells.to_cells(sample_limit);

        CovarageGrid { cells, size }
    }
    pub fn gen_sample_cells(size: usize, limit: usize, samples_per_cell: usize, graphics: &mut Graphics2D) -> SampleCells {
        let mut covarage_grid_gen = CovarageGridGen::new(limit, samples_per_cell, size);
        let mut camera = camera::CameraManger::new();
        let mut mandel_brot_render = MandelbrotRender::new();
        while !covarage_grid_gen.is_finished() {
            // camera.update();
            covarage_grid_gen.sample_neighbors();
            // mandel_brot_render.draw(&camera);
            covarage_grid_gen.draw(&camera, graphics);

        }
        covarage_grid_gen.to_complet_sampled_cells()
    }
    pub fn draw(&self, camera: &camera::CameraManger, graphics: &mut Graphics2D) {
        if !camera.draw_cells() {
            return;
        }
        for cell in self.cells.iter() {
            cell.draw(self.size, camera, graphics);
        }
    }
    fn get_file_name(size: usize, limit: usize, samples_per_cell: usize) -> String {
        format!(
            "./gridsize: {}, limit: {}, samples_per_cells: {}.cells",
            size.to_string(),
            limit.to_string(),
            samples_per_cell.to_string(),
        )
    }
}
