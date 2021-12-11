use super::util::*;
use super::camera::*;
use glam::DVec2 as Vec2;
use glam::IVec2;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Cell {
    center: IVec2,
}

impl Cell {
    pub fn new(center: IVec2) -> Self {
        Cell { center }
    }
    pub fn dummy() -> Self {
        Cell {
            center: IVec2::new(0, 0),
        }
    }
    fn get_corner(&self, grid_size: usize) -> Vec2 {
        self.center.as_dvec2() * Cell::side_length(grid_size)
    }
    pub fn gen_point_inside(&self, grid_size: usize, rng: &mut ThreadRng) -> Vec2 {
        gen_point_in_square(
            self.get_corner(grid_size),
            Cell::side_length(grid_size),
            rng,
        )
    }
    pub fn get_neighbors(&self) -> Vec<Cell> {
        vec![
            Cell::new(self.center + IVec2::new(1, 0)),
            Cell::new(self.center + IVec2::new(0, 1)),
            Cell::new(self.center + IVec2::new(-1, 0)),
            Cell::new(self.center + IVec2::new(0, -1)),
        ]
    }
    pub fn area(grid_size: usize) -> f64 {
        Cell::side_length(grid_size) * Cell::side_length(grid_size)
    }
    pub fn from_index(index: usize, grid_size: usize) -> Cell {
        let x = index % grid_size;
        let y = index / grid_size;
        Cell::from_index_2d(x, y, grid_size)
    }
    pub fn from_index_2d(x: usize, y: usize, grid_size: usize) -> Self {
        let offset = IVec2::splat(grid_size as i32 / 2);
        let center = IVec2::new(x as i32 - 1, y as i32) - offset;
        Cell { center }
    }
    pub fn index(&self, grid_size: usize) -> usize {
        let (x, y) = self.index_2d(grid_size);
        (y * grid_size) + x
    }
    pub fn index_2d(&self, grid_size: usize) -> (usize, usize) {
        let index = self.center + IVec2::splat((grid_size / 2) as i32);
        ((index.x + 1) as usize, index.y as usize)
    }
    pub fn is_y_negativ(&self) -> bool {
        self.center.y <= 0
    }
    fn side_length(grid_size: usize) -> f64 {
        4. / grid_size as f64
    }
    pub fn draw(&self, grid_size: usize, camera: &CameraManger) {
        let mut size = Vec2::splat(-Cell::side_length(grid_size));
        let mut corner = self.get_corner(grid_size);
        camera.draw_rect(corner, size, GREEN);
        corner.y *= -1.;
        size.y *= -1.;
        camera.draw_rect(corner, size, GREEN);
    }
}

mod tests {
    use super::*;
    #[test]
    fn from_index_form_index_is_same() {
        let cell = Cell::new(IVec2::new(1, 2));
        for grid_size in 100..1000 {
            let other = Cell::from_index(cell.index(grid_size), grid_size);
            assert_eq!(cell, other);
        }
    }
}