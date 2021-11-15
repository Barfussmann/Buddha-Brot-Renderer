use super::polygon::*;
use super::util::*;
use glam::dvec2 as vec2;
use glam::DVec2 as Vec2;
use macroquad::color::*;

enum GenSample {
    ToShort,
    UnderMinAngle,
    OverMaxAngle,
    Sample(Vec2),
}

pub struct Boundary {
    pub samples: Vec<Vec2>,
    limit: usize,
    angle_threshold: f64,
}

impl Boundary {
    pub fn new(iteration_limit: usize, angle_threshold: f64) -> Boundary {
        let y_offset = 0.02 / (0.3 * iteration_limit as f64).exp();
        Boundary {
            samples: vec![
                get_boundary_point(Vec2::ZERO, vec2(2., 0.), iteration_limit),
                get_boundary_point(vec2(0., y_offset), vec2(2.000, y_offset), iteration_limit),
            ],
            limit: iteration_limit,
            angle_threshold: angle_threshold.to_radians(),
        }
    }

    fn subdived_last_line(&mut self) {
        let start = *self.samples.iter().nth_back(1).unwrap();
        let direction = self.get_direction_of_last_line() / 2.;

        let sample = match self.gen_sample_at_angle(start, direction, 0., self.angle_threshold * 3.)
        {
            GenSample::Sample(sample) => sample,
            GenSample::UnderMinAngle => unreachable!(),
            GenSample::ToShort | GenSample::OverMaxAngle => {
                // retrys the subdive because of numerical instabilities
                // draw_line(&start, &(start + direction), RED, 0.002);
                self.samples.pop();
                self.subdived_last_line();
                return;
            }
        };

        self.samples.insert(self.samples.len() - 1, sample);
    }

    fn gen_sample_at_angle(
        &self,
        start: Vec2,
        direction: Vec2,
        min_angle: f64,
        max_angle: f64,
    ) -> GenSample {
        fn rotate_clockwise(vec: Vec2, angle: f64) -> Vec2 {
            vec2(
                vec.x * angle.cos() - vec.y * angle.sin(),
                vec.x * angle.sin() + vec.y * angle.cos(),
            )
        }

        if direction.length() < f64::EPSILON * 1000. {
            return GenSample::ToShort;
        }

        let upper_left_bound = start + rotate_clockwise(direction, -max_angle);
        let upper_right_bound = start + rotate_clockwise(direction, max_angle);

        let is_upper_left_inside = is_inside(&upper_left_bound, self.limit);
        let is_upper_right_inside = is_inside(&upper_right_bound, self.limit);

        if is_upper_left_inside == is_upper_right_inside {
            return GenSample::OverMaxAngle;
        }

        let lower_left_bound = start + rotate_clockwise(direction, -min_angle);
        let lower_right_bound = start + rotate_clockwise(direction, min_angle);

        let is_lower_left_inside = is_inside(&lower_left_bound, self.limit);
        let is_lower_right_inside = is_inside(&lower_right_bound, self.limit);

        if is_lower_left_inside != is_lower_right_inside {
            return GenSample::UnderMinAngle;
        }

        if is_lower_left_inside != is_upper_left_inside {
            return GenSample::Sample(get_boundary_point(
                lower_left_bound,
                upper_left_bound,
                self.limit,
            ));
        } else {
            return GenSample::Sample(get_boundary_point(
                lower_right_bound,
                upper_right_bound,
                self.limit,
            ));
        }
    }

    fn get_direction_of_last_line(&self) -> Vec2 {
        *self.samples.last().unwrap() - *self.samples.iter().nth_back(1).unwrap()
    }

    pub fn gen_next_sample(&mut self) -> bool {
        let start = *self.samples.last().unwrap();
        let mut direction = self.get_direction_of_last_line();

        if start.y <= 0. {
            if direction.length() > 0.00001 {
                let len = self.samples.len();
                self.subdived_last_line();
                if len < self.samples.len() {
                    self.samples.pop();
                }
                return true;
            } else {
                return false;
            }
        }

        for _ in 0..100 {
            match self.gen_sample_at_angle(
                start,
                direction,
                self.angle_threshold,
                self.angle_threshold * 2.,
            ) {
                GenSample::ToShort => return false,
                GenSample::UnderMinAngle => direction *= 1.1,
                GenSample::OverMaxAngle => direction /= 1.1,
                GenSample::Sample(sample) => {
                    self.samples.push(sample);
                    return true;
                }
            }
        }
        // only reachable when the previouse loop takes ages to find new samples.
        // It is ether in a loop or can't finde point with a small enough angle.
        // subdives last line to help
        self.subdived_last_line();
        return true;
    }

    pub fn gen_all(&mut self) {
        while self.samples.last().unwrap() != self.samples.iter().nth_back(1).unwrap()
            && self.gen_next_sample()
        {}
        self.remove_negativ_y_samples();
    }

    fn remove_negativ_y_samples(&mut self) {
        self.samples = self.samples.iter().filter(|a|a.y >= 0.).copied().collect();
    }
    pub fn gen_polygon(&self, max_area_change_factor: f64) -> Polygon {
        let points: Vec<Vec2> = self
            .samples
            .iter()
            .skip(1)
            .cloned()
            .chain(
                self.samples
                    .iter()
                    .rev()
                    .skip(1)
                    .map(|point_to_flip| vec2(point_to_flip.x, point_to_flip.y * -1.)),
            )
            .collect();

        let polygon = Polygon::new(points);
        if max_area_change_factor > 0. {
            polygon.simplify(max_area_change_factor)
        } else {
            polygon
        }
    }

    pub fn draw(&self, line_width: f32) {
        for [p1, p2] in self.samples.array_windows::<2>() {
            draw_line(p1, p2, GREEN, line_width)
        }
    }
}
