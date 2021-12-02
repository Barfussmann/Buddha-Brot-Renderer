use super::camera::*;
use super::cell::*;
use super::range_encoder::*;
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
        let (x, y) = cell.index(self.grid_size);
        self.collums[x].insert_index(y);
    }
    pub fn remove(&mut self, cell: Cell) {
        let (x, y) = cell.index(self.grid_size);
        self.collums[x].remove_index(y);
    }
    pub fn activ_count(&self) -> usize {
        self.collums.iter().map(|c| c.activ_count()).sum()
    }
    pub fn is_activ(&self, cell: Cell) -> bool {
        let (x, y) = cell.index(self.grid_size);
        self.collums[x].is_activ(y)
    }
    pub fn draw(&self, color: Color, camera: &CameraManger) {
        for (x, collum) in self.collums.iter().enumerate() {
            collum.draw(x, color, self.grid_size, camera);
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = Cell> + '_ {
        self.collums
            .iter()
            .enumerate()
            .map(move |(x, collum)| {
                collum
                    .iter()
                    .map(move |y| Cell::from_index(x, y, self.grid_size))
            })
            .flatten()
    }
    pub fn is_empty(&self) -> bool {
        self.collums.iter().all(|c| c.is_empty())
    }
    pub fn clear(&mut self) {
        for collum in self.collums.iter_mut() {
            collum.clear();
        }
    }
    pub fn get_collum(&self, index: usize) -> &RangeEncoder {
        &self.collums[index]
    }
    pub fn get_grid_size(&self) -> usize {
        self.grid_size
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
        assert!(grid.collums[GRID_SIZE / 2 + 10].is_activ(GRID_SIZE / 2 + 5));
    }
    #[test]
    fn activ_count_is_0_after_new() {
        let grid = Grid::new(GRID_SIZE);
        assert_eq!(grid.activ_count(), 0);
    }
    #[test]
    fn activ_count_is_equal_to_inserted_cells() {
        let mut grid = Grid::new(GRID_SIZE);
        let half_grid_size = (GRID_SIZE / 2) as i32;
        let mut counter = 0;
        for x in -half_grid_size..half_grid_size {
            grid.insert(Cell::new(ivec2(x, x)));
            counter += 1;
            assert!(grid.activ_count() == counter);
        }
    }
    #[test]
    fn cell_activ_afte_insertion() {
        let mut grid = Grid::new(GRID_SIZE);
        grid.insert(Cell::new(ivec2(10, 5)));
        assert!(grid.is_activ(Cell::new(ivec2(10, 5))));
    }
    #[test]
    fn new_range_encoder_is_empty() {
        let grid = Grid::new(GRID_SIZE);
        assert!(grid.is_empty());
    }
    #[test]
    fn grid_after_clear_empty() {
        let mut grid = Grid::new(GRID_SIZE);
        grid.insert(Cell::new(ivec2(10, 5)));
        grid.clear();
        assert!(grid.is_empty());
    }
    #[test]
    fn grid_is_empty_after_insert_and_ther_remove() {
        let mut grid = Grid::new(GRID_SIZE);
        let cells = vec![
            Cell::new(ivec2(10, 6)),
            Cell::new(ivec2(10, 5)),
            Cell::new(ivec2(10, 7)),
        ];
        for cell in cells.iter() {
            grid.insert(*cell);
        }
        for cell in cells.iter() {
            grid.remove(*cell);
        }
        assert!(grid.is_empty());
    }
}
