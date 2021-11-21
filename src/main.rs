#![allow(dead_code)]
#![feature(exclusive_range_pattern)]
#![allow(unused_variables)]
#![feature(array_windows)]
#![feature(test)]

mod camera;
mod grid;
mod grid_bound;
mod range;
mod range_encoder;
mod util;
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

    let mut grid = grid_bound::CovarageGrid::new(50);

    loop {
        if is_key_down(KeyCode::U) {
            grid.sample_neighbors(100, &mut thread_rng());
        }
        if is_key_down(KeyCode::I) {}

        camera_manager.update();

        // dbg!(grid.new_neighbor_len());
        grid.draw();
        if grid.new_neighbor_len() == 0 {
            grid.sample_neighbors(10, &mut thread_rng());
        } else {
            for _ in 0..10000 {
                grid.sample_new_neighbors(&mut thread_rng());
            }
        }
        println!("Area: {}", grid.area());

        next_frame().await;
    }
}
