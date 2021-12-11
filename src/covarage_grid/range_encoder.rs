extern crate test;
use super::camera::*;
use macroquad::color::Color;

use super::range::*;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RangeEncoder {
    activ_ranges: Vec<Range>,
}
impl RangeEncoder {
    pub fn new() -> Self {
        Self {
            activ_ranges: Vec::new(),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.activ_ranges.is_empty()
    }
    pub fn insert_index(&mut self, index: usize) {
        let (is_activ, insert_index) = self.binary_search_for_index(index);
        if is_activ {
            return;
        }
        let range = Range::from_index(index);

        if self.activ_ranges.is_empty() {
            self.activ_ranges.push(range);
            return;
        }

        if insert_index == self.activ_ranges.len() {
            self.activ_ranges.push(range);
            return;
        }

        if insert_index == 0 {
            if let Some(new_range) = range.merge_with(self.activ_ranges[0]) {
                self.activ_ranges[0] = new_range;
            } else {
                self.activ_ranges.insert(0, range);
            }
            return;
        }

        let range_before = self.activ_ranges[insert_index - 1];
        let range_after = self.activ_ranges[insert_index];

        let is_adjacent_before = range_before.relation_to(range) == Relation::AdjacentBefore;
        let is_adjacent_after = range_after.relation_to(range) == Relation::AdjacentAfter
            || range_after.relation_to(range) == Relation::AdjacentBefore;
        match (is_adjacent_before, is_adjacent_after) {
            (true, true) => {
                let next_range = self.activ_ranges.remove(insert_index);
                let new_range = self.activ_ranges[insert_index - 1]
                    .merge_with(range)
                    .unwrap()
                    .merge_with(next_range)
                    .unwrap();
                self.activ_ranges[insert_index - 1] = new_range;
            }
            (true, false) => {
                let new_range = self.activ_ranges[insert_index - 1]
                    .merge_with(range)
                    .unwrap();
                self.activ_ranges[insert_index - 1] = new_range;
            }
            (false, true) => {
                let new_range = range.merge_with(self.activ_ranges[insert_index]).unwrap();
                self.activ_ranges[insert_index] = new_range;
            }
            (false, false) => {
                self.activ_ranges.insert(insert_index, range);
            }
        }
    }
    fn binary_search_for_index(&self, index: usize) -> (bool, usize) {
        let mut size = self.activ_ranges.len();
        let mut right = size;
        let mut left = 0;
        let range_of_index = Range::from_index(index);
        while left < right {
            let mid = left + size / 2;
            match self.activ_ranges[mid].relation_to(range_of_index) {
                Relation::Before => left = mid + 1,
                Relation::After => right = mid,
                Relation::Overlapping => return (true, mid),
                Relation::AdjacentBefore => return (false, mid),
                Relation::AdjacentAfter => return (false, mid),
            }
            size = right - left;
        }
        (false, left)
    }
    pub fn is_activ(&self, index: usize) -> bool {
        self.binary_search_for_index(index).0
    }
    pub fn draw(&self, x: usize, color: Color, grid_size: usize, camera: &CameraManger) {
        for range in self.activ_ranges.iter() {
            range.draw(x, color, grid_size, camera);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_is_empty() {
        assert!(RangeEncoder::new().is_empty());
    }
    #[test]
    fn after_insertion_isnt_empty() {
        let mut range_encoder = RangeEncoder::new();
        range_encoder.insert_index(5);
        assert!(!range_encoder.is_empty());
    }
    #[test]
    fn cell_is_activ_after_isertion() {
        let mut range_encoder = RangeEncoder::new();
        range_encoder.insert_index(5);
        range_encoder.is_activ(5);
    }
    #[test]
    fn all_cells_activ_afet_range_insertion() {
        let mut range_encoder = RangeEncoder::new();
        for i in 0..100 {
            range_encoder.insert_index(i);
        }
        for i in 0..100 {
            assert!(range_encoder.is_activ(i));
        }
    }
    #[test]
    fn after_2_consecutive_insert_only_1_activ_range() {
        let mut range_encoder = RangeEncoder::new();
        range_encoder.insert_index(5);
        range_encoder.insert_index(6);
        assert_eq!(range_encoder.activ_ranges.len(), 1)
    }
    #[test]
    fn activ_ranges_are_sorted() {
        let mut range_encoder = RangeEncoder::new();
        for index in [54, 61, 17, 12, 143, 22] {
            range_encoder.insert_index(index);
        }
        for ranges in range_encoder.activ_ranges.windows(2) {
            match ranges[0].relation_to(ranges[1]) {
                super::Relation::Before => {}
                super::Relation::AdjacentBefore => {}
                super::Relation::Overlapping => panic!("can't overlapp"),
                super::Relation::AdjacentAfter => panic!("can't be after"),
                super::Relation::After => panic!("can't be after"),
            }
        }
    }
    #[test]
    fn is_activ_of_non_activ_cells_is_false() {
        let mut range_encoder = RangeEncoder::new();
        for index in [12, 17, 22, 54, 61, 143] {
            range_encoder.insert_index(index);
        }
        for index in [0, 1, 3, 40, 200, 1000] {
            assert!(!range_encoder.is_activ(index));
        }
    }
}
