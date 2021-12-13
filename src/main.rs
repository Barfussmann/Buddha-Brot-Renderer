#![allow(dead_code)]
#![warn(
    clippy::branches_sharing_code,
    clippy::cognitive_complexity,
    clippy::option_if_let_else,
    clippy::suspicious_operation_groupings,
    clippy::useless_let_if_seq
)]
#![feature(test, portable_simd, iter_zip, array_chunks)]

mod buddha;
mod camera;
mod covarage_grid;
mod mandel_brot_render;
mod mandel_iter;

use covarage_grid::CovarageGrid;
use mandel_brot_render::MandelbrotRender;
use speedy2d::{window, Graphics2D, Window};
use glam::dvec2;

const SIZE: usize = 1024;
const WIDTH: usize = SIZE;
const HEIGHT: usize = (SIZE as f64 * (2.64 / 3.0)) as usize;

struct WindowHandler {
    mandel_render: MandelbrotRender,
    camera: camera::CameraManger,
}

impl window::WindowHandler for WindowHandler {
    fn on_key_down(
        &mut self,
        helper: &mut window::WindowHelper<()>,
        virtual_key_code: Option<window::VirtualKeyCode>,
        scancode: window::KeyScancode,
    ) {
        self.camera.on_key_down(virtual_key_code);
    }
    fn on_mouse_button_down(
        &mut self,
        helper: &mut window::WindowHelper<()>,
        button: window::MouseButton,
    ) {
        self.camera.on_mouse_button_down(button);
    }
    fn on_mouse_button_up(
        &mut self,
        helper: &mut window::WindowHelper<()>,
        button: window::MouseButton,
    ) {
        self.camera.on_mouse_button_up(button);
    }
    fn on_mouse_move(
        &mut self,
        helper: &mut window::WindowHelper<()>,
        position: speedy2d::dimen::Vector2<f32>,
    ) {
        let pos = dvec2(position.x as f64, position.y as f64);
        self.camera
            .on_mouse_move(pos);
    }
    fn on_draw(&mut self, helper: &mut window::WindowHelper, graphics: &mut Graphics2D) {
        self.camera.update();
        self.mandel_render.draw(&self.camera, graphics);
        helper.request_redraw();
    }
}

fn main() {
    let handle = WindowHandler {
        mandel_render: MandelbrotRender::new(),
        camera: camera::CameraManger::new(),
    };
    let window = Window::new_centered("Mandelbrot", (WIDTH as u32, HEIGHT as u32)).unwrap();
    window.run_loop(handle);
}
