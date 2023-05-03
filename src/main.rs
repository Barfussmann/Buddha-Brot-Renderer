#![allow(dead_code, unused)]
#![warn(clippy::nursery)]
#![feature(test, portable_simd, array_chunks)]

mod camera;
mod histogram;
mod iterat_point;
mod mandel_brot_render;
mod mandel_iter;
mod pixels;

// use buddha::Buddha;
use camera::*;

use crate::mandel_brot_render::MandelbrotRender;

const SIZE: usize = 1024;
pub const WIDTH: usize = SIZE;
pub const HEIGHT: usize = (SIZE as f64 * (2.64 / 3.0)) as usize;
pub const MIN_ITER: usize = 50;
pub const MAX_ITER: usize = 100;

fn main() {
    let mandelbrot = MandelbrotRender::new();
    // let buddha = Buddha::new(100, ViewRect::default(), &COVARAGE_GRID);
    let test = CameraManger::start(false, mandelbrot);
    // blue::Blue::start(512);
}
