use super::{cell::Cell, sampled_cell::*};
use serde::{Deserialize, Serialize};
use rand::seq::SliceRandom;

#[derive(Serialize, Deserialize, Debug)]
pub struct SampleCells {
    pub cells: Vec<SampledCell>,
    pub size: usize,
}
impl SampleCells {
    pub fn new(cells: Vec<SampledCell>, size: usize) -> Self {
        Self { cells, size }
    }
    pub fn to_cells(&self, limit: usize) -> Vec<Cell> {
        let mut cells: Vec<Cell> = self.cells
            .iter()
            .take_while(|a| a.get_highest_iteration() as usize >= limit)
            .map(|a| a.get_cell(self.size))
            .collect();
        cells.shuffle(&mut rand::thread_rng());
        cells

    }
}
