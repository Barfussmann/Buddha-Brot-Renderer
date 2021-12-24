#![allow(dead_code, unused_variables)]
#![warn(
    clippy::nursery,
)]
#![feature(test, portable_simd, iter_zip, array_chunks)]

mod buddha;
mod camera;
mod covarage_grid;
mod mandel_brot_render;
mod mandel_iter;
mod sample_gen;
mod sample_mutator;

use buddha::Buddha;
use camera::*;
use covarage_grid::CovarageGrid;
use lazy_static::*;

const SIZE: usize = 1024;
pub const WIDTH: usize = SIZE;
pub const HEIGHT: usize = (SIZE as f64 * (2.64 / 3.0)) as usize;
lazy_static! {
    static ref COVARAGE_GRID: CovarageGrid = CovarageGrid::get_covarag_grid(10_000, 30, 1_000, 30);
}

fn main() {
    let buddha = Buddha::new(1000, ViewRect::default(), &COVARAGE_GRID);

    let test = CameraManger::start(false, buddha);
    // SingleWindowApplication::run(test);
}
