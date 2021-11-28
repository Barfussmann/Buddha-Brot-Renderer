use super::util::*;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Rect {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}
impl Rect {
    pub fn new(x: usize, y: usize, w: usize, h: usize) -> Self {
        assert!(w > 0);
        assert!(h > 0);
        Self { x, y, w, h }
    }
    pub fn area(&self) -> usize {
        self.w * self.h
    }
    pub fn draw(&self, grid_size: usize, color: Color) {
        let side_length = 4. / grid_size as f64;
        let pos = Vec2::new(self.y as f64, self.x as f64) * side_length;
        let offset = Vec2::splat(side_length * grid_size as f64 / 2.);
        let width = self.w as f64 * side_length;
        let heigth = self.h as f64 * side_length;
        draw_rect(pos - offset, width, heigth, color);
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn area_of_rect_is_width_times_height() {
        let rect = Rect::new(0, 0, 10, 10);
        assert_eq!(rect.area(), 100);
    }
    #[test]
    #[should_panic]
    fn rect_new_panics_if_width_is_zero() {
        Rect::new(0, 0, 0, 10);
    }
    #[test]
    #[should_panic]
    fn rect_new_panics_if_height_is_zero() {
        Rect::new(0, 0, 10, 0);
    }
}