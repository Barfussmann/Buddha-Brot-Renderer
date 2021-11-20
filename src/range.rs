extern crate test;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Relation {
    Before,
    AdjacentBefore,
    Overlapping,
    AdjacentAfter,
    After,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range {
    start: usize,
    end: usize,
}
impl Range {
    fn new(start: usize, end: usize) -> Self {
        assert!(start < end);
        Self { start, end }
    }
    fn from_index(index: usize) -> Self {
        Self {
            start: index,
            end: index + 1,
        }
    }
    fn is_inside(&self, index: usize) -> bool {
        self.start <= index && index < self.end
    }
    fn relation_to(&self, other: Self) -> Relation {
        if self.end < other.start {
            return Relation::Before;
        }
        if self.start > other.end {
            return Relation::After;
        }
        if self.end == other.start {
            return Relation::AdjacentBefore;
        }
        if self.start == other.end {
            return Relation::AdjacentAfter;
        }
        return Relation::Overlapping;
    }
    fn merge_adjacent(&self, other: Self) -> Self {
        assert!(self.relation_to(other) == Relation::AdjacentBefore || self.relation_to(other) == Relation::AdjacentAfter);
        let new_start = self.start.min(other.start);
        let new_end = self.end.max(other.end);
        Self::new(new_start, new_end)
    }
    fn merge_overlapping(&self, other: Self) -> Self {
        assert!(self.relation_to(other) == Relation::Overlapping);
        let new_start = self.start.min(other.start);
        let new_end = self.end.max(other.end);
        Self::new(new_start, new_end)
    }
    pub fn merge(&self, other: Self) -> Option<Self> {
        match self.relation_to(other) {
            Relation::Before => None,
            Relation::AdjacentBefore => Some(self.merge_adjacent(other)),
            Relation::Overlapping => Some(self.merge_overlapping(other)),
            Relation::AdjacentAfter => Some(self.merge_adjacent(other)),
            Relation::After => None,
        }
    }
    pub fn intersect(&self, other: Self) -> Option<Self> {
        match self.relation_to(other) {
            Relation::Before => None,
            Relation::AdjacentBefore => None,
            Relation::Overlapping => {
                let new_start = self.start.max(other.start);
                let new_end = self.end.min(other.end);
                Some(Self::new(new_start, new_end))
            },
            Relation::AdjacentAfter => None,
            Relation::After => None,
            
        }
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
        let range2 = Range:: new(7,12);
        assert_eq!(range1.relation_to(range2), Relation::Overlapping);
        assert_eq!(range2.relation_to(range1), Relation::Overlapping);
    }
    #[test]
    fn merge_adjacent_before_or_after() {
        let range1 = Range::new(5, 10);
        let range2 = Range::new(10, 20);
        assert_eq!(range1.merge(range2), Some(Range::new(5, 20)));
        assert_eq!(range2.merge(range1), Some(Range::new(5, 20)));
    }
    #[test]
    fn merge_not_adjacent_before_or_after() {
        let range1 = Range::new(5, 10);
        let range2 = Range::new(15, 20);
        assert_eq!(range1.merge(range2), None);
        assert_eq!(range2.merge(range1), None);
    }
    #[test]
    fn merge_overlapping_half() {
        let range1 = Range::new(5, 10);
        let range2 = Range::new(7, 20);
        assert_eq!(range1.merge(range2), Some(Range::new(5, 20)));
        assert_eq!(range2.merge(range1), Some(Range::new(5, 20)));
    }
    #[test]
    fn merge_overlapping_one_inside_other() {
        let range1 = Range::new(5, 15);
        let range2 = Range:: new(7,12);
        assert_eq!(range1.merge(range2), Some(Range::new(5, 15)));
        assert_eq!(range2.merge(range1), Some(Range::new(5, 15)));
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
        let range2 = Range:: new(7,12);
        assert_eq!(range1.intersect(range2), Some(Range::new(7, 12)));
        assert_eq!(range2.intersect(range1), Some(Range::new(7, 12)));
    }
}
