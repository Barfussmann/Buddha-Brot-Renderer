use super::{camera::*, cell::*};
use glam::dvec2;
use macroquad::color::Color;
use macroquad::color::GREEN;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Grid {
    cells: HashSet<Cell>,
    cells_for_drawing: Vec<Cell>,
    last_sort: usize,
    grid_size: usize,
}
impl Grid {
    pub fn new(grid_size: usize) -> Self {
        Self {
            cells: HashSet::new(),
            cells_for_drawing: Vec::new(),
            grid_size,
            last_sort: 1,
        }
    }
    pub fn insert(&mut self, cell: Cell) {
        self.cells_for_drawing.push(cell);
        self.cells.insert(cell);
    }
    pub fn is_activ(&self, cell: Cell) -> bool {
        self.cells.contains(&cell)
    }
    pub fn draw(&mut self, camera: &CameraManger) {
        if self.cells_for_drawing.len() - self.last_sort > 100_000 {
            self.cells_for_drawing
                .sort_by_key(|cell| cell.index(self.grid_size));
            self.last_sort = self.cells_for_drawing.len();
        }

        let mut first_cell_in_block = Cell::dummy();
        let mut prev_index = 0;
        let side_length = Cell::side_length(self.grid_size);
        let color = Color::new(0., 1., 0., 0.5);
        let mut last_cell = Cell::dummy();
        for cell in self.cells_for_drawing.iter() {
            if prev_index + 1 != cell.index(self.grid_size) {
                let mut corner = first_cell_in_block.get_corner(self.grid_size);
                let x_height = last_cell.get_corner(self.grid_size).x - corner.x + side_length;

                camera.draw_rect(corner, dvec2(x_height, side_length), color);
                corner.y *= -1.;
                camera.draw_rect(corner, dvec2(x_height, -side_length), color);
                first_cell_in_block = *cell;
            }
            last_cell = *cell;
            prev_index = cell.index(self.grid_size);
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
