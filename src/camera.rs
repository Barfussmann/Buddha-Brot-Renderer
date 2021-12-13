use glam::{dvec2, DVec2};
use speedy2d::*;

const SIZE: usize = 1024;
const WIDTH: usize = SIZE;
const HEIGHT: usize = (SIZE as f64 * (2.64 / 3.0)) as usize;

pub struct CameraManger {
    top_left_corner: DVec2,
    view_size: DVec2,
    mouse_poss_at_middle_click: Option<DVec2>,
    had_change: bool,
    draw_cells: bool,
    mouse_pos: DVec2,
    zoom_delta: f64,
}

impl CameraManger {
    pub fn new() -> Self {
        Self {
            top_left_corner: dvec2(-2.0, -1.32),
            view_size: dvec2(3.0, 2.64),
            mouse_poss_at_middle_click: None,
            had_change: true,
            draw_cells: false,
            mouse_pos: DVec2::ZERO,
            zoom_delta: 1.0,
        }
    }
    fn zoom(&mut self, zoom: f64) {
        let camera_offset = (self.get_mouse_pos() - self.top_left_corner) - self.view_size / 2.;
        let camara_rect_shrinkage = 0.5 * (1. - 1. / zoom);
        let center_offset = self.view_size * camara_rect_shrinkage;
        println!("{:?}", center_offset);
        self.top_left_corner += camera_offset * camara_rect_shrinkage * 2. + center_offset;

        self.had_change = true;

        self.view_size /= zoom;
    }
    fn update_drag(&mut self) {
        if let Some(mouse_poss_at_middle_click) = self.mouse_poss_at_middle_click {
            let delta = mouse_poss_at_middle_click - self.get_mouse_pos();
            self.top_left_corner += delta;
            println!("{:?}", self.top_left_corner);
            self.had_change = true;
        }
    }
    pub fn on_mouse_move(&mut self, position: DVec2) {
        self.mouse_pos = position;
        self.update_drag();
    }
    pub fn on_mouse_button_up(&mut self, button: window::MouseButton) {
        match button {
            window::MouseButton::Left => self.zoom_delta = 1.0,
            window::MouseButton::Middle => self.mouse_poss_at_middle_click = None,
            window::MouseButton::Right => self.zoom_delta = 1.0,
            window::MouseButton::Other(_) => {}
        }
    }
    pub fn on_mouse_button_down(&mut self, button: window::MouseButton) {
        match button {
            window::MouseButton::Left => self.zoom_delta = 1.1,
            window::MouseButton::Middle => {
                self.mouse_poss_at_middle_click = Some(self.get_mouse_pos())
            }
            window::MouseButton::Right => self.zoom_delta = 0.9,
            window::MouseButton::Other(_) => {}
        }
    }
    pub fn on_key_down(&mut self, scan_code: Option<window::VirtualKeyCode>) {
        self.had_change = false;
        if scan_code == Some(window::VirtualKeyCode::Space) {
            self.top_left_corner = dvec2(-2.0, -1.32);
            self.view_size = dvec2(3.0, 2.64);
            self.had_change = true;
        }
        if scan_code == Some(window::VirtualKeyCode::X) {
            self.draw_cells = !self.draw_cells;
        }
    }
    pub fn update(&mut self) {
        self.zoom(self.zoom_delta);
    }
    fn get_mouse_pos(&self) -> DVec2 {
        let mouse_offset = self.mouse_pos / dvec2(WIDTH as f64, HEIGHT as f64) * self.view_size;
        self.top_left_corner + mouse_offset
    }
    pub fn get_view_rect(&self) -> (DVec2, DVec2) {
        (self.top_left_corner, self.view_size)
    }
    pub fn draw_rect(&self, corner: DVec2, size: DVec2, graphics: &mut speedy2d::Graphics2D) {
        let corner = (corner - self.top_left_corner) / self.view_size;
        let size = size / self.view_size;
        let screen_mult = dvec2(WIDTH as f64, WIDTH as f64);
        let screen_corner = dvec2(corner.x, corner.y) * screen_mult;
        let screen_size = dvec2(size.x, size.y) * screen_mult;

        let screen_corner = dimen::Vector2::new(screen_corner.x as f32, screen_corner.y as f32);
        let screen_size = dimen::Vector2::new(screen_size.x as f32, screen_size.y as f32);
        let rect = shape::Rectangle::new(screen_corner, screen_size);
        graphics.draw_rectangle(rect, color::Color::GREEN);
    }
    pub fn had_change(&self) -> bool {
        self.had_change
    }
    pub fn draw_cells(&self) -> bool {
        self.draw_cells
    }
}
