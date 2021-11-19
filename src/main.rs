#![allow(dead_code)]
#![feature(exclusive_range_pattern)]
#![allow(unused_variables)]
#![feature(array_windows)]
#![feature(test)]

mod camera;
mod grid_bound;
mod util;
mod range_encoder;
mod range;
// use crate::bulb_gen::*;
// use crate::util::*;
use rand::thread_rng;

use macroquad::prelude::{is_key_down, next_frame, Conf, KeyCode};

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

    let mut grid = grid_bound::Grid::new(25);

    loop {
        if is_key_down(KeyCode::U) {
            grid.sample_neighbors(100, &mut thread_rng());
        }
        if is_key_down(KeyCode::I) {}

        camera_manager.update();

        dbg!(grid.new_neighbor_len());
        if grid.new_neighbor_len() == 0 {
            grid.sample_neighbors(10, &mut thread_rng());
            grid.draw();
        } else {
            for _ in 0..100 {
                grid.sample_new_neighbors(&mut thread_rng());
                grid.draw();
            }
        }

        next_frame().await;
    }
}
