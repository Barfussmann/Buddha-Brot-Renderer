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
mod cell;
mod grid;
mod grid_bound;
mod mandel_brot_render;
mod mandel_iter;
mod range;
mod range_encoder;
mod save_cell;
mod util;
mod worker;

use macroquad::prelude::{next_frame, Conf};
use std::time::Instant;

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
    let mut camera_manager = camera::CameraManger::new();
    let mut mandel_brot_render = mandel_brot_render::MandelbrotRender::new(WIDTH, HEIGHT);

    let mut grid = grid_bound::CovarageGrid::new(10, 1_000, 10_000);

    mandel_brot_render.set_camera_rect(camera_manager.get_view_rect());

    let start = Instant::now();
    loop {
        camera_manager.update();

        if camera_manager.had_change() {
            mandel_brot_render.set_camera_rect(camera_manager.get_view_rect());
        }

        mandel_brot_render.draw();
        grid.draw(&camera_manager);
        grid.sample_neighbors();

        let processed_cells = grid.get_processed_cells_count();
        let duration = start.elapsed();
        println!(
            "Cells per second: {}",
            processed_cells as f64 / duration.as_secs_f64()
        );

        next_frame().await;
    }
}
