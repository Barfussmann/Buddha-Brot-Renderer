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
mod grid_reducer;
mod mandel_brot_render;
mod mandel_iter;
mod range;
mod range_encoder;
mod u_rect;
mod util;
// use crate::util::*;
use glam::dvec2 as vec2;

// use macroquad::prelude::*;
use macroquad::prelude::{
    draw_texture, is_key_pressed, next_frame, Conf, Image, KeyCode, Texture2D, WHITE,
};

fn window_conf() -> Conf {
    Conf {
        window_title: "Mandelbrot".to_owned(),
        fullscreen: false,
        window_width: 900,
        window_height: 900,

        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> std::io::Result<()> {
    let mut camera_manager = camera::CameraManger::new();
    let mut mandel_brot_render =
        mandel_brot_render::MandelbrotRender::new(900, 900, vec2(-2., -2.), vec2(2., 2.));

    let mut image = Image::gen_image_color(900, 900, WHITE);
    let texture = Texture2D::from_image(&image);

    let mut draw_manager = draw_manager::DrawManager::new();

    let mut grid = grid_bound::CovarageGrid::new(30, 100, 10_000);

    loop {
        camera_manager.update();
        mandel_brot_render.set_camera_rect(camera_manager.get_camera_rect());
        image.update(mandel_brot_render.get_colors());
        texture.update(&image);
        draw_texture(texture, 0., 0., WHITE);
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
        // grid.draw(&draw_manager);
        // grid.sample();
        // println!(
        //     "area: {}, sample: {}",
        //     grid.area(),
        //     grid.current_sample_count
        // );
        next_frame().await;
    }
}
