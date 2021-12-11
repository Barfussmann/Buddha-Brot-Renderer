use super::cell::Cell;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SampledCell {
    cell_index: usize,
    highest_iteration: u16,
}
impl SampledCell {
    pub fn new(cell: Cell, highest_iteration: u16, grid_size: usize) -> Self {
        SampledCell {
            cell_index: cell.index(grid_size),
            highest_iteration,
        }
    }
    pub fn get_cell(&self, grid_size: usize) -> Cell {
        Cell::from_index(self.cell_index, grid_size)
    }
    pub fn get_highest_iteration(&self) -> u16 {
        self.highest_iteration
    }
}
