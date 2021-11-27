use glam::DVec2 as Vec2;
use glam::IVec2;
use super::util::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Cell {
    center: IVec2,
    starting_sample_count: usize,
}

impl Cell {
    pub fn new(center: IVec2, starting_sample_count: usize) -> Self {
        Cell {
            center,
            starting_sample_count,
        }
    }
    fn get_center(&self, grid_size: usize) -> Vec2 {
        self.center.as_dvec2() * Cell::side_length(grid_size)
    }
    pub fn gen_point_inside(&self, grid_size: usize, rng: &mut ThreadRng) -> Vec2 {
        gen_point_in_square(self.get_center(grid_size), Cell::side_length(grid_size), rng)
    }
    pub fn get_neighbors(&self, current_sample_count: usize) -> Vec<Cell> {
        vec![
            Cell::new(self.center + IVec2::new(1, 0), current_sample_count),
            Cell::new(self.center + IVec2::new(0, 1), current_sample_count),
            Cell::new(self.center + IVec2::new(-1, 0), current_sample_count),
            Cell::new(self.center + IVec2::new(0, -1), current_sample_count),
        ]
    }
    pub fn draw(&self, color: Color, grid_size: usize) {
        draw_square(self.get_center(grid_size), Cell::side_length(grid_size), color);
    }
    pub fn area(grid_size: usize) -> f64 {
        Cell::side_length(grid_size) * Cell::side_length(grid_size)
    }
    pub fn index(&self, grid_size: usize) -> (usize, usize) {
        let index = self.center + IVec2::splat((grid_size / 2) as i32);
        (index.x as usize, index.y as usize)
    }
    fn side_length(grid_size: usize) -> f64 {
        4. / grid_size as f64
    }
    pub fn get_starting_sample_count(&self) -> usize {
        self.starting_sample_count
    }
}