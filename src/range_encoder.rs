extern crate test;

use super::range::*;
#[derive(Debug, Clone, PartialEq, Eq)]
struct RangeEncoder {
    activ_ranges: Vec<Range>,
}
impl RangeEncoder {
    fn new() -> Self {
        Self {
            activ_ranges: Vec::new(),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.activ_ranges.is_empty()
    }
    fn try_to_append_index_existing_range(&mut self, index: usize) -> bool {
        let range = Range::from_index(index);
        for (index,range_to_append_to) in self.activ_ranges.iter_mut().enumerate() {
            if let Some(new_range) = range_to_append_to.merge(range) {
                *range_to_append_to = new_range;
                self.try_to_merge_activ_range_at_index_with_next(index);
                return true;
            }
        }
        return false;
    }
    fn try_to_merge_activ_range_at_index_with_next(&mut self, index: usize) {
        if index == self.activ_ranges.len() - 1 {
            return;
        }
        if let Some(new_range) = self.activ_ranges[index].merge(self.activ_ranges[index + 1]) {
            self.activ_ranges[index] = new_range;
            self.activ_ranges.remove(index + 1);
        }
    }
    pub fn insert(&mut self, index: usize) {
        if !self.try_to_append_index_existing_range(index) {
            self.activ_ranges.push(Range::from_index(index));
        }
    }
    pub fn activ_cells(&self) -> usize {
        self.activ_ranges.iter().map(|range| range.len()).sum()
    }
    pub fn is_activ(&self, index: usize) -> bool {
        self.activ_ranges.iter().any(|r| r.is_inside(index))
    }
    pub fn intersect(&self, other: &Self) -> Self {
        let mut result = Self::new();
        for range in self.activ_ranges.iter() {
            for other_range in other.activ_ranges.iter() {
                if let Some(new_range) = range.intersect(*other_range) {
                    result.activ_ranges.push(new_range);
                }
            }
        }
        result
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
        assert_eq!(range_encoder.activ_cells(), 1);
    }
    #[test]
    fn activ_cells_equal_to_inserted_cells() {
        let mut range_encoder = RangeEncoder::new();
        for i in 1..100 {
            range_encoder.insert(i);
            assert_eq!(range_encoder.activ_cells(), i);
        }
    }
    #[test]
    fn activ_cells_equal_to_inserted_cells2() {
        let indecies = [0, 2,3,4, 45, 67, 89, 90, 91, 92, 93, 94, 95, 96, 99];
        let mut range_encoder = RangeEncoder::new();
        for index in indecies.iter() {
            range_encoder.insert(*index);
        }
        assert_eq!(range_encoder.activ_cells(), indecies.len());
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
    fn after_two_consecutive_insert_only_1_activ_range() {
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
    fn itersection_of_empty_and_non_empty_range_encoder_is_empty() {
        let mut range_encoder = RangeEncoder::new();
        range_encoder.insert(5);
        let range_encoder2 = RangeEncoder::new();
        let intersection = range_encoder.intersect(&range_encoder2);
        assert!(intersection.is_empty());
    }
    fn itersection_of_equal_range_encoder_is_equal() {
        let mut range_encoder = RangeEncoder::new();
        range_encoder.insert(5);
        range_encoder.insert(6);
        range_encoder.insert(8);
        let intersection = range_encoder.intersect(&range_encoder);
        assert_eq!(range_encoder, intersection);
    }
    fn intrsection_of_ranges_are_all_itersections() {
        let mut range_encoder1 = RangeEncoder::new();
        for index in [5,6,7,12,13,22,] {
            range_encoder1.insert(index);
        }
        let mut range_encoder2 = RangeEncoder::new();
        for index in [4,5,7,8,11,12,13,22] {
            range_encoder2.insert(index);
        }
        let intersection = range_encoder1.intersect(&range_encoder2);
        assert_eq!(intersection, range_encoder2.intersect(&range_encoder1));
        for index in [5,7,12,13,22] {
            assert!(intersection.is_activ(index));
        }
    }
}