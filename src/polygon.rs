use super::polygon_reducer::*;
use super::util::*;
// use glam::dvec2 as vec2;
use super::triangulation::Triangulation;
use glam::DVec2 as Vec2;
use macroquad::color::*;

struct AreaOfRemovedTriangle {}
impl RemoveOrder for AreaOfRemovedTriangle {
    fn get_value(prev: Vec2, this: Vec2, next: Vec2) -> Option<f64> {
        Some(triangle_area(prev, this, next))
    }
}
struct AreaOfTriangleWhenConvex {}
impl RemoveOrder for AreaOfTriangleWhenConvex {
    fn get_value(prev: Vec2, this: Vec2, next: Vec2) -> Option<f64> {
        let delta_1 = this - prev;
        let delta_2 = next - this;
        let angle = delta_1.angle_between(delta_2);
        if angle < 0. {
            None
        } else {
            Some(triangle_area(prev, this, next))
        }
    }
}

pub struct Polygon {
    points: Vec<Vec2>,
}

impl Polygon {
    pub fn new(points: Vec<Vec2>) -> Polygon {
        Polygon { points }
    }
    pub fn get_points(&self) -> &Vec<Vec2> {
        &self.points
    }
    pub fn len(&self) -> usize {
        self.points.len()
    }
    pub fn draw(&self, line_width: f32) {
        if self.len() < 3 {
            return;
        }
        for [p1, p2] in self.points.array_windows::<2>() {
            draw_line(p1, p2, GREEN, line_width)
        }
        draw_line(
            self.points.first().unwrap(),
            self.points.last().unwrap(),
            GREEN,
            line_width,
        );
    }
    // https://en.wikipedia.org/wiki/Polygon#Area
    pub fn area(&self) -> f64 {
        let mut area = 0.;
        let mut previous_point = self.points.last().unwrap();
        for this_point in self.points.iter() {
            area += previous_point.x * this_point.y - this_point.x * previous_point.y;
            previous_point = this_point;
        }
        (area / 2.) as f64
    }
    // https://en.wikipedia.org/wiki/Centroid#Of_a_polygon
    pub fn centroid(&self) -> Vec2 {
        let mut center = Vec2::ZERO;
        let mut previous_point = *self.points.last().unwrap();
        for &this_point in self.points.iter() {
            let sum = previous_point + this_point;
            center += sum
                * Vec2::splat(
                        previous_point.x * this_point.y - this_point.x * previous_point.y,
                    );
            previous_point = this_point;
        }
        center / (6. * self.area())
    }
    pub fn triangulate(&self) -> Triangulation {
        let mut reducer = PolygonReducer::<AreaOfTriangleWhenConvex>::new(&self);

        reducer.reduce_till_empty();

        reducer.get_triangles()
    }
    pub fn simplify(&self, max_area_change_factor: f64) -> Polygon {
        let mut reducer = PolygonReducer::<AreaOfRemovedTriangle>::new(&self);
        let max_area_change = self.area() * max_area_change_factor;
        let mut total_area_change = 0.;

        while total_area_change < max_area_change {
            total_area_change += reducer.remove_next_point().unwrap();
        }
        reducer.get_polygon()
    }
}
