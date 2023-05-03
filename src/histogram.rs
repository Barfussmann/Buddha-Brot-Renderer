use super::camera::ViewRect;
use super::pixels::Pixels;
use glam::DVec2;
use minifb::{Window, WindowOptions};

pub struct Histogram {
    pixels: Pixels,
    view: ViewRect,
    window: Window,
    width: usize,
    height: usize,
}
impl Histogram {
    pub fn new(width: usize, height: usize, view: ViewRect, scale: minifb::Scale) -> Self {
        let window_options = WindowOptions {
            scale,
            ..WindowOptions::default()
        };
        Self {
            pixels: Pixels::new(width, height),
            view,
            window: Window::new("Histogram", width, height, window_options).unwrap(),
            width,
            height,
        }
    }
    pub fn add_point(&mut self, point: DVec2) {
        let index = self.view.screen_index(point);
        if index < self.pixels.pixels.len() {
            self.pixels.add_one(self.view.screen_index(point));
        }
    }
    pub fn add_index(&mut self, index: usize) {
        self.pixels.add_one(index);
    }
    pub fn set_index(&mut self, index: usize, value: u32) {
        self.pixels.set(index, value);
    }
    pub fn draw_squared_normalise(&mut self) {
        self.window
            .update_with_buffer(&self.pixels.squre_normalise(), self.width, self.height)
            .unwrap();
    }
    pub fn draw_linear_normalise(&mut self) {
        self.window
            .update_with_buffer(&self.pixels.linear_normalise(), self.width, self.height)
            .unwrap();
    }
    pub fn clear(&mut self) {
        self.pixels.clear();
    }
}
