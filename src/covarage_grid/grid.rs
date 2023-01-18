use super::camera::ViewRect;
use super::cell::*;
use crate::{HEIGHT, WIDTH};
use glam::dvec2;
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
    pub fn draw(&mut self, view: ViewRect) -> Vec<u32> {
        let step_size = view.view_size / dvec2(WIDTH as f64, HEIGHT as f64);

        let mut pixels = vec![0; WIDTH * HEIGHT];

        let start = view.top_left_corner;
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let pos = start + step_size * dvec2(x as f64, y as f64);
                let cell = Cell::from_pos(pos, self.grid_size);
                if self.cells.contains(&cell) {
                    pixels[x + y * WIDTH] = 255 << 8;
                }
            }
        }
        pixels
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use glam::ivec2;
    const GRID_SIZE: usize = 100;
    #[test]
    fn cell_activ_afte_insertion() {
        let mut grid = Grid::new(GRID_SIZE);
        grid.insert(Cell::new(ivec2(10, 5)));
        assert!(grid.is_activ(Cell::new(ivec2(10, 5))));
    }
}
