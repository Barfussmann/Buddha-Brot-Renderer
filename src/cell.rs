use super::camera::*;
use super::util::*;
use glam::DVec2 as Vec2;
use glam::IVec2;

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
    pub fn get_neighbors(&self, current_sample_count: usize) -> Vec<Cell> {
        vec![
            Cell::new(self.center + IVec2::new(1, 0), current_sample_count),
            Cell::new(self.center + IVec2::new(0, 1), current_sample_count),
            Cell::new(self.center + IVec2::new(-1, 0), current_sample_count),
            Cell::new(self.center + IVec2::new(0, -1), current_sample_count),
        ]
    }
    pub fn draw(&self, color: Color, grid_size: usize, camera: &CameraManger) {
        camera.draw_rect(
            self.get_corner(grid_size),
            -Vec2::splat(Cell::side_length(grid_size)),
            color,
        );
    }
    pub fn area(grid_size: usize) -> f64 {
        Cell::side_length(grid_size) * Cell::side_length(grid_size)
    }
    pub fn from_index(x: usize, y: usize, starting_sample_count: usize, grid_size: usize) -> Self {
        let offset = IVec2::splat(grid_size as i32 / 2);
        let center = IVec2::new(x as i32, y as i32) - offset;
        Cell {
            center,
            starting_sample_count,
        }
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
