use core::simd::*;

pub struct Pixels {
    pub pixels: Vec<u32>,
}
impl Pixels {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            pixels: vec![0; width * height],
        }
    }
    pub fn add_one(&mut self, index: usize) {
        self.pixels[index] = self.pixels[index].saturating_add(1);
    }
    pub fn quad_add_one(&mut self, index: i64x4) {
        let indexs = index.as_array();
        let added_pixels = [
            self.pixels[indexs[0] as usize].saturating_add(1),
            self.pixels[indexs[1] as usize].saturating_add(1),
            self.pixels[indexs[2] as usize].saturating_add(1),
            self.pixels[indexs[3] as usize].saturating_add(1),
        ];
        self.pixels[indexs[0] as usize] = added_pixels[0];
        self.pixels[indexs[1] as usize] = added_pixels[1];
        self.pixels[indexs[2] as usize] = added_pixels[2];
        self.pixels[indexs[3] as usize] = added_pixels[3];
    }
    pub fn set(&mut self, index: usize, value: u32) {
        self.pixels[index] = value;
    }
    pub fn squre_normalise(&mut self) -> Vec<u32> {
        self.pixels[0] = 0;
        let max = *self.pixels.iter().max().unwrap();
        let mut reduced_max = max;
        while self
            .pixels
            .iter()
            .filter(|&&pixel| pixel > reduced_max)
            .count()
            * 1_000
            < self.pixels.len()
            && reduced_max > 1
        {
            reduced_max /= 2;
        }
        let mul = reduced_max as f32 / (255.0_f32).powi(2);
        self.pixels
            .iter()
            .map(|pixel| {
                let color = (*pixel as f32 / mul).sqrt().clamp(0., 255.) as u32;
                color | color << 8 | color << 16
            })
            .collect()
    }
    pub fn linear_normalise(&mut self) -> Vec<u32> {
        self.pixels[0] = 0;
        let max = *self.pixels.iter().max().unwrap();
        let mul = max as f32 / 255.0_f32;
        self.pixels
            .iter()
            .map(|pixel| {
                let color = (*pixel as f32 / mul).clamp(0., 255.) as u32;
                color | color << 8 | color << 16
            })
            .collect()
    }
    pub fn clear(&mut self) {
        self.pixels.iter_mut().for_each(|pixel| *pixel = 0);
    }
}
