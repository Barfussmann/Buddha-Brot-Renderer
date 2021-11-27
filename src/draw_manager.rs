use macroquad::prelude::{is_key_pressed, KeyCode};

pub struct DrawManager {
    draw_neighbors: bool,
    draw_new_neighbors: bool,
    draw_inside_cells: bool,
    draw_all_visited_cells: bool,
}

impl DrawManager {
    pub fn new() -> Self {
        Self {
            draw_neighbors: false,
            draw_new_neighbors: false,
            draw_inside_cells: true,
            draw_all_visited_cells: false,
        }
    }
    pub fn update(&mut self) {
        if is_key_pressed(KeyCode::X) {
            self.draw_neighbors = !self.draw_neighbors;
        }
        if is_key_pressed(KeyCode::V) {
            self.draw_new_neighbors = !self.draw_new_neighbors;
        }
        if is_key_pressed(KeyCode::L) {
            self.draw_inside_cells = !self.draw_inside_cells;
        }
        if is_key_pressed(KeyCode::C) {
            self.draw_all_visited_cells = !self.draw_all_visited_cells;
        }
    }
    pub fn get_draw_neighbors(&self) -> bool {
        self.draw_neighbors
    }
    pub fn get_draw_new_neighbors(&self) -> bool {
        self.draw_new_neighbors
    }
    pub fn get_draw_inside_cells(&self) -> bool {
        self.draw_inside_cells
    }
    pub fn get_draw_all_visited_cells(&self) -> bool {
        self.draw_all_visited_cells
    }
}
