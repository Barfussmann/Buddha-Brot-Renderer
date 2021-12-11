extern crate test;
use super::camera::*;
use glam::{dvec2, DVec2};
use macroquad::prelude::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Relation {
    Before,
    AdjacentBefore,
    Overlapping,
    AdjacentAfter,
    After,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}
impl Range {
    pub fn new(start: usize, end: usize) -> Self {
        assert!(start <= end);
        Self { start, end }
    }
    pub fn from_index(index: usize) -> Self {
        Self {
            start: index,
            end: index,
        }
    }
    pub fn relation_to(&self, other: Self) -> Relation {
        if self.end + 1 < other.start {
            Relation::Before
        } else if self.start > other.end + 1 {
            Relation::After
        } else if self.end + 1 == other.start {
            Relation::AdjacentBefore
        } else if self.start == other.end + 1 {
            Relation::AdjacentAfter
        } else {
            Relation::Overlapping
        }
    }
    pub fn merge_with(&self, other: Self) -> Option<Self> {
        match self.relation_to(other) {
            Relation::Before => return None,
            Relation::AdjacentBefore => {}
            Relation::Overlapping => {}
            Relation::AdjacentAfter => {}
            Relation::After => return None,
        }
        let new_start = self.start.min(other.start);
        let new_end = self.end.max(other.end);
        Some(Self::new(new_start, new_end))
    }
    pub fn len(&self) -> usize {
        self.end - self.start + 1
    }
    pub fn draw(&self, x: usize, color: Color, grid_size: usize, camera: &CameraManger) {
        let side_length = 4. / grid_size as f64;
        let index = DVec2::new((x - 1) as f64, self.start as f64) - DVec2::ONE;
        let mut corner = (index - DVec2::splat((grid_size / 2) as f64)) * side_length;
        let delta_y = self.len() as f64 * side_length;
        camera.draw_rect(corner, dvec2(side_length, delta_y), color);
        corner.y *= -1.;
        camera.draw_rect(corner, dvec2(side_length, -delta_y), color);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic]
    fn smaller_end_then_start_panics() {
        Range::new(10, 9);
    }
    #[test]
    fn relation_before_or_after() {
        let range1 = Range::new(5, 10);
        let range2 = Range::new(15, 20);
        assert_eq!(range1.relation_to(range2), Relation::Before);
        assert_eq!(range2.relation_to(range1), Relation::After);
    }
    #[test]
    fn relation_adjacent_before_or_after() {
        let range1 = Range::new(5, 9);
        let range2 = Range::new(10, 20);
        assert_eq!(range1.relation_to(range2), Relation::AdjacentBefore);
        assert_eq!(range2.relation_to(range1), Relation::AdjacentAfter);
    }
    #[test]
    fn relation_overlapping_half() {
        let range1 = Range::new(5, 10);
        let range2 = Range::new(7, 20);
        assert_eq!(range1.relation_to(range2), Relation::Overlapping);
        assert_eq!(range2.relation_to(range1), Relation::Overlapping);
    }
    #[test]
    fn relation_overlapping_one_inside_other() {
        let range1 = Range::new(5, 15);
        let range2 = Range::new(7, 12);
        assert_eq!(range1.relation_to(range2), Relation::Overlapping);
        assert_eq!(range2.relation_to(range1), Relation::Overlapping);
    }
    #[test]
    fn merge_adjacent_before_or_after() {
        let range1 = Range::new(5, 9);
        let range2 = Range::new(10, 20);
        assert_eq!(range1.merge_with(range2), Some(Range::new(5, 20)));
        assert_eq!(range2.merge_with(range1), Some(Range::new(5, 20)));
    }
    #[test]
    fn merge_not_adjacent_before_or_after() {
        let range1 = Range::new(5, 10);
        let range2 = Range::new(15, 20);
        assert_eq!(range1.merge_with(range2), None);
        assert_eq!(range2.merge_with(range1), None);
    }
    #[test]
    fn merge_overlapping_half() {
        let range1 = Range::new(5, 10);
        let range2 = Range::new(7, 20);
        assert_eq!(range1.merge_with(range2), Some(Range::new(5, 20)));
        assert_eq!(range2.merge_with(range1), Some(Range::new(5, 20)));
    }
    #[test]
    fn merge_overlapping_one_inside_other() {
        let range1 = Range::new(5, 15);
        let range2 = Range::new(7, 12);
        assert_eq!(range1.merge_with(range2), Some(Range::new(5, 15)));
        assert_eq!(range2.merge_with(range1), Some(Range::new(5, 15)));
    }
    #[test]
    fn len_of_range_is_end_minus_start_plus_one() {
        let range = Range::new(5, 10);
        assert_eq!(range.len(), 5 + 1);
    }
    #[test]
    fn len_of_from_index_is_1() {
        let range = Range::from_index(5);
        assert_eq!(range.len(), 1);
    }
    #[test]
    fn one_long_range_from_new() {
        let range = Range::new(5, 5);
        assert_eq!(range.len(), 1)
    }
}
