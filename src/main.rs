#![allow(dead_code, unused_variables)]
#![warn(clippy::nursery)]
#![feature(test, portable_simd, iter_zip, array_chunks)]

mod buddha;
mod camera;
mod covarage_grid;
mod histogram;
mod mandel_brot_render;
mod mandel_iter;
mod pixels;
mod sample_checker;
mod sample_gen;
mod sample_mutator;
mod worker;

use buddha::Buddha;
use camera::*;
use covarage_grid::CovarageGrid;
use lazy_static::*;

const SIZE: usize = 1024;
pub const WIDTH: usize = SIZE;
pub const HEIGHT: usize = (SIZE as f64 * (2.64 / 3.0)) as usize;
pub const MIN_ITER: usize = 50;
pub const MAX_ITER: usize = 100;
lazy_static! {
    static ref COVARAGE_GRID: CovarageGrid = CovarageGrid::get_covarag_grid(10_000, 30, 10_000, 30);
}

fn main() {
    let buddha = Buddha::new(100, ViewRect::default(), &COVARAGE_GRID);
    let test = CameraManger::start(false, buddha);
}
