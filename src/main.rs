#![allow(dead_code)]
#![feature(exclusive_range_pattern)]
#![allow(unused_variables)]
#![feature(array_windows)]
#![feature(test)]

mod camera;
mod grid_bound;
mod util;
// use crate::bulb_gen::*;
// use crate::util::*;
use rand::thread_rng;

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

    let mut grid = grid_bound::Grid::new(50);

    loop {
        camera_manager.update();

        grid.draw();
        dbg!(grid.neighbor_len());
        for _ in 0..1 {
            grid.update_neighbors(100, &mut thread_rng());
        }

        next_frame().await;
    }
}
