use super::cell::Cell;
pub struct SampledCell {
    cell: Cell,
    highest_iteration: u16,
}
impl SampledCell {
    pub fn new(cell: Cell, highest_iteration: u16) -> Self {
        SampledCell {
            cell,
            highest_iteration,
        }
    }
    pub fn get_cell(&self) -> Cell {
        self.cell
    }
    pub fn get_highest_iteration(&self) -> u16 {
        self.highest_iteration
    }
}
