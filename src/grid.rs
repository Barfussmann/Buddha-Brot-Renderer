use super::grid_bound::*;
use super::range_encoder::*;
use macroquad::color::Color;

pub struct Grid {
    collums: Vec<RangeEncoder>,
}
impl Grid {
    pub fn new() -> Self {
        Self {
            collums: vec![RangeEncoder::new(); GRID_SIZE],
        }
    }
    pub fn insert(&mut self, cell: Cell) {
        let (x, y) = cell.index();
        self.collums[x].insert(y);
    }
    pub fn activ_count(&self) -> usize {
        self.collums.iter().map(|c| c.activ_count()).sum()
    }
    pub fn is_activ(&self, cell: Cell) -> bool {
        let (x, y) = cell.index();
        self.collums[x].is_activ(y)
    }
    pub fn draw(&self, color: Color) {
        for (x, collum) in self.collums.iter().enumerate() {
            collum.draw(x, color);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use glam::ivec2;
    #[test]
    fn test_grid_new() {
        let grid = Grid::new();
        assert_eq!(grid.collums.len(), GRID_SIZE);
    }
    #[test]
    fn new_grid_collums_empty() {
        let grid = Grid::new();
        for collum in grid.collums.iter() {
            assert!(collum.is_empty());
        }
    }
    #[test]
    fn test_grid_insert() {
        let mut grid = Grid::new();
        grid.insert(Cell::new(ivec2(10, 5), 0));
        assert!(grid.collums[GRID_SIZE / 2 + 10].is_activ(GRID_SIZE / 2 + 5));
    }
    #[test]
    fn activ_count_is_0_after_new() {
        let grid = Grid::new();
        assert_eq!(grid.activ_count(), 0);
    }
    #[test]
    fn activ_count_is_equal_to_inserted_cells() {
        let mut grid = Grid::new();
        let half_grid_size = (GRID_SIZE / 2) as i32;
        let mut counter = 0;
        for x in -half_grid_size..half_grid_size {
            grid.insert(Cell::new(ivec2(x, x), 0));
            counter += 1;
            assert!(grid.activ_count() == counter);
        }
    }
    #[test]
    fn cell_activ_afte_insertion() {
        let mut grid = Grid::new();
        grid.insert(Cell::new(ivec2(10, 5), 0));
        assert!(grid.is_activ(Cell::new(ivec2(10, 5), 0)));
    }
}
