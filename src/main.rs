#![allow(dead_code, unused_variables, unused_imports)]
#![warn(
    clippy::branches_sharing_code,
    clippy::cognitive_complexity,
    clippy::option_if_let_else,
    clippy::suspicious_operation_groupings,
    clippy::useless_let_if_seq
)]
#![feature(test, portable_simd, iter_zip, array_chunks)]

mod buddha;
mod camera;
mod covarage_grid;
mod mandel_brot_render;
mod mandel_iter;

use covarage_grid::CovarageGrid;
use kludgine::prelude::*;
use buddha::Buddha;

const SIZE: usize = 1024;
pub const WIDTH: usize = SIZE;
pub const HEIGHT: usize = (SIZE as f64 * (2.64 / 3.0)) as usize;

fn main() {
    let covarage_grid = CovarageGrid::get_covarag_grid(10_000, 30, 100_000, 50);
    let saples = covarage_grid.gen_sapmles(1_000);
    // let buddha = Buddha::new(saples, 100, (Dvec()));
}
