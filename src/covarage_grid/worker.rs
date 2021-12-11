use super::cell::*;
use super::sampled_cell::SampledCell;
use super::mandel_iter::MultiMandelIterator;
use core_simd::i64x4;
use rand::prelude::{thread_rng, ThreadRng};
use std::sync::mpsc;

pub struct Worker {
    cell_to_sample: spmc::Receiver<Cell>,
    cell_that_are_inside: mpsc::Sender<SampledCell>,
    current_cells: [Cell; 4],
    sampels: usize,
    grid_size: usize,
    limit: usize,
    rng: ThreadRng,
    finished: bool,
}
impl Worker {
    pub fn start(
        cell_to_work_on: spmc::Receiver<Cell>,
        cell_that_are_inside: mpsc::Sender<SampledCell>,
        limit: usize,
        sampels: usize,
        grid_size: usize,
    ) {
        let mut worker = Self {
            cell_to_sample: cell_to_work_on,
            cell_that_are_inside,
            current_cells: [Cell::dummy(); 4],
            sampels: sampels / 4,
            grid_size,
            limit,
            rng: thread_rng(),
            finished: false,
        };
        worker.work()
    }
    fn replace_current_cells(&mut self) {
        let new_cell_with_negativ_y = loop {
            if let Ok(new_cell) = self.cell_to_sample.recv() {
                if new_cell.is_y_negativ() {
                    break new_cell;
                }
            } else {
                self.finished = true;
                return;
            }
        };
        self.current_cells = [new_cell_with_negativ_y; 4];
    }
    fn send_current_cells(&mut self, max_iteration: i64) {
        if self
            .cell_that_are_inside
            .send(SampledCell::new(
                self.current_cells[0],
                max_iteration as u16,
                self.grid_size,
            ))
            .is_err()
        {
            self.finished = true;
        }
    }
    pub fn work(&mut self) {
        let mut max_iterations = i64x4::splat(0);
        let mut current_iterations = 0;
        let mut any_inside = false;
        while !self.finished {
            let (iterations, is_inside) = quad_test_in_cell(
                self.current_cells,
                self.limit,
                self.grid_size,
                1024,
                &mut self.rng,
            );
            max_iterations = max_iterations.max(iterations);
            any_inside |= is_inside;
            current_iterations += 1;
            if current_iterations == self.sampels {
                if any_inside {
                    self.send_current_cells(max_iterations.horizontal_max());
                }
                self.replace_current_cells();
                max_iterations = i64x4::splat(0);
                current_iterations = 0;
                any_inside = false;
            }
        }
    }
}


pub fn quad_test_in_cell(
    cells: [Cell; 4],
    limit: usize,
    grid_size: usize,
    iteration_depth: usize,
    rng: &mut ThreadRng,
) -> (i64x4, bool) {
    let inside_points = [
        cells[0].gen_point_inside(grid_size, rng),
        cells[1].gen_point_inside(grid_size, rng),
        cells[2].gen_point_inside(grid_size, rng),
        cells[3].gen_point_inside(grid_size, rng),
    ];
    let x = [
        inside_points[0].x,
        inside_points[1].x,
        inside_points[2].x,
        inside_points[3].x,
    ];
    let y = [
        inside_points[0].y,
        inside_points[1].y,
        inside_points[2].y,
        inside_points[3].y,
    ];
    let mut mandel_iter = MultiMandelIterator::new(x, y);
    mandel_iter.iterate(iteration_depth);

    (
        mandel_iter.raw_get_iterations(),
        mandel_iter.is_inside(limit, iteration_depth).is_some(),
    )
}
