use super::grid::*;
use super::range::*;
use super::rect::*;
use super::range_encoder::*;

struct GridReducer {
    grid: Grid,
    rects: Vec<Rect>,
}
impl GridReducer {
    pub fn new(grid: Grid) -> Self {
        Self {
            grid,
            rects: Vec::new(),
        }
    }
    fn biggest_rect(&self) {

    }
    fn biggest_rect_starting_in_collum(&self, starting_collum_index: usize) -> Option<Rect> {
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
        Some(Rect::new(biggest_range_rect.start, starting_collum_index, biggest_area / h, h))
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
}
