use super::cell::Cell;
pub struct SaveCell {
    cell: Cell,
    highest_iteration: u16,
}
impl SaveCell {
    pub fn new(cell: Cell, highest_iteration: u16) -> Self {
        SaveCell {
            cell,
            highest_iteration,
        }
    }
    pub fn get_cell(&self) -> Cell {
        self.cell
    }
}