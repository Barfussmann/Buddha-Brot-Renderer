use super::pixels::Pixels;
use super::camera::ViewRect;
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
    pub fn new(width: usize, height: usize, view: ViewRect) -> Self {
        Self {
            pixels: Pixels::new(width, height),
            view,
            window: Window::new("Histogram", width, height, WindowOptions::default()).unwrap(),
            width,
            height,
        }
    }
    pub fn add_point(&mut self, point: DVec2) {
        // println!("{:?}", point);
        // println!("{:?}", self.view.screen_index(point));
        let index = self.view.screen_index(point);
        if index < self.pixels.pixels.len() {
            self.pixels.add_one(self.view.screen_index(point));
        }
    }
    pub fn draw(&mut self) {
        self.window.update_with_buffer(&self.pixels.noramalise(), self.width, self.height).unwrap();
    }
    pub fn clear(&mut self){
        self.pixels.clear();
    }
}