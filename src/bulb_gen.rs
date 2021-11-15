use super::polygon::*;
use super::util::*;

pub struct BulbGen {
    centroid: Vec2,
    boundary: Polygon,
    cycle_length: usize,
}
impl BulbGen {
    pub fn new(point: Vec2, cycle_length: usize) -> BulbGen {
        assert!(is_cycle(point, cycle_length));
        BulbGen {
            centroid: point,
            boundary: Polygon::new(Vec::new()),
            cycle_length,
        }
    }
    fn gen_first_boundary(&mut self) {
        self.boundary = Polygon::new(vec![
            get_cycle_boundary_point(
                self.centroid,
                self.centroid + vec2(2., 0.),
                self.cycle_length,
            ),
            get_cycle_boundary_point(
                self.centroid,
                self.centroid + vec2(0., 2.),
                self.cycle_length,
            ),
            get_cycle_boundary_point(
                self.centroid,
                self.centroid + vec2(-2., 0.),
                self.cycle_length,
            ),
            get_cycle_boundary_point(
                self.centroid,
                self.centroid + vec2(0., -2.),
                self.cycle_length,
            ),
        ]);
        self.centroid = self.boundary.centroid();
    }
    pub fn draw(&self, line_width: f32) {
        self.boundary.draw(line_width);
    }
    pub fn double_points(&mut self) {
        let new_point_count = if self.boundary.len() == 0 {
            4
        } else {
            self.boundary.len() * 2
        };

        let mut new_boundary_points = Vec::with_capacity(new_point_count);

        let mut angle = 0.;
        let angle_per_point = (360. / new_point_count as f64).to_radians();

        for _ in 0..new_point_count {
            angle += angle_per_point;
            let direction = vec2(2., 0.).rotate(angle);
            let outside_point = self.centroid + direction;
            new_boundary_points.push(get_cycle_boundary_point(
                self.centroid,
                outside_point,
                self.cycle_length,
            ));
        }
        self.boundary = Polygon::new(new_boundary_points);
        self.centroid = self.boundary.centroid();
    }
}
