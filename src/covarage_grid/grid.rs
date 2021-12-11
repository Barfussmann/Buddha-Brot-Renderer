use super::{camera::*, cell::*, range_encoder::*};
use macroquad::color::Color;

#[derive(Clone, Debug)]
pub struct Grid {
    collums: Vec<RangeEncoder>,
    grid_size: usize,
}
impl Grid {
    pub fn new(grid_size: usize) -> Self {
        Self {
            collums: vec![RangeEncoder::new(); grid_size],
            grid_size,
        }
    }
    pub fn insert(&mut self, cell: Cell) {
        let (x, y) = cell.index_2d(self.grid_size);
        self.collums[x].insert_index(y);
    }
    pub fn is_activ(&self, cell: Cell) -> bool {
        let (x, y) = cell.index_2d(self.grid_size);
        self.collums[x].is_activ(y)
    }
    pub fn draw(&self, color: Color, camera: &CameraManger) {
        for (x, collum) in self.collums.iter().enumerate() {
            collum.draw(x, color, self.grid_size, camera);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use glam::ivec2;
    const GRID_SIZE: usize = 100;
    #[test]
    fn test_grid_new() {
        let grid = Grid::new(GRID_SIZE);
        assert_eq!(grid.collums.len(), GRID_SIZE);
    }
    #[test]
    fn new_grid_collums_empty() {
        let grid = Grid::new(GRID_SIZE);
        for collum in grid.collums.iter() {
            assert!(collum.is_empty());
        }
    }
    #[test]
    fn test_grid_insert() {
        let mut grid = Grid::new(GRID_SIZE);
        grid.insert(Cell::new(ivec2(10, 5)));
        assert!(grid.collums[GRID_SIZE / 2 + 11].is_activ(GRID_SIZE / 2 + 5));
    }
    #[test]
    fn cell_activ_afte_insertion() {
        let mut grid = Grid::new(GRID_SIZE);
        grid.insert(Cell::new(ivec2(10, 5)));
        assert!(grid.is_activ(Cell::new(ivec2(10, 5))));
    }
}
