use super::grid::*;
use super::range::*;
use super::range_encoder::*;
use super::u_rect::*;

pub struct GridReducer {
    grid: Grid,
    rects: Vec<URect>,
}
impl GridReducer {
    pub fn new(grid: Grid) -> Self {
        Self {
            grid,
            rects: Vec::new(),
        }
    }
    pub fn biggest_rect(&self) -> Option<URect> {
        if self.grid.is_empty() {
            return None;
        }
        let mut biggest_rect = URect::new(0, 0, usize::MAX, usize::MAX);
        let mut biggest_area = 0;
        for i in 0..self.grid.get_grid_size() {
            if let Some(new_biggest_rect) = self
                .biggest_rect_starting_in_collum(i)
                .filter(|rect| rect.area() > biggest_area)
            {
                biggest_area = new_biggest_rect.area();
                biggest_rect = new_biggest_rect;
            }
        }
        assert_ne!(biggest_rect, URect::new(0, 0, usize::MAX, usize::MAX));
        Some(biggest_rect)
    }
    fn biggest_rect_starting_in_collum(&self, starting_collum_index: usize) -> Option<URect> {
        fn get_longest_range(range_encoder: &RangeEncoder) -> Range {
            *range_encoder
                .get_activ_ranges()
                .iter()
                .max_by_key(|range| range.len())
                .unwrap()
        }
        let collum = self.grid.get_collum(starting_collum_index);
        if collum.is_empty() {
            return None;
        }
        let mut intersection = collum.clone();
        let mut biggest_range_rect = get_longest_range(&intersection);
        let mut biggest_area = biggest_range_rect.len();
        for collum_index in starting_collum_index + 1..self.grid.get_grid_size() {
            intersection = intersection.intersect(self.grid.get_collum(collum_index));
            if intersection.is_empty() {
                break;
            }
            let longest_range = get_longest_range(&intersection);
            let delta_y = collum_index - starting_collum_index;
            if longest_range.len() * delta_y > biggest_area {
                biggest_range_rect = longest_range;
                biggest_area = biggest_range_rect.len() * delta_y;
            }
        }
        let h = biggest_range_rect.len();
        debug_assert_eq!(biggest_area.rem_euclid(h), 0);
        Some(URect::new(
            biggest_range_rect.start,
            starting_collum_index,
            biggest_area / h,
            h,
        ))
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
}
