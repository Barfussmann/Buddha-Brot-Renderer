use super::util::*;
use glam::DVec2 as Vec2;
use glam::IVec2;
use std::collections::HashSet;
// use coz::*;

const GRID_SIZE: usize = 100000;
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
    fn in_set(&self, limit: usize, rng: &mut ThreadRng) -> bool {
        // is_inside(&self.gen_point_inside(rng), limit)
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
        draw_square(self.get_center(), SIDE_LENGTH * 1., color);
    }
    fn area() -> f64 {
        SIDE_LENGTH * SIDE_LENGTH
    }
}

pub struct Grid {
    inside_cells: HashSet<Cell>,
    neighbors: HashSet<Cell>,
    all_cells: HashSet<Cell>,
    new_neighbors: HashSet<Cell>,
    limit: usize,
}

impl Grid {
    pub fn new(limit: usize) -> Self {
        let mut grid = Grid {
            inside_cells: HashSet::new(),
            neighbors: HashSet::new(),
            all_cells: HashSet::new(),
            new_neighbors: HashSet::new(),
            limit,
        };
        let start = Cell::new(IVec2::new((GRID_SIZE / 16) as i32, 0));
        grid.inside_cells.insert(start);
        grid.neighbors.extend(start.get_neighbors());
        grid.all_cells.insert(start);
        grid.all_cells.extend(grid.neighbors.iter().cloned());
        grid.new_neighbors.extend(grid.neighbors.iter().cloned());
        grid
    }
    pub fn sample_neighbors(&mut self, sample_per_cell: usize, rng: &mut ThreadRng) {
        self.new_neighbors.clear();
        for cell in self.neighbors.clone() {
            for _ in 0..sample_per_cell {
                if cell.in_set(self.limit, rng) {
                    self.add_inside_cell(cell);
                    break;
                }
            }
        }
        self.neighbors.extend(self.new_neighbors.iter().cloned());
    }
    pub fn sample_new_neighbors(&mut self, rng: &mut ThreadRng) {
        let new_neighbors_copy = self.new_neighbors.clone();
        self.new_neighbors.clear();
        for cell in new_neighbors_copy {
            if cell.in_set(self.limit, rng) {
                self.add_inside_cell(cell);
                continue;
            }
        }
        self.neighbors.extend(self.new_neighbors.iter().cloned());
    }
    fn add_inside_cell(&mut self, cell: Cell) {
        self.inside_cells.insert(cell);
        self.neighbors.remove(&cell);
        for neighbor in cell.get_neighbors() {
            if !self.all_cells.contains(&neighbor) {
                self.new_neighbors.insert(neighbor);
                self.all_cells.insert(neighbor);
            }
        }
    }
    pub fn draw(&self) {
        // for inside in self.inside_cells.iter() {
        //     inside.draw(GREEN);
        // }
        // for neighbors in self.neighbors.iter() {
        //     neighbors.draw(RED);
        // }
        for new_neighbors in self.new_neighbors.iter() {
            new_neighbors.draw(RED);
        }
    }
    pub fn area(&self) -> f64 {
        self.inside_cells.len() as f64 * Cell::area()
    }
    pub fn new_neighbor_len(&self) -> usize {
        self.new_neighbors.len()
    }
}
