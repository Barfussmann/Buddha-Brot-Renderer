use super::{camera::*, cell::*};
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Grid {
    cells: HashSet<Cell>,
    grid_size: usize,
}
impl Grid {
    pub fn new(grid_size: usize) -> Self {
        Self {
            cells: HashSet::new(),
            grid_size,
        }
    }
    pub fn insert(&mut self, cell: Cell) {
        self.cells.insert(cell);
    }
    pub fn is_activ(&self, cell: Cell) -> bool {
        self.cells.contains(&cell)
    }
    pub fn draw(&self, camera: &CameraManger) {
        for cell in self.cells.iter() {
            cell.draw(self.grid_size, camera);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use glam::ivec2;
    const GRID_SIZE: usize = 100;
    #[test]
    #[test]
    fn cell_activ_afte_insertion() {
        let mut grid = Grid::new(GRID_SIZE);
        grid.insert(Cell::new(ivec2(10, 5)));
        assert!(grid.is_activ(Cell::new(ivec2(10, 5))));
    }
}
