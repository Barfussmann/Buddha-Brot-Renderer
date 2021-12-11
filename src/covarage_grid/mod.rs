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


use cell::Cell;
use camera::*;

pub struct CovarageGrid {
    cells: Vec<Cell>,
    size: usize,
}
impl CovarageGrid {
    pub fn new(size: usize, cells: Vec<Cell>) -> Self {
        Self {
            cells,
            size,
        }
    }
    pub fn draw(&self, camera: &CameraManger) {
        for cell in self.cells.iter() {
            cell.draw(self.size, camera);
        }
    }
}