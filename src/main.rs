#![allow(dead_code)]
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

use buddha::Buddha;
use camera::*;
use covarage_grid::CovarageGrid;
use kludgine::prelude::*;
use lazy_static::*;

const SIZE: usize = 1024;
pub const WIDTH: usize = SIZE;
pub const HEIGHT: usize = (SIZE as f64 * (2.64 / 3.0)) as usize;
lazy_static! {
    static ref COVARAGE_GRID: CovarageGrid = CovarageGrid::get_covarag_grid(10_000, 30, 1_000, 30);
}

fn main() {
    let buddha = Buddha::new(100, ViewRect::default(), &COVARAGE_GRID);
    
    let test = CameraManger::new(false, buddha);
    SingleWindowApplication::run(test);
}
