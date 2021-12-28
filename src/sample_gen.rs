// use Image;
use super::covarage_grid::cell::Cell;
use flume::Sender;
use glam::DVec2;
use rand::prelude::*;

pub struct SampleGen {
    cells: Vec<Cell>,
    grid_size: usize,
    new_samples: Sender<Vec<DVec2>>,
}

impl SampleGen {
    pub fn start(cells: Vec<Cell>, grid_size: usize, sample_sender: Sender<Vec<DVec2>>) {
        Self {
            cells,
            grid_size,
            new_samples: sample_sender,
        }
        .work();
    }
    fn work(&mut self) {
        let mut enumerated_image = image::open("./FreeBlueNoiseTextures/Data/256_256/HDR_L_0.png")
            .unwrap()
            .into_luma16()
            .into_raw()
            .iter()
            .copied()
            .enumerate()
            .collect::<Vec<_>>();
        enumerated_image.sort_unstable_by_key(|&(_, color)| color);
        let pixel_order = enumerated_image
            .into_iter()
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        let mut pixel_order_iter = pixel_order.iter().cycle();
        let mut cell_iter = self.cells.iter();

        let mut index = *pixel_order_iter.next().unwrap();
        loop {
            let mut samples = Vec::with_capacity(1024);
            for _ in 0..512 {
                if let Some(cell) = cell_iter.next() {
                    let sample = cell.gen_point_from_index_inside(index, 256, self.grid_size);
                    samples.push(sample);
                } else {
                    index = *pixel_order_iter.next().unwrap();
                    cell_iter = self.cells.iter();
                }
            }
            if self.new_samples.send(samples).is_err() {
                break;
            }
        }
    }
}
