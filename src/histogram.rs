use super::camera::ViewRect;
use super::pixels::Pixels;
use glam::DVec2;
use minifb::{Window, WindowOptions};
use rustfft::algorithm::Radix4;
use rustfft::num_complex::Complex;
use rustfft::FftDirection;
use rustfft::Fft;

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

pub struct FFTHistogram {
    pub pixels: Vec<f64>,
    window: Window,
    size: usize,
    scratch: Vec<Complex<f64>>,
}
impl FFTHistogram {
    pub fn new(size: usize, scale: minifb::Scale) -> Self {
        let window_options = WindowOptions {
            scale,
            ..WindowOptions::default()
        };
        Self {
            pixels: vec![0.; size * size],
            window: Window::new("FFTHistogram", size, size, window_options).unwrap(),
            scratch: vec![Complex::new(0., 0.); size * size],
            size,
        }
    }
    pub fn draw(&mut self) {
        let fft = Radix4::new(self.pixels.len(), FftDirection::Forward);
        let mut complex_pixels = self.pixels.iter().map(|x| Complex::new(*x, 0.)).collect::<Vec<_>>();
        fft.process_with_scratch(&mut complex_pixels, &mut self.scratch);
        // fft.process()
        let values = complex_pixels.iter().map(|c| c.norm() ).collect::<Vec<f64>>();
        let mul = 255. / values
            .iter()
            .skip(1)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let mut pixels = values
            .iter()
            .map(|v| {
                let value = (v * mul) as u32;
                value << 16 | value << 8 | value
            })
            .collect::<Vec<u32>>();
        pixels.rotate_right(self.size / 2 + self.size * self.size / 2);
        
        self.window
            .update_with_buffer(&pixels, self.size, self.size)
            .unwrap();
    }
}
