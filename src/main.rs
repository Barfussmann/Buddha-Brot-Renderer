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
// use crate::util::*;
use glam::dvec2 as vec2;

// use macroquad::prelude::*;
use macroquad::prelude::{draw_texture, next_frame, Color, Conf, Image, Texture2D, WHITE};

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

    let mut image = Image::gen_image_color(WIDTH as u16, HEIGHT as u16, WHITE);
    let texture = Texture2D::from_image(&image);

    let mut draw_manager = draw_manager::DrawManager::new();

    let mut grid = grid_bound::CovarageGrid::new(30, 10_000, 1_000);

    loop {
        camera_manager.update();

        if camera_manager.had_change() {
            mandel_brot_render.set_camera_rect(camera_manager.get_view_rect());
            image.update(mandel_brot_render.get_colors());
            texture.update(&image);
        }


        draw_manager.update();

        draw_texture(texture, 0., 0., Color::new(1., 1., 1., 1.));
        grid.draw(&draw_manager, &camera_manager);
        grid.sample_neighbors();

        // if is_key_pressed(KeyCode::U) {
        //     let area = grid.area();
        //     let reduced_area = grid.real_covered_area();
        //     println!(
        //         "area: {}, reduced area: {}, verhältin: {}",
        //         area,
        //         reduced_area,
        //         reduced_area / area
        //     )
        // }

        next_frame().await;
    }
}
