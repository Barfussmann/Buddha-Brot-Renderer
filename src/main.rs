#![allow(dead_code)]
#![warn(
    clippy::branches_sharing_code,
    clippy::cognitive_complexity,
    clippy::option_if_let_else,
    clippy::suspicious_operation_groupings,
    clippy::useless_let_if_seq
)]
#![feature(test, portable_simd, iter_zip, array_chunks)]

mod camera;
mod covarage_grid;
mod mandel_brot_render;
mod mandel_iter;

use covarage_grid::CovarageGrid;
use macroquad::prelude::{next_frame, Conf};

const SIZE: usize = 1024;
const WIDTH: usize = SIZE;
const HEIGHT: usize = (SIZE as f64 * (2.64 / 3.0)) as usize;

fn window_conf() -> Conf {
    Conf {
        window_title: "Mandelbrot".to_owned(),
        fullscreen: false,
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,

        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> std::io::Result<()> {
    let covarage_grid = CovarageGrid::get_covarag_grid(30_000, 10, 4, 30).await;
    let mut camera = camera::CameraManger::new();
    loop {
        camera.update();
        covarage_grid.draw(&camera);

        next_frame().await;
    }
}
