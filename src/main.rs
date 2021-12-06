#![allow(dead_code)]
#![feature(test)]
#![feature(portable_simd)]
#![feature(iter_zip)]
#![feature(array_chunks)]

mod camera;
mod cell;
mod draw_manager;
mod grid;
mod grid_bound;
mod mandel_brot_render;
mod mandel_iter;
mod range;
mod range_encoder;
mod u_rect;
mod util;
mod worker;

use glam::dvec2 as vec2;
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
    let mut mandel_brot_render =
        mandel_brot_render::MandelbrotRender::new(WIDTH, HEIGHT, vec2(-2., -2.), vec2(2., 2.));

    let mut draw_manager = draw_manager::DrawManager::new();

    let mut grid = grid_bound::CovarageGrid::new(30, 100, 1_000_000);

    mandel_brot_render.set_camera_rect(camera_manager.get_view_rect());

    let start_time = Instant::now();
    loop {
        camera_manager.update();

        if camera_manager.had_change() {
            mandel_brot_render.set_camera_rect(camera_manager.get_view_rect());
        }

        draw_manager.update();

        mandel_brot_render.draw();
        grid.draw(&draw_manager, &camera_manager);
        grid.sample_neighbors();
        let run_time = Instant::now().duration_since(start_time).as_secs_f32();
        // println!(
        //     "Cells per Second: {}",
        //     grid.get_processed_cells() as f32 / run_time
        // );

        next_frame().await;
    }
}
