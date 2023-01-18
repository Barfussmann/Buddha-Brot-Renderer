use super::camera::ViewRect;
use super::histogram::{FFTHistogram, Histogram};
use rand::prelude::*;

pub struct Blue {
    pixel_hist: Histogram,
    pixel_fft: FFTHistogram,
    value_hist: Histogram,
    points: Vec<u64>,
    value: Vec<f64>,
    size: usize,
    gausian_2d: Vec<f64>,
    gausian_1d: Vec<f64>,
    half_gausian_length: i32,
    sigma: f64,
    point_count: u64,
}

impl Blue {
    pub fn start(size: usize) {
        assert!(size.is_power_of_two());
        let scale = match size {
            1024 => minifb::Scale::X1,
            512 => minifb::Scale::X2,
            256 => minifb::Scale::X4,
            128 => minifb::Scale::X8,
            64 => minifb::Scale::X16,
            32 => minifb::Scale::X32,
            _ => panic!("Invalid size"),
        };
        Self {
            pixel_hist: Histogram::new(size, size, ViewRect::default(), scale),
            pixel_fft: FFTHistogram::new(size, scale),
            value_hist: Histogram::new(size, size, ViewRect::default(), scale),

            points: vec![0; size * size],
            value: vec![f64::EPSILON; size * size],
            size,
            gausian_2d: Vec::new(),
            gausian_1d: Vec::new(),
            half_gausian_length: 0,
            sigma: 0.0,
            point_count: 0,
        }
        .run();
    }
    fn run(&mut self) {
        let mut rng = thread_rng();
        self.set_sigma(2.0);

        let samples = 100;
        for _ in 0..samples {
            let index = rng.gen_range(0..self.size * self.size);
            if self.points[index] == 0 {
                self.add_point(index);
            }
        }
        'outer: loop {
            self.draw();
            for _ in 0..10 {
                if !self.swab_max_min() {
                    break 'outer;
                }
            }
            // std::thread::sleep(std::time::Duration::from_millis(300));
        }
        self.point_count = samples;
        loop {
            for _ in 0..500 {
                if self.point_count < (self.size * self.size) as u64 {
                    self.add_min_point();
                } else {
                    break;
                }
            }
            self.draw();
        }
    }
    fn draw(&mut self) {
        for (point, fft_value) in std::iter::zip(&self.points, &mut self.pixel_fft.pixels) {
            *fft_value = *point as f64;
        }
        self.pixel_hist.draw_linear_normalise();
        self.update_value_hist();
        self.value_hist.draw_squared_normalise();

        self.pixel_fft.draw();
    }
    fn remove_max_point(&mut self) -> usize {
        let max_index = self.get_biggest();
        self.remove_point(max_index);
        max_index
    }
    fn add_min_point(&mut self) -> usize {
        let min_index = self.get_smallest();
        self.add_point(min_index);
        min_index
    }
    fn swab_max_min(&mut self) -> bool {
        let max_index = self.remove_max_point();
        let min_index = self.add_min_point();
        min_index != max_index
    }
    fn get_biggest(&mut self) -> usize {
        assert!(self.point_count > 0);
        self.value
            .iter()
            .enumerate()
            .filter(|(index, _)| self.points[*index] != 0)
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0
    }
    fn get_smallest(&self) -> usize {
        self.value
            .iter()
            .enumerate()
            .filter(|(index, _)| self.points[*index] == 0)
            .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0
    }
    fn decrease_sigma(&mut self) {
        for (pixel, value) in std::iter::zip(&self.points, &mut self.value) {
            if *pixel != 0 {
                *value = 1.;
            } else {
                *value = 0.;
            }
        }
        self.set_sigma(self.sigma * 0.9);
        self.whole_screen_gaus();
    }
    fn remove_point(&mut self, index: usize) {
        assert!(self.points[index] != 0);
        self.point_count -= 1;
        self.points[index] = 0;
        self.pixel_hist.set_index(index, 0);
        self.add_pixel_gaus(index, -1.0);
    }
    fn add_point(&mut self, index: usize) {
        assert!(self.points[index] == 0);
        self.point_count += 1;
        self.points[index] = self.point_count;
        self.pixel_hist.set_index(index, self.point_count as u32);
        self.add_pixel_gaus(index, 1.);
    }
    fn wraping_add(&mut self, point: (i32, i32), value: f64) {
        let index = self.wrap_point(point);
        self.value[index] += value;
    }
    fn set_sigma(&mut self, sigma: f64) {
        fn calc_gaus(r_squared: i32, sigma: f64) -> f64 {
            // (-r_squared as f64 / (2. * sigma * sigma)).exp()
            //     / (2. * std::f64::consts::PI * sigma * sigma)
            (-(r_squared as f64).sqrt() / sigma).exp()
        }
        self.sigma = sigma;

        self.gausian_2d.clear();
        self.gausian_1d.clear();

        for x in 1.. {
            if calc_gaus(x * x, sigma) < f64::EPSILON {
                self.half_gausian_length = x;
                break;
            }
        }
        for x in -self.half_gausian_length..=self.half_gausian_length {
            self.gausian_1d.push(calc_gaus(x * x, sigma));
        }
        // for y in self.gausian_1d.iter() {
        //     for x in self.gausian_1d.iter() {
        //         self.gausian_2d.push(x * y);
        //     }
        // }
        for y in -self.half_gausian_length..=self.half_gausian_length {
            for x in -self.half_gausian_length..=self.half_gausian_length {
                self.gausian_2d.push(calc_gaus(x * x + y * y, sigma));
            }
        }
    }
    fn whole_screen_gaus(&mut self) {
        const fn wrap(x: i32, size: usize) -> i32 {
            if x < 0 {
                x + size as i32
            } else if x >= size as i32 {
                x - size as i32
            } else {
                x
            }
        }
        for (value, pixel) in std::iter::zip(&mut self.value, &self.points) {
            if *pixel != 0 {
                *value = 1.;
            } else {
                *value = 0.;
            }
        }

        let value = self.value.clone();

        for (y, window) in value.chunks_exact(self.size).enumerate() {
            for x in 0..self.size as i32 {
                let mut sum = 0.;
                for x_offset in 0..self.half_gausian_length * 2 + 1 {
                    let x_index = wrap(x + x_offset - self.half_gausian_length, self.size);
                    sum += window[x_index as usize] * self.gausian_1d[x_offset as usize];
                }
                self.value[y * self.size + x as usize] = sum;
            }
        }
        let value = self.value.clone();
        for x in 0..self.size {
            for y in 0..self.size as i32 {
                let mut sum = 0.;
                for y_offset in 0..self.half_gausian_length * 2 + 1 {
                    let y_index = wrap(y + y_offset - self.half_gausian_length, self.size);
                    sum += value[y_index as usize * self.size + x]
                        * self.gausian_1d[y_offset as usize];
                }
                self.value[y as usize * self.size + x] = sum;
            }
        }
    }
    fn add_pixel_gaus(&mut self, index: usize, mul: f64) {
        let x_offset = (index % self.size) as i32 - self.half_gausian_length as i32;
        let y_offset = (index / self.size) as i32 - self.half_gausian_length as i32;
        for y in 0..self.half_gausian_length * 2 + 1 {
            let y_value = y as i32 + y_offset;
            for x in 0..self.half_gausian_length * 2 + 1 {
                let x_value = x as i32 + x_offset;
                let gaus_index = x + y * (self.half_gausian_length * 2 + 1);
                self.wraping_add(
                    (x_value, y_value),
                    self.gausian_2d[gaus_index as usize] * mul,
                );
            }
        }
    }
    fn update_value_hist(&mut self) {
        let offset = self
            .value
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        for i in 0..self.size * self.size {
            self.value_hist
                .set_index(i, ((self.value[i] - offset) * 10_000_000.) as u32);
        }
    }
    const fn wrap_point(&self, (mut x, mut y): (i32, i32)) -> usize {
        if x < 0 {
            x += self.size as i32;
        } else if x >= self.size as i32 {
            x -= self.size as i32;
        }
        if y < 0 {
            y += self.size as i32;
        } else if y >= self.size as i32 {
            y -= self.size as i32;
        }

        (x as usize) + (y as usize) * self.size
    }
}
