use super::cell::*;
use super::draw_manager::DrawManager;
use super::grid::*;
use super::util::*;
use glam::IVec2;
use rand::thread_rng;
use rayon::prelude::*;

pub struct CovarageGrid {
    pub inside_cells: Grid,
    neighbors: Vec<Cell>,
    pub all_visited_cells: Grid,
    new_neighbors: Grid,
    limit: usize,
    pub current_sample_count: usize,
    sample_per_update: usize,
    grid_size: usize,
}

impl CovarageGrid {
    pub fn new(limit: usize, sample_per_iter: usize, grid_size: usize) -> Self {
        let mut grid = CovarageGrid {
            inside_cells: Grid::new(grid_size),
            neighbors: Vec::new(),
            all_visited_cells: Grid::new(grid_size),
            new_neighbors: Grid::new(grid_size),
            limit,
            current_sample_count: 0,
            sample_per_update: sample_per_iter,
            grid_size,
        };
        let starting_x = (grid_size / 16) as i32;
        for x in starting_x..starting_x + (grid_size / 16) as i32 {
            let start = Cell::new(IVec2::new(x, 0), 0);
            grid.neighbors.push(start);
            grid.all_visited_cells.insert(start);
        }
        grid.sample_neighbors();
        grid
    }
    pub fn sample(&mut self) {
        if self.new_neighbors.is_empty() {
            self.sample_neighbors();
        } else {
            for _ in 0..100 {
                self.sample_new_neighbors();
            }
        }
    }
    pub fn sample_neighbors(&mut self) {
        self.current_sample_count += self.sample_per_update;
        assert!(self.new_neighbors.is_empty(), "new_neighbors isn't empty");
        let max_sample_count = self.current_sample_count.saturating_sub(1000);

        self.neighbors.retain(|cell| {
            !self.inside_cells.is_activ(*cell)
                && cell.get_starting_sample_count() >= max_sample_count
        });
        let new_inside_cells = self
            .neighbors
            .par_iter()
            .map_init(
                || thread_rng(),
                |rng, cell| {
                    if self.sample_cell(*cell, self.sample_per_update, rng) {
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
        self.neighbors
            .extend(self.new_neighbors.iter(self.current_sample_count));
    }
    pub fn sample_new_neighbors(&mut self) {
        if self.new_neighbors.is_empty() {
            return;
        }
        let new_inside_cells = self
            .new_neighbors
            .iter(self.current_sample_count)
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
        self.new_neighbors.clear();
        for inside_cell in new_inside_cells {
            self.add_inside_cell(inside_cell);
        }
        self.neighbors
            .extend(self.new_neighbors.iter(self.current_sample_count));
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
        self.inside_cells.insert(cell);
        for neighbor in cell.get_neighbors(self.current_sample_count) {
            if !self.all_visited_cells.is_activ(neighbor) {
                self.new_neighbors.insert(neighbor);
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
                cell.draw(RED, self.grid_size);
            }
        }
        if draw_manager.get_draw_new_neighbors() {
            self.new_neighbors.draw(BLUE);
        }
    }
    pub fn area(&self) -> f64 {
        self.inside_cells.activ_count() as f64 * Cell::area(self.grid_size)
    }
    pub fn real_covered_area(&self) -> f64 {
        let samples_per_cell = 1000;
        let total_samples = self.inside_cells.activ_count() * samples_per_cell;

        let inside_samples = self
            .inside_cells
            .iter(0)
            .par_bridge()
            .map_init(
                || thread_rng(),
                |rng, cell| {
                    let mut inside_samples = 0;
                    for _ in 0..samples_per_cell / 4 {
                        let (inside_limit, inside_set) =
                            raw_quad_inside_test(cell, self.limit, self.grid_size, rng);
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
}
