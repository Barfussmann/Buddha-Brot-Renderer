use super::grid::*;
use super::rect::*;

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
}


#[cfg(test)]
mod tests {
    // use super::*;

}