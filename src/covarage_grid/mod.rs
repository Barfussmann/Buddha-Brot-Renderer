pub mod cell;
pub mod covarage_grid_gen;
mod grid;
mod range;
mod range_encoder;
mod sample_cells;
mod sampled_cell;
mod worker;

use super::camera;
use super::mandel_iter;

use glam::IVec2;

pub struct CovarageGrid {
    cells: Vec<cell::Cell>,
    size: usize,
}
impl CovarageGrid {
    pub fn new(size: usize, cells: Vec<cell::Cell>) -> Self {
        Self { cells, size }
    }
    pub fn draw(&self, camera: &camera::CameraManger) {
        for cell in self.cells.iter() {
            cell.draw(self.size, camera);
        }
    }
}
