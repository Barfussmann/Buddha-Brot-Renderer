extern crate test;

// use super::range::*;

struct RangeEncoder {
    switch_indicies: Vec<usize>,
}
impl RangeEncoder {
    fn new() -> Self {
        Self {
            switch_indicies: Vec::new(),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.switch_indicies.is_empty()
    }
    pub fn insert(&mut self, index: usize) {
        self.switch_indicies.push(index);
        self.switch_indicies.push(index + 1);
    }
    pub fn activ_cells(&self) -> usize {
        self.switch_indicies.len() / 2
    }
    pub fn is_activ(&self, index: usize) -> bool {
        self.switch_indicies.binary_search(&index).is_ok()
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
    fn activ_cell_equal_to_inserted_cells() {
        let mut range_encoder = RangeEncoder::new();
        for i in 1..100 {
            range_encoder.insert(i);
            assert_eq!(range_encoder.activ_cells(), i);
        }
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
    // #[test]
    // fn after_two_consecutive_insert_only_2_switch_points() {
    //     let mut range_encoder = RangeEncoder::new();
    //     range_encoder.insert(5);
    //     range_encoder.insert(6);
    //     assert_eq!(range_encoder.switch_indicies.len(), 2)
    // }
}