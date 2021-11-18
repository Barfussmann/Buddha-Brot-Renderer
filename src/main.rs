#![allow(dead_code)]
#![feature(exclusive_range_pattern)]
#![allow(unused_variables)]
#![feature(array_windows)]
#![feature(test)]

mod grid_bound;
mod util;
// use crate::bulb_gen::*;
// use crate::util::*;
use rand::thread_rng;

use macroquad::prelude::{
    is_key_down, is_key_pressed, is_mouse_button_pressed, mouse_position_local, next_frame,
    set_camera, Camera2D, Conf, KeyCode, MouseButton, Rect, Vec2,
};

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
    let mut camera_rect = Rect::new(-2.01, -1.26, 3.02, 2.52);
    set_camera(&Camera2D::from_display_rect(camera_rect));

    let mut grid = grid_bound::Grid::new(50);

    // let mut boundarys = Vec::new();
    // for i in 13..=13 {
    //     boundarys.push(iterative_bound_gen::Boundary::new(i, 1.));
    // }
    // for boundary in boundarys.iter_mut() {
    //     boundary.gen_all();
    // }
    // let polygons: Vec<polygon::Polygon> = boundarys
    //     .iter()
    //     .map(|a| a.gen_polygon(0.00000001))
    //     .collect();

    // let triangulations: Vec<triangulation::Triangulation> =
    //     polygons.iter().map(|a| a.triangulate()).collect();

    let starting_line_width = 0.002;
    let zoom_factor = 2.;
    let inverse_zoom_factor = 1. - 1. / zoom_factor;
    let mut zoom = 1.;
    let mut line_width = starting_line_width / zoom;
    let mut threshold = 0.0001;

    // let mut bulb = BulbGen::new(vec2(0.0, 0.0), 0);
    // for _ in 0..7 {
    //     bulb.double_points();
    // }

    loop {
        {
            if is_mouse_button_pressed(MouseButton::Left) {
                let mouse_pos = mouse_position_local();
                let positiv_mouse_pos = 0.5 * (mouse_pos + Vec2::new(1., 1.));
                let new_x =
                    camera_rect.x + camera_rect.w * inverse_zoom_factor * positiv_mouse_pos.x;
                let new_y =
                    camera_rect.y + camera_rect.h * inverse_zoom_factor * positiv_mouse_pos.y;
                let new_h = inverse_zoom_factor * camera_rect.h;
                let new_w = inverse_zoom_factor * camera_rect.w;
                camera_rect = Rect::new(new_x, new_y, new_w, new_h);
                zoom *= zoom_factor;
                line_width = starting_line_width / zoom;
                set_camera(&Camera2D::from_display_rect(camera_rect));
            }
            if is_key_pressed(KeyCode::Space) {
                camera_rect = Rect::new(-2.01, -1.26, 3.02, 2.52);
                zoom = 1.;
                line_width = starting_line_width / zoom;
                set_camera(&Camera2D::from_display_rect(camera_rect));
            }
            if is_key_down(KeyCode::U) {
                threshold *= 1.01;
            }
            if is_key_down(KeyCode::I) {
                threshold *= 0.99;
            }
        }

        grid.draw();
        dbg!(grid.neighbor_len());
        for _ in 0..1 {
            grid.update_neighbors(100, &mut thread_rng());
        }

        next_frame().await;
    }
}
