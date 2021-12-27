use minifb::{Key, Window, WindowOptions};

use super::mandel_brot_render::MandelbrotRender;
use glam::{dvec2, DVec2};

use super::{HEIGHT, WIDTH};

pub struct CameraManger<T: Updateable> {
    view_rect: ViewRect,
    mouse_poss_at_click: Option<DVec2>,
    mouse_pos: DVec2,
    mandel_background: Option<MandelbrotRender>,
    window: Window,
    generator: T,
}

impl<T> CameraManger<T>
where
    T: Updateable + 'static,
{
    pub fn start(mandel_render: bool, generator: T) {
        Self {
            view_rect: ViewRect::default(),
            mouse_poss_at_click: None,
            mouse_pos: DVec2::ZERO,
            mandel_background: mandel_render.then(MandelbrotRender::new),
            window: Window::new("Mandelbrot", WIDTH, HEIGHT, WindowOptions::default()).unwrap(),
            generator,
        }
        .run();
    }
    fn run(&mut self) {
        while self.window.is_open() {
            self.generator.update();
            self.window
                .update_with_buffer(&self.generator.draw(self.view_rect), WIDTH, HEIGHT)
                .unwrap();
            // let view = self.get_view_rect();
            // if let Some(mandel) = &mut self.mandel_background {
            //     self.window
            //         .update_with_buffer(&mandel.get_raw_pixels(view), WIDTH, HEIGHT)
            //         .unwrap();
            // }
            self.process_input();
            if self.generator.is_finished() {
                self.generator.finish();
                break;
            }
        }
    }
    fn zoom(&mut self, zoom: f64) {
        let camera_offset =
            (self.get_mouse_pos() - self.view_rect.top_left_corner) - self.view_rect.view_size / 2.;
        let camara_rect_shrinkage = 0.5 * (1. - 1. / zoom);
        let center_offset = self.view_rect.view_size * camara_rect_shrinkage;
        self.view_rect.top_left_corner +=
            camera_offset * camara_rect_shrinkage * 2. + center_offset;

        self.view_rect.view_size /= zoom;

        self.request_redraw();
    }
    fn update_drag(&mut self) {
        if let Some(mouse_poss_at_middle_click) = self.mouse_poss_at_click {
            let delta = mouse_poss_at_middle_click - self.get_mouse_pos();
            self.view_rect.top_left_corner += delta;
            self.request_redraw();
        }
    }
    fn get_mouse_pos(&self) -> DVec2 {
        let mouse_offset =
            self.mouse_pos / dvec2(WIDTH as f64, HEIGHT as f64) * self.view_rect.view_size;
        self.view_rect.top_left_corner + mouse_offset
    }
    pub fn get_view_rect(&self) -> ViewRect {
        self.view_rect
    }
    pub fn reset_zoom(&mut self) {
        self.view_rect = ViewRect::default();
        self.request_redraw();
    }
    fn request_redraw(&mut self) {
        self.generator.update_view_rect(self.view_rect);
    }

    fn process_input(&mut self) {
        if self.window.is_key_down(Key::Space) {
            self.reset_zoom()
        }
        let last_mouse_pos = self.mouse_pos;
        let mouse_pos = self.window.get_mouse_pos(minifb::MouseMode::Pass).unwrap();
        let scaled_pos: DVec2 = dvec2(mouse_pos.0 as f64, mouse_pos.1 as f64);

        self.mouse_pos = scaled_pos;
        if last_mouse_pos != self.mouse_pos {
            self.update_drag();
        }

        if self.window.get_mouse_down(minifb::MouseButton::Left)
            || self.window.get_mouse_down(minifb::MouseButton::Middle)
            || self.window.get_mouse_down(minifb::MouseButton::Right)
        {
            if self.mouse_poss_at_click.is_none() {
                self.mouse_poss_at_click = Some(self.get_mouse_pos());
            }
        } else {
            self.mouse_poss_at_click = None;
        }

        if let Some((_, y)) = self.window.get_scroll_wheel() {
            self.zoom((y as f64 / 3.).exp2());
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ViewRect {
    pub top_left_corner: DVec2,
    pub view_size: DVec2,
}
impl ViewRect {
    pub fn get_screen_scale(&self) -> DVec2 {
        dvec2(WIDTH as f64, HEIGHT as f64) / self.view_size
    }
    pub fn get_bottom_right_corner(&self) -> DVec2 {
        self.top_left_corner + self.view_size
    }
    pub fn screen_index(&self, point: DVec2) -> usize {
        let screen = (point - self.top_left_corner) * self.get_screen_scale();
        screen.x as usize + (screen.y as usize * WIDTH)
    }
}
impl Default for ViewRect {
    fn default() -> Self {
        Self {
            top_left_corner: dvec2(-2.0, -1.32),
            view_size: dvec2(3.0, 2.64),
        }
    }
}

pub trait Updateable {
    fn update(&mut self) {}
    fn draw(&mut self, view: ViewRect) -> Vec<u32>;
    fn is_finished(&self) -> bool {
        false
    }
    fn finish(&mut self) {}
    fn update_view_rect(&mut self, _view_rect: ViewRect) {}
}
