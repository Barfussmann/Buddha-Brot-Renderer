use super::cell::Cell;
use super::camera::*;

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