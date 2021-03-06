use super::covarage_grid::cell::Cell;
use super::worker::{PointType, WorkerMessage};
use flume::Sender;
use std::sync::atomic;

const TEXTURE_SIZE: usize = 256;
static REQUESTED_SAMPLES: atomic::AtomicU64 = atomic::AtomicU64::new(0);
static RESET_INDEX: atomic::AtomicBool = atomic::AtomicBool::new(false);

pub struct SampleGen {
    cells: Vec<Cell>,
    grid_size: usize,
    new_samples: Sender<WorkerMessage>,
}

impl SampleGen {
    pub fn start(cells: Vec<Cell>, grid_size: usize, sample_sender: Sender<WorkerMessage>) {
        Self {
            cells,
            grid_size,
            new_samples: sample_sender,
        }
        .work();
    }
    fn work(&mut self) {
        let mut enumerated_image = image::open(format!(
            "./FreeBlueNoiseTextures/Data/{TEXTURE_SIZE}_{TEXTURE_SIZE}/HDR_L_0.png"
        ))
        .unwrap()
        .into_luma16()
        .into_raw()
        .iter()
        .copied()
        .enumerate()
        .collect::<Vec<_>>();
        enumerated_image.sort_unstable_by_key(|&(_, color)| color);
        let mut pixel_order = enumerated_image
            .into_iter()
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        pixel_order.reverse();

        let mut pixel_order_iter = pixel_order.iter().cycle();
        let mut cell_iter = self.cells.iter();

        let mut index = *pixel_order_iter.next().unwrap();
        loop {
            std::thread::sleep(std::time::Duration::from_millis(1));
            let requested_samples = REQUESTED_SAMPLES.swap(0, atomic::Ordering::Relaxed);
            for _ in 0..requested_samples {
                let mut samples = Vec::with_capacity(1024);
                for _ in 0..1024 {
                    if let Some(cell) = cell_iter.next() {
                        let sample =
                            cell.gen_point_from_index_inside(index, TEXTURE_SIZE, self.grid_size);
                        samples.push(sample);
                    } else {
                        if RESET_INDEX.swap(false, atomic::Ordering::Relaxed) {
                            pixel_order_iter = pixel_order.iter().cycle();
                        }
                        index = *pixel_order_iter.next().unwrap();
                        cell_iter = self.cells.iter();
                        println!("Next");
                    }
                }
                if self
                    .new_samples
                    .send(WorkerMessage::CheckPoints(PointType::New(samples)))
                    .is_err()
                {
                    break;
                }
            }
        }
    }
    pub fn request_sample() {
        REQUESTED_SAMPLES.fetch_add(1, atomic::Ordering::Relaxed);
    }
    pub fn reset_rnd() {
        RESET_INDEX.store(true, atomic::Ordering::Relaxed);
    }
}
