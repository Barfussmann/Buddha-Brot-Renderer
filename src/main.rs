#![allow(dead_code)]
#![feature(test)]
#![feature(portable_simd)]
#![feature(iter_zip)]
#![feature(array_chunks)]

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

use macroquad::prelude::{is_key_down, next_frame, Conf, KeyCode};
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

    let mut grid = grid_bound::CovarageGrid::new(4, 1_00, 10_000);

    mandel_brot_render.set_camera_rect(camera_manager.get_view_rect());

    let mut limit = 30;

    loop {
        if is_key_down(KeyCode::U) {
            limit -= 1;
            grid.rebuild_grid(limit);
            println!("limit: {}, area: {}", limit, grid.get_area());
        } else if is_key_down(KeyCode::I) {
            limit += 1;
            grid.rebuild_grid(limit);
            println!("limit: {}, area: {}", limit, grid.get_area());
        }

        camera_manager.update();

        if camera_manager.had_change() {
            mandel_brot_render.set_camera_rect(camera_manager.get_view_rect());
        }


        mandel_brot_render.draw();
        grid.draw(&camera_manager);
        grid.sample_neighbors();

        next_frame().await;
    }
}
