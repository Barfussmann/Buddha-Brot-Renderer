use super::sampled_cell::*;
use serde::{Deserialize, Serialize};
use std::fs;
use bincode;

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletSampledCells {
    cells: Vec<SampledCell>,
    size: usize,
}
impl CompletSampledCells {
    pub fn new(cells: Vec<SampledCell>, size: usize) -> Self {
        CompletSampledCells { cells, size }
    }
    pub fn save(&self) {
        let encoded = bincode::serialize(&self).unwrap();
        fs::write("./sampeld_cells", encoded).unwrap();
    }
}