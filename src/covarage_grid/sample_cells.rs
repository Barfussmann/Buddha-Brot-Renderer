use super::CovarageGrid;
use super::sampled_cell::*;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct SampleCells {
    cells: Vec<SampledCell>,
    size: usize,
}
impl SampleCells {
    pub fn new(cells: Vec<SampledCell>, size: usize) -> Self {
        SampleCells { cells, size }
    }
    pub fn save(&self) {
        let encoded = bincode::serialize(&self).unwrap();
        fs::write("./sampeld_cells", encoded).unwrap();
    }
    pub fn load(&self) -> Self {
        let data = fs::read("./sampled_cells").unwrap();
        bincode::deserialize(&data).unwrap()
    }
    pub fn to_covarage_grid(&self, limit: usize) -> CovarageGrid {
        let cells = self
            .cells
            .iter()
            .take_while(|a| a.get_highest_iteration() as usize >= limit)
            .map(|a| a.get_cell(self.size))
            .collect();
        CovarageGrid::new(self.size, cells)
    }
}
