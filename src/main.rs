#![allow(dead_code)]
#![feature(test)]
#![feature(portable_simd)]
#![feature(iter_zip)]

mod camera;
mod cell;
mod draw_manager;
mod grid;
mod grid_bound;
mod grid_reducer;
mod mandel_iter;
mod range;
mod range_encoder;
mod rect;
mod util;
// use crate::util::*;

use macroquad::prelude::{is_key_pressed, next_frame, Conf, KeyCode};

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
    let mut draw_manager = draw_manager::DrawManager::new();

    let mut grid = grid_bound::CovarageGrid::new(30, 100, 10_000);

    loop {
        camera_manager.update();
        draw_manager.update();
        if is_key_pressed(KeyCode::U) {
            let area = grid.area();
            let reduced_area = grid.real_covered_area();
            println!(
                "area: {}, reduced area: {}, verhältin: {}",
                area,
                reduced_area,
                reduced_area / area
            )
        }

        // let grid_reducer = grid_reducer::GridReducer::new(grid.all_visited_cells.clone());
        // grid_reducer.biggest_rect();
        grid.draw(&draw_manager);
        grid.sample();
        // println!(
        //     "area: {}, sample: {}",
        //     grid.area(),
        //     grid.current_sample_count
        // );
        next_frame().await;
    }
}
