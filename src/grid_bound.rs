use super::grid::*;
use super::util::*;
use glam::DVec2 as Vec2;
use glam::IVec2;
use rand::thread_rng;
use rayon::prelude::*;
use super::draw_manager::DrawManager;

pub const GRID_SIZE: usize = 10000;
pub const SIDE_LENGTH: f64 = 4. / GRID_SIZE as f64;

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
    fn get_center(&self) -> Vec2 {
        self.center.as_dvec2() * SIDE_LENGTH
    }
    pub fn gen_point_inside(&self, rng: &mut ThreadRng) -> Vec2 {
        gen_point_in_square(self.get_center(), SIDE_LENGTH, rng)
    }
    fn get_neighbors(&self, current_sample_count: usize) -> Vec<Cell> {
        vec![
            Cell::new(self.center + IVec2::new(1, 0), current_sample_count),
            Cell::new(self.center + IVec2::new(0, 1), current_sample_count),
            Cell::new(self.center + IVec2::new(-1, 0), current_sample_count),
            Cell::new(self.center + IVec2::new(0, -1), current_sample_count),
        ]
    }
    fn draw(&self, color: Color) {
        draw_square(self.get_center(), SIDE_LENGTH * 1., color);
    }
    fn area() -> f64 {
        SIDE_LENGTH * SIDE_LENGTH
    }
    pub fn index(&self) -> (usize, usize) {
        let index = self.center + IVec2::splat((GRID_SIZE / 2) as i32);
        (index.x as usize, index.y as usize)
    }
}

pub struct CovarageGrid {
    inside_cells: Grid,
    pub neighbors: Vec<Cell>,
    neighbors_clone: Vec<Cell>,
    all_visited_cells: Grid,
    pub new_neighbors: Vec<Cell>,
    limit: usize,
    pub total_sample_count: usize,
    sample_per_iter: usize,
}

impl CovarageGrid {
    pub fn new(limit: usize, sample_per_iter: usize) -> Self {
        let mut grid = CovarageGrid {
            inside_cells: Grid::new(),
            neighbors: Vec::new(),
            neighbors_clone: Vec::new(),
            all_visited_cells: Grid::new(),
            new_neighbors: Vec::new(),
            limit,
            total_sample_count: 0,
            sample_per_iter,
        };
        let starting_x = (GRID_SIZE / 16) as i32;
        for x in starting_x..starting_x + (GRID_SIZE / 16) as i32 {
            let start = Cell::new(IVec2::new(x, 0), 0);
            grid.neighbors.push(start);
            grid.all_visited_cells.insert(start);
        }
        grid
    }
    pub fn sample(&mut self) {
        if self.new_neighbors.is_empty() {
            self.sample_neighbors();
        } else {
            for _ in 0..10 {
                self.sample_new_neighbors();
            }
        }
    }
    pub fn sample_neighbors(&mut self) {
        self.total_sample_count += self.sample_per_iter;
        assert!(self.new_neighbors.is_empty(), "new_neighbors isn't empty");
        let max_sample_count = self.total_sample_count.saturating_sub(1000);
        std::mem::swap(&mut self.neighbors, &mut self.neighbors_clone);
        self.neighbors.clear();

        self.neighbors
            .par_extend(self.neighbors_clone.par_iter().copied().filter(|cell| {
                !self.inside_cells.is_activ(*cell) && cell.starting_sample_count >= max_sample_count
            }));
        let new_inside_cells = self
            .neighbors
            .par_iter()
            .map_init(
                || thread_rng(),
                |rng, cell| {
                    if self.sample_cell(*cell, self.sample_per_iter, rng) {
                        Some(*cell)
                    } else {
                        None
                    }
                },
            )
            .filter_map(|cell| cell)
            .collect::<Vec<_>>();
        for new_inside_cell in new_inside_cells {
            self.add_inside_cell(new_inside_cell);
        }
        self.neighbors.extend(self.new_neighbors.iter().cloned());
    }
    pub fn sample_new_neighbors(&mut self) {
        let rng = &mut thread_rng();
        let new_neighbors_copy = self.new_neighbors.clone();
        self.new_neighbors.clear();
        for cell in new_neighbors_copy {
            if quad_inside_test(cell, self.limit, rng) {
                self.add_inside_cell(cell);
            }
        }
        self.neighbors.extend(self.new_neighbors.iter().cloned());
    }
    #[inline(always)]
    fn sample_cell(&self, cell: Cell, count: usize, rng: &mut ThreadRng) -> bool {
        for _ in 0..count {
            if quad_inside_test(cell, self.limit, rng) {
                return true;
            }
        }
        false
    }
    fn add_inside_cell(&mut self, cell: Cell) {
        self.inside_cells.insert(cell);
        for neighbor in cell.get_neighbors(self.total_sample_count) {
            if !self.all_visited_cells.is_activ(neighbor) {
                self.new_neighbors.push(neighbor);
                self.all_visited_cells.insert(neighbor);
            }
        }
    }
    pub fn draw(&self, draw_manager: &DrawManager) {
        if draw_manager.get_draw_all_visited_cells() {
            self.all_visited_cells.draw(PINK);
        }
        if draw_manager.get_draw_inside_cells() {
            self.inside_cells.draw(GREEN);
        }
        if draw_manager.get_draw_neighbors() {
            for cell in self.neighbors.iter() {
                cell.draw(RED);
            }
        }
        if draw_manager.get_draw_new_neighbors() {
            for cell in self.new_neighbors.iter() {
                cell.draw(BLUE);
            }
        }
    }
    pub fn area(&self) -> f64 {
        self.inside_cells.activ_count() as f64 * Cell::area()
    }
}
