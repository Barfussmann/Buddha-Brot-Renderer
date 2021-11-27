extern crate test;
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
    pub fn insert(&mut self, index: usize) {
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
        let is_adjacent_after = range_after.relation_to(range) == Relation::AdjacentAfter || range_after.relation_to(range) == Relation::AdjacentBefore;
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
    pub fn activ_count(&self) -> usize {
        self.activ_ranges.iter().map(|range| range.len()).sum()
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
    pub fn intersect(&self, other: &Self) -> Self {
        let mut this_range_iter = self.activ_ranges.iter().peekable();
        let mut other_range_iter = other.activ_ranges.iter();

        let mut next_this_range = this_range_iter.next();
        let mut next_other_range = other_range_iter.next();

        let mut result = RangeEncoder::new();

        while next_this_range.is_some() & next_other_range.is_some() {
            let this = next_this_range.unwrap();
            let other = next_other_range.unwrap();
            match this.relation_to(*other) {
                Relation::Before | Relation::AdjacentBefore => {
                    next_this_range = this_range_iter.next();
                }
                Relation::Overlapping => {
                    result.activ_ranges.push(this.intersect(*other).unwrap());
                    if let Some(new_next_this_range) =
                        this_range_iter.next_if(|next_this| next_this.relation_to(*other) == Relation::Overlapping)
                    {
                        next_this_range = Some(new_next_this_range);
                    } else {
                        next_other_range = other_range_iter.next();
                    }
                }
                Relation::AdjacentAfter | Relation::After => {
                    next_other_range = other_range_iter.next();
                }
            }
        }

        result
    }
    pub fn draw(&self, x: usize, color: Color, grid_size: usize) {
        for range in self.activ_ranges.iter() {
            range.draw(x, color, grid_size);
        }
    }
    pub fn iter(& self) -> impl Iterator<Item = usize> + '_ {
        self.activ_ranges.iter().map(|range| range.iter()).flatten()
    }
}
#[cfg(test)]
mod tests {
    use super::RangeEncoder;
    #[test]
    fn new_is_empty() {
        assert!(RangeEncoder::new().is_empty());
    }
    #[test]
    fn after_insertion_isnt_empty() {
        let mut range_encoder = RangeEncoder::new();
        range_encoder.insert(5);
        assert!(!range_encoder.is_empty());
    }
    #[test]
    fn after_insertion_activ_cells_is_1() {
        let mut range_encoder = RangeEncoder::new();
        range_encoder.insert(5);
        assert_eq!(range_encoder.activ_count(), 1);
    }
    #[test]
    fn activ_cells_equal_to_inserted_cells() {
        let mut range_encoder = RangeEncoder::new();
        for i in 1..100 {
            range_encoder.insert(i);
            assert_eq!(range_encoder.activ_count(), i);
        }
    }
    #[test]
    fn activ_cells_equal_to_inserted_cells2() {
        let indecies = [0, 2, 3, 4, 45, 67, 89, 90, 91, 92, 93, 94, 95, 96, 99];
        let mut range_encoder = RangeEncoder::new();
        for index in indecies.iter() {
            range_encoder.insert(*index);
        }
        assert_eq!(range_encoder.activ_count(), indecies.len());
    }
    #[test]
    fn iter_is_equal_to_inserted_cells() {
        let mut range_encoder = RangeEncoder::new();
        let indecies = [0, 2, 3, 4, 45, 67, 89, 90, 91, 92, 93, 94, 95, 96, 99];

        for index in indecies.iter() {
            range_encoder.insert(*index);
        }
        assert_eq!(range_encoder.iter().collect::<Vec<_>>(), indecies.to_vec());
    }
    #[test]
    fn cell_is_activ_after_isertion() {
        let mut range_encoder = RangeEncoder::new();
        range_encoder.insert(5);
        range_encoder.is_activ(5);
    }
    #[test]
    fn all_cells_activ_afet_range_insertion() {
        let mut range_encoder = RangeEncoder::new();
        for i in 0..100 {
            range_encoder.insert(i);
        }
        for i in 0..100 {
            assert!(range_encoder.is_activ(i));
        }
    }
    #[test]
    fn after_2_consecutive_insert_only_1_activ_range() {
        let mut range_encoder = RangeEncoder::new();
        range_encoder.insert(5);
        range_encoder.insert(6);
        assert_eq!(range_encoder.activ_ranges.len(), 1)
    }
    #[test]
    fn itersection_of_two_empty_range_encoder_is_empty() {
        let range_encoder = RangeEncoder::new();
        let range_encoder2 = RangeEncoder::new();
        let intersection = range_encoder.intersect(&range_encoder2);
        assert!(intersection.is_empty());
    }
    #[test]
    fn itersection_of_empty_and_non_empty_range_encoder_is_empty() {
        let mut range_encoder = RangeEncoder::new();
        range_encoder.insert(5);
        let range_encoder2 = RangeEncoder::new();
        let intersection = range_encoder.intersect(&range_encoder2);
        assert!(intersection.is_empty());
    }
    #[test]
    fn itersection_of_equal_range_encoder_is_equal() {
        let mut range_encoder = RangeEncoder::new();
        range_encoder.insert(5);
        range_encoder.insert(6);
        range_encoder.insert(8);
        let intersection = range_encoder.intersect(&range_encoder);
        assert_eq!(range_encoder, intersection);
    }
    #[test]
    fn intersection_of_ranges_are_all_itersections() {
        let mut range_encoder1 = RangeEncoder::new();
        for index in [5, 6, 7, 12, 13, 22] {
            range_encoder1.insert(index);
        }
        let mut range_encoder2 = RangeEncoder::new();
        for index in [4, 5, 7, 8, 11, 12, 13, 22] {
            range_encoder2.insert(index);
        }
        let intersection = range_encoder1.intersect(&range_encoder2);
        assert_eq!(intersection, range_encoder2.intersect(&range_encoder1));
        for index in [5, 7, 12, 13, 22] {
            assert!(intersection.is_activ(index));
        }
    }
    #[test]
    fn activ_ranges_are_sorted() {
        let mut range_encoder = RangeEncoder::new();
        for index in [54, 61, 17, 12, 143, 22] {
            range_encoder.insert(index);
        }
        for ranges in range_encoder.activ_ranges.windows(2) {
            match ranges[0].relation_to(ranges[1]) {
                crate::range::Relation::Before => {}
                crate::range::Relation::AdjacentBefore => {}
                crate::range::Relation::Overlapping => panic!("can't overlapp"),
                crate::range::Relation::AdjacentAfter => panic!("can't be after"),
                crate::range::Relation::After => panic!("can't be after"),
            }
        }
    }
    #[test]
    fn is_activ_of_non_activ_cells_is_false() {
        let mut range_encoder = RangeEncoder::new();
        for index in [12, 17, 22, 54, 61, 143] {
            range_encoder.insert(index);
        }
        for index in [0, 1, 3, 40, 200, 1000] {
            assert!(!range_encoder.is_activ(index));
        }
    }
}
