#![allow(dead_code)]
#![feature(exclusive_range_pattern)]
#![allow(unused_variables)]
#![feature(array_windows)]
#![feature(test)]
#![feature(portable_simd)]

mod camera;
mod grid;
mod grid_bound;
mod mandel_iter;
mod range;
mod range_encoder;
mod util;
// use crate::util::*;

use macroquad::prelude::{next_frame, Conf};

fn window_conf() -> Conf {
    Conf {
        window_title: "Mandelbrot".to_owned(),
        fullscreen: false,
        window_height: 750,
        window_width: 900,

        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> std::io::Result<()> {
    let mut camera_manager = camera::CameraManger::new();

    let mut grid = grid_bound::CovarageGrid::new(30, 2);

    loop {
        camera_manager.update();

        // dbg!(grid.new_neighbor_len());
        grid.sample();
        grid.draw();
        if grid.new_neighbors.len() == 0 {
            dbg!(grid.neighbors.len());
            dbg!(grid.total_sample_count);
        }
        next_frame().await;
    }
}
