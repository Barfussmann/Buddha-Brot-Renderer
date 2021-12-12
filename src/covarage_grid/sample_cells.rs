use super::{cell::Cell, sampled_cell::*};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SampleCells {
    pub cells: Vec<SampledCell>,
    pub size: usize,
}
impl SampleCells {
    pub fn new(cells: Vec<SampledCell>, size: usize) -> Self {
        SampleCells { cells, size }
    }
    pub fn to_cells(&self, limit: usize) -> Vec<Cell> {
        self.cells
            .iter()
            .take_while(|a| a.get_highest_iteration() as usize >= limit)
            .map(|a| a.get_cell(self.size))
            .collect()
    }
}
