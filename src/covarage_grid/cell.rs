use glam::{dvec2, DVec2, IVec2};
use rand::{rngs::ThreadRng, Rng};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Cell {
    corner: IVec2,
}

impl Cell {
    pub const fn new(corner: IVec2) -> Self {
        Self { corner }
    }
    pub fn dummy() -> Self {
        Self {
            corner: IVec2::new(0, 0),
        }
    }
    pub fn get_corner(&self, grid_size: usize) -> DVec2 {
        self.corner.as_dvec2() * Self::side_length(grid_size)
    }
    pub fn gen_point_inside(&self, grid_size: usize, rng: &mut ThreadRng) -> DVec2 {
        let side_length = Self::side_length(grid_size);
        let corner = self.get_corner(grid_size);

        let offset = dvec2(
            rng.gen_range(0. ..side_length),
            rng.gen_range(0. ..side_length),
        );
        let mut point = corner + offset;
        if rng.gen() {
            point.y *= -1.0;
        }
        point
    }
    pub fn get_neighbors(&self) -> Vec<Self> {
        vec![
            Self::new(self.corner + IVec2::new(1, 0)),
            Self::new(self.corner + IVec2::new(0, 1)),
            Self::new(self.corner + IVec2::new(-1, 0)),
            Self::new(self.corner + IVec2::new(0, -1)),
        ]
    }
    pub fn from_index(index: usize, grid_size: usize) -> Self {
        let x = index % grid_size;
        let y = index / grid_size;
        Self::from_index_2d(x, y, grid_size)
    }
    pub fn from_index_2d(x: usize, y: usize, grid_size: usize) -> Self {
        let offset = IVec2::splat(grid_size as i32 / 2);
        let center = IVec2::new(x as i32 - 1, y as i32) - offset;
        Self { corner: center }
    }
    pub fn index(&self, grid_size: usize) -> usize {
        let (x, y) = self.index_2d(grid_size);
        (y * grid_size) + x
    }
    pub fn index_2d(&self, grid_size: usize) -> (usize, usize) {
        let index = self.corner + IVec2::splat((grid_size / 2) as i32);
        ((index.x + 1) as usize, index.y as usize)
    }
    pub fn is_y_negativ(&self) -> bool {
        self.corner.y < 0
    }
    pub fn side_length(grid_size: usize) -> f64 {
        4. / grid_size as f64
    }
    pub fn from_pos(pos: DVec2, grid_size: usize) -> Self {
        let mut ipos = (pos / Self::side_length(grid_size)).as_ivec2();
        if pos.x.is_sign_negative() {
            ipos.x -= 1;
        }
        Self::new(ipos)
    }
    pub fn gen_point_from_index_inside(&self, index: usize, index_grid_size: usize, global_grid_size: usize) -> DVec2 {
        let x = index % index_grid_size;
        let y = index / index_grid_size;
        let side_length = Self::side_length(global_grid_size);
        let step_size = side_length / index_grid_size as f64;
        let offset = dvec2(
            (x as f64) * step_size,
            (y as f64) * step_size,
        );
        let corner = self.get_corner(global_grid_size);
        corner + offset
    }
}

mod tests {
    #[allow(unused_imports)]
    use rand::thread_rng;
    #[test]
    fn from_index_form_index_is_same() {
        for grid_size in 100..1000 {
            let cell = super::Cell::new(super::IVec2::new(1, 2));
            let other = super::Cell::from_index(cell.index(grid_size), grid_size);
            assert_eq!(cell, other);
        }
    }
    #[test]
    fn from_pos_is_same_as_gen_point_pos() {
        let cell_1 = super::Cell::from_pos(super::DVec2::ZERO, 1000);
        let cell_2 = super::Cell::from_pos(super::dvec2(-1., 1.), 1000);
        let rng = &mut thread_rng();
        for _ in 0..10_000 {
            let mut from_pos_1 = super::Cell::from_pos(cell_1.gen_point_inside(1000, rng), 1000);
            from_pos_1.corner.y = from_pos_1.corner.y.abs();
            assert_eq!(cell_1, from_pos_1);
            let mut from_pos_2 = super::Cell::from_pos(cell_2.gen_point_inside(1000, rng), 1000);
            from_pos_2.corner.y = from_pos_2.corner.y.abs();
            assert_eq!(cell_2, from_pos_2);
        }
    }
}
