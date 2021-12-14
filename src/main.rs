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

use covarage_grid::CovarageGrid;
use speedy2d::Window;

const SIZE: usize = 1024;
pub const WIDTH: usize = SIZE;
pub const HEIGHT: usize = (SIZE as f64 * (2.64 / 3.0)) as usize;

fn main() {
    println!("test 1");
    let test = Window::new_centered("test", (100, 100)).unwrap();
    let _test = CovarageGrid::gen_sample_cells(10_000, 10, 4);
    println!("test 2");
}
