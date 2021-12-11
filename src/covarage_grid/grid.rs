use super::{camera::*, cell::*};
use macroquad::color::Color;
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
    pub fn draw(&self, color: Color, camera: &CameraManger) {
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
    // fn test_grid_new() {
    //     let grid = Grid::new(GRID_SIZE);
    //     assert_eq!(grid.collums.len(), GRID_SIZE);
    // }
    // #[test]
    // fn new_grid_collums_empty() {
    //     let grid = Grid::new(GRID_SIZE);
    //     for collum in grid.collums.iter() {
    //         assert!(collum.is_empty());
    //     }
    // }
    // #[test]
    // fn test_grid_insert() {
    //     let mut grid = Grid::new(GRID_SIZE);
    //     grid.insert(Cell::new(ivec2(10, 5)));
    //     assert!(grid.collums[GRID_SIZE / 2 + 11].is_activ(GRID_SIZE / 2 + 5));
    // }
    #[test]
    fn cell_activ_afte_insertion() {
        let mut grid = Grid::new(GRID_SIZE);
        grid.insert(Cell::new(ivec2(10, 5)));
        assert!(grid.is_activ(Cell::new(ivec2(10, 5))));
    }
}
