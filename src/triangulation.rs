use super::util::*;
use macroquad::color::RED;
use glam::DVec2 as Vec2;
use rand::prelude::{Rng, ThreadRng};

pub struct Triangle {
    a: Vec2,
    b: Vec2,
    c: Vec2,
}

impl Triangle {
    pub fn new(a: Vec2, b: Vec2, c: Vec2) -> Triangle {
        Triangle { a, b, c }
    }
    fn area(&self) -> f64 {
        triangle_area(self.a, self.b, self.c)
    }
    fn gen_point_inside(&self, rng: &mut ThreadRng) -> Vec2 {
        gen_point_in_triangle(self.a, self.b, self.c, rng)
    }
}

pub struct Triangulation {
    triangles: Vec<Triangle>,
    area: f64,
}
impl Triangulation {
    pub fn new(triangles: Vec<Triangle>) -> Triangulation {
        let mut trig = Triangulation {
            triangles,
            area: 0.,
        };
        trig.update_area();
        trig
    }
    fn update_area(&mut self) {
        self.area = self.triangles.iter().map(|a| a.area()).sum();
    }
    pub fn area(&self) -> f64 {
        self.area
    }
    pub fn gen_points_inside(&self, count: usize, rng: &mut ThreadRng) -> Vec<Vec2> {
        let mut points = Vec::with_capacity(count);

        let area_per_point = self.area / count as f64;
        let mut remaining_area: f64 = area_per_point * rng.gen::<f64>();

        'outer: for triangle in self.triangles.iter().cycle() {
            let triangle_area = triangle.area();
            remaining_area += triangle_area;
            let points_in_this_triangle = (remaining_area / area_per_point) as usize;
            remaining_area = remaining_area.rem_euclid(area_per_point);
            for _ in 0..points_in_this_triangle {
                points.push(triangle.gen_point_inside(rng));
                if points.len() == count {
                    break 'outer;
                }
            }
        }
        assert_eq!(count, points.len());
        points
    }
    pub fn draw(&self, line_width: f32) {
        for triangle in self.triangles.iter() {
            draw_line(&triangle.a, &triangle.b, RED, line_width);
            draw_line(&triangle.b, &triangle.c, RED, line_width);
            draw_line(&triangle.c, &triangle.a, RED, line_width);
        }
    }
}
