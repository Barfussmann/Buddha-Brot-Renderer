use super::camera::*;
use super::cell::*;
use super::draw_manager::DrawManager;
use super::grid::*;
use super::util::*;
use glam::IVec2;
use rand::thread_rng;
use rayon::prelude::*;

pub struct CovarageGrid {
    pub inside_cells: Grid,
    neighbors: Grid,
    limit: usize,
    sample_per_update: usize,
    grid_size: usize,
}

impl CovarageGrid {
    pub fn new(limit: usize, sample_per_iter: usize, grid_size: usize) -> Self {
        let mut grid = CovarageGrid {
            inside_cells: Grid::new(grid_size),
            neighbors: Grid::new(grid_size),
            limit,
            sample_per_update: sample_per_iter,
            grid_size,
        };
        let starting_x = (grid_size / 16) as i32;
        for x in starting_x..starting_x + (grid_size / 16) as i32 {
            let start = Cell::new(IVec2::new(x, 0));
            grid.neighbors.insert(start);
        }
        grid.sample_neighbors();
        grid
    }
    pub fn sample_neighbors(&mut self) {
        let new_inside_cells = self
            .neighbors
            .iter()
            .par_bridge()
            .map_init(
                || thread_rng(),
                |rng, cell| {
                    if self.sample_cell(cell, self.sample_per_update, rng) {
                        Some(cell)
                    } else {
                        None
                    }
                },
            )
            .filter_map(|cell| cell)
            .collect::<Vec<_>>();
            self.neighbors.clear();
        for new_inside_cell in new_inside_cells {
            self.add_inside_cell(new_inside_cell);
        }
    }
    #[inline(always)]
    fn sample_cell(&self, cell: Cell, count: usize, rng: &mut ThreadRng) -> bool {
        for _ in 0..count {
            if quad_inside_test(cell, self.limit, self.grid_size, rng) {
                return true;
            }
        }
        false
    }
    fn add_inside_cell(&mut self, cell: Cell) {
        for neighbor in cell.get_neighbors() {
            if !self.chech_if_neighbor_is_new(neighbor) {
                continue;
            }
            self.neighbors.insert(neighbor);
        }
        self.inside_cells.insert(cell);
    }
    /// Has to be called bevore cell are inserted
    fn chech_if_neighbor_is_new(&self, cell: Cell) -> bool {
        for neighbor in cell.get_neighbors() {
            if self.inside_cells.is_activ(neighbor) {
                return false;
            }
        }
        return true
    }
    pub fn area(&self) -> f64 {
        self.inside_cells.activ_count() as f64 * Cell::area(self.grid_size)
    }
    pub fn real_covered_area(&self) -> f64 {
        let samples_per_cell = 1000;
        let total_samples = self.inside_cells.activ_count() * samples_per_cell;

        let inside_samples = self
            .inside_cells
            .iter()
            .par_bridge()
            .map_init(
                || thread_rng(),
                |rng, cell| {
                    let mut inside_samples = 0;
                    for _ in 0..samples_per_cell / 4 {
                        let (inside_limit, inside_set) =
                            unsafe { raw_quad_inside_test(cell, self.limit, self.grid_size, rng) };
                        inside_samples += std::iter::zip(inside_limit, inside_set)
                            .filter_map(|(in_limit, in_set)| (in_limit && !in_set).then(|| true))
                            .count();
                    }
                    return inside_samples;
                },
            )
            .sum::<usize>();
        let total_area = self.area();
        println!("sample: {} million", total_samples / 1_000_000);
        total_area * (inside_samples as f64 / total_samples as f64)
    }
    pub fn draw(&self, draw_manager: &DrawManager, camera: &CameraManger) {
        if draw_manager.get_draw_inside_cells() {
            self.inside_cells.draw(GREEN, camera);
        }
        if draw_manager.get_draw_neighbors() {
            self.neighbors.draw(RED, camera);
        }
    }
}
