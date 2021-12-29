use flume::{Sender, Receiver};
use glam::DVec2;
use super::mandel_iter::iterat_points;

enum Work<'a> {
    Pixel((&'a [f64], f64, &'a mut [u32])),
}

struct Worker<'a> {
    work: Receiver<Work<'a>>,
    // finished_work
}

impl<'a> Worker<'a> {
    fn start(work: Receiver<Work<'a>>) {
        Self {
            work
        }.work();
    }
    fn work(&mut self) {
        for work in self.work.iter() {
            match work {
                Work::Pixel(_) => todo!(),
            }
        }

    }
    fn render_pixel((x, y, pixels): (&'a [f64], f64, &'a mut [u32])) {
        let y = [y; 4];
        for (x, pixel) in x.array_chunks::<4>().zip(pixels.array_chunks_mut::<4>()) {
            *pixel = iterations_to_color(iterat_points(*x, y, 256).to_array()); 
        }
    }
}

fn iterations_to_color(iterations: [i64; 4]) -> [u32; 4] {
    let mut colors = [0; 4];
    for i in 0..4 {
        let color_value = 255 - ((iterations[i] as f32).sqrt() * 15.) as u32;
        colors[i] = color_value + (color_value << 8) + (color_value << 16);
    }
    colors
}
