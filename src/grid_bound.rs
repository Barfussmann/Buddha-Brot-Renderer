use super::util::*;
use glam::DVec2 as Vec2;
use glam::IVec2;
use std::collections::HashSet;
use coz::*;

const GRID_SIZE: usize = 1000;
const SIDE_LENGTH: f64 = 4. / GRID_SIZE as f64;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Cell {
    center: IVec2,
}

impl Cell {
    fn new(center: IVec2) -> Self {
        Cell { center }
    }
    fn get_center(&self) -> Vec2 {
        self.center.as_dvec2() * SIDE_LENGTH
    }
    fn gen_point_inside(&self, rng: &mut ThreadRng) -> Vec2 {
        gen_point_in_square(self.get_center(), SIDE_LENGTH, rng)
    }
    // true allway right. False can have false negatives
    fn test_if_in_set(&self, limit: usize, rng: &mut ThreadRng) -> bool {
        let point = self.gen_point_inside(rng);
        is_inside(&point, limit) && !is_any_cycle(point)
    }
    fn get_neighbors(&self) -> Vec<Cell> {
        vec![
            Cell::new(self.center + IVec2::new(1, 0)),
            Cell::new(self.center + IVec2::new(0, 1)),
            Cell::new(self.center + IVec2::new(-1, 0)),
            Cell::new(self.center + IVec2::new(0, -1)),
        ]
    }
    fn draw(&self, color: Color) {
        draw_square(self.get_center(), SIDE_LENGTH, color);
    }
    fn area() -> f64 {
        SIDE_LENGTH * SIDE_LENGTH
    }
}

pub struct Grid {
    inside_cells: HashSet<Cell>,
    neighbors: HashSet<Cell>,
    limit: usize,
}

impl Grid {
    pub fn new(limit: usize) -> Self {
        let mut grid = Grid {
            inside_cells: HashSet::new(),
            neighbors: HashSet::new(),
            limit,
        };
        let start = Cell::new(IVec2::new(((GRID_SIZE / 16) as i32), 0));
        grid.inside_cells.insert(start);
        grid.neighbors.extend(start.get_neighbors());
        grid
    }
    pub fn update_neighbors(&mut self, sample_per_cell: usize, rng: &mut ThreadRng) {
        let mut new_neighbors = HashSet::new();
        for cell in self.neighbors.iter() {
            for _ in 0..sample_per_cell {
                // progress!();
                if cell.test_if_in_set(self.limit, rng) {
                    self.inside_cells.insert(*cell);
                    new_neighbors.extend(cell.get_neighbors());
                    break;
                }
            }
        }
        self.neighbors.extend(new_neighbors);
        self.remove_inside_neighbors();
    }
    fn remove_inside_neighbors(&mut self) {
        self.neighbors = self
            .neighbors
            .difference(&self.inside_cells)
            .copied()
            .collect();
    }
    pub fn draw(&self) {
        for cell in self.inside_cells.iter() {
            cell.draw(GREEN);
        }
        for cell in self.neighbors.iter() {
            cell.draw(RED);
        }
    }
    pub fn area(&self) -> f64 {
        self.inside_cells.len() as f64 * Cell::area()
    }
    pub fn neighbor_len(&self) -> usize {
        self.neighbors.len()
    }
}
