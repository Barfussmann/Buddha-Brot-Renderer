use super::camera::*;
use glam::{dvec2, DVec2, IVec2};
use rand::{rngs::ThreadRng, Rng};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Cell {
    corner: IVec2,
}

impl Cell {
    pub fn new(corner: IVec2) -> Self {
        Cell { corner }
    }
    pub fn dummy() -> Self {
        Cell {
            corner: IVec2::new(0, 0),
        }
    }
    pub fn get_corner(&self, grid_size: usize) -> DVec2 {
        self.corner.as_dvec2() * Cell::side_length(grid_size)
    }
    pub fn gen_point_inside(&self, grid_size: usize, rng: &mut ThreadRng) -> DVec2 {
        let side_length = Cell::side_length(grid_size);
        let corner = self.get_corner(grid_size);
        let offset = dvec2(
            rng.gen_range(0. ..side_length),
            rng.gen_range(0. ..side_length),
        );
        corner + offset
    }
    pub fn get_neighbors(&self) -> Vec<Cell> {
        vec![
            Cell::new(self.corner + IVec2::new(1, 0)),
            Cell::new(self.corner + IVec2::new(0, 1)),
            Cell::new(self.corner + IVec2::new(-1, 0)),
            Cell::new(self.corner + IVec2::new(0, -1)),
        ]
    }
    pub fn from_index(index: usize, grid_size: usize) -> Cell {
        let x = index % grid_size;
        let y = index / grid_size;
        Cell::from_index_2d(x, y, grid_size)
    }
    pub fn from_index_2d(x: usize, y: usize, grid_size: usize) -> Self {
        let offset = IVec2::splat(grid_size as i32 / 2);
        let center = IVec2::new(x as i32 - 1, y as i32) - offset;
        Cell { corner: center }
    }
    pub fn index(&self, grid_size: usize) -> usize {
        let (x, y) = self.index_2d(grid_size);
        (y * grid_size) + x
    }
    pub fn index_2d(&self, grid_size: usize) -> (usize, usize) {
        let index = self.corner + IVec2::splat((grid_size / 2) as i32);
        ((index.x + 1) as usize, index.y as usize)
    }
    pub fn is_y_negativ(&self) -> bool {
        self.corner.y < 0
    }
    pub fn side_length(grid_size: usize) -> f64 {
        4. / grid_size as f64
    }
}

mod tests {
    #[test]
    fn from_index_form_index_is_same() {
        let cell = super::Cell::new(super::IVec2::new(1, 2));
        for grid_size in 100..1000 {
            let other = super::Cell::from_index(cell.index(grid_size), grid_size);
            assert_eq!(cell, other);
        }
    }
}
