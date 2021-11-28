extern crate test;
use glam::Vec2;
use macroquad::prelude::{draw_rectangle, Color};

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
        assert!(start < end);
        Self { start, end }
    }
    pub fn from_index(index: usize) -> Self {
        Self {
            start: index,
            end: index + 1,
        }
    }
    pub fn is_inside(&self, index: usize) -> bool {
        self.start <= index && index < self.end
    }
    pub fn relation_to(&self, other: Self) -> Relation {
        if self.end < other.start {
            return Relation::Before;
        } else if self.start > other.end {
            return Relation::After;
        } else if self.end == other.start {
            return Relation::AdjacentBefore;
        } else if self.start == other.end {
            return Relation::AdjacentAfter;
        } else {
            return Relation::Overlapping;
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
    pub fn split_remove(&self, index: usize) -> (Self, Self) {
        debug_assert!(self.is_inside(index), "index {} is not inside range {:?}", index, self);
        debug_assert!(self.len() > 2, "Cannot split a range with less then 3 indices");
        debug_assert!(index != self.start && index != self.end - 1, "Cannot split at start or end");
        let left = Self::new(self.start, index);
        let right = Self::new(index + 1, self.end);
        (left, right)
    }
    pub fn intersect(&self, other: Self) -> Option<Self> {
        match self.relation_to(other) {
            Relation::Before => None,
            Relation::AdjacentBefore => None,
            Relation::Overlapping => {
                let new_start = self.start.max(other.start);
                let new_end = self.end.min(other.end);
                Some(Self::new(new_start, new_end))
            }
            Relation::AdjacentAfter => None,
            Relation::After => None,
        }
    }
    pub fn len(&self) -> usize {
        self.end - self.start
    }
    pub fn draw(&self, x: usize, color: Color, grid_size: usize) {
        let side_length = 4. / grid_size as f64;
        let index = Vec2::new(x as f32, self.start as f32);
        let center = (index - Vec2::splat((grid_size / 2) as f32)) * side_length as f32;
        let corner_pos = center - Vec2::splat(side_length as f32 / 2.0);
        let delta_y = (self.end - self.start) as f32 * side_length as f32;
        draw_rectangle(
            corner_pos.x,
            corner_pos.y,
            side_length as f32,
            delta_y,
            color,
        );
    }
    pub fn iter(&self) -> std::ops::Range<usize> {
        self.start..self.end
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic]
    fn smaller_end_then_start_panics() {
        Range::new(10, 5);
    }
    #[test]
    fn start_is_in_range() {
        let range = Range::new(5, 10);
        assert!(range.is_inside(5));
    }
    #[test]
    fn end_is_not_range() {
        let range = Range::new(5, 10);
        assert!(!range.is_inside(10));
    }
    #[test]
    fn all_between_is_inside() {
        let range = Range::new(5, 10);
        for i in 5..10 {
            assert!(range.is_inside(i));
        }
    }
    #[test]
    fn from_index_is_inside() {
        let range = Range::from_index(5);
        assert!(range.is_inside(5));
    }
    #[test]
    fn from_index_neighbor_not_inside() {
        let range = Range::from_index(5);
        assert!(!range.is_inside(4));
        assert!(!range.is_inside(6));
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
        let range1 = Range::new(5, 10);
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
        let range1 = Range::new(5, 10);
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
    fn intersect_non_overlapping_empty() {
        let range1 = Range::new(5, 10);
        let range2 = Range::new(15, 20);
        assert_eq!(range1.intersect(range2), None);
        assert_eq!(range2.intersect(range1), None);
    }
    #[test]
    fn intersect_adjacent_before_or_after() {
        let range1 = Range::new(5, 10);
        let range2 = Range::new(10, 20);
        assert_eq!(range1.intersect(range2), None);
        assert_eq!(range2.intersect(range1), None);
    }
    #[test]
    fn intersect_overlapping_half() {
        let range1 = Range::new(5, 10);
        let range2 = Range::new(7, 20);
        assert_eq!(range1.intersect(range2), Some(Range::new(7, 10)));
        assert_eq!(range2.intersect(range1), Some(Range::new(7, 10)));
    }
    #[test]
    fn intersect_overlapping_one_inside_other() {
        let range1 = Range::new(5, 15);
        let range2 = Range::new(7, 12);
        assert_eq!(range1.intersect(range2), Some(Range::new(7, 12)));
        assert_eq!(range2.intersect(range1), Some(Range::new(7, 12)));
    }
    #[test]
    fn len_of_range_is_end_minus_start() {
        let range = Range::new(5, 10);
        assert_eq!(range.len(), 5);
    }
    #[test]
    fn len_of_from_index_is_1() {
        let range = Range::from_index(5);
        assert_eq!(range.len(), 1);
    }
    #[test]
    fn all_iter_values_are_inside() {
        let range = Range::new(5, 10);
        for i in 5..10 {
            assert!(range.is_inside(i));
        }
    }
    #[test]
    fn iter_has_all_values_from_range() {
        let range = Range::new(5, 10);
        let mut iter = range.iter();
        for i in 5..10 {
            assert_eq!(iter.next(), Some(i));
        }
        assert_eq!(iter.next(), None);
    }
    #[test]
    #[should_panic]
    fn split_at_outside_panics() {
        let range = Range::new(5, 10);
        range.split_remove(4);
    }
    #[test]
    #[should_panic]
    fn split_at_start_panics() {
        let range = Range::new(5, 10);
        range.split_remove(5);
    }
    #[test]
    #[should_panic]
    fn split_at_end_panics() {
        let range = Range::new(5, 10);
        range.split_remove(9);
    }
    #[test]
    fn split_return_correct_ranges() {
        let range = Range::new(5, 10);
        let (left, right) = range.split_remove(7);
        assert_eq!(left, Range::new(5, 7));
        assert_eq!(right, Range::new(8, 10));
    }
}
