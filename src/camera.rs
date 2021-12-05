use glam::{dvec2, DVec2};
use macroquad::prelude::{
    draw_rectangle, is_key_pressed, is_mouse_button_down, is_mouse_button_pressed,
    mouse_position_local, mouse_wheel, screen_height, screen_width, vec2, Color, KeyCode,
    MouseButton, Vec2,
};

pub struct CameraManger {
    top_left_corner: DVec2,
    view_size: DVec2,
    zoom_factor: f64,
    mouse_poss_at_middle_click: DVec2,
    had_change: bool,
}

impl CameraManger {
    pub fn new() -> Self {
        let manger = Self {
            top_left_corner: dvec2(-2.0, -1.32),
            view_size: dvec2(3.0, 2.64),
            zoom_factor: 1.5,
            mouse_poss_at_middle_click: dvec2(0., 0.),
            had_change: true,
        };
        manger
    }
    fn zoom(&mut self, zoom: f64) {
        let mouse_screen_pos_f32 = mouse_position_local();
        let mouse_screen_pos =
            DVec2::new(mouse_screen_pos_f32.x as f64, mouse_screen_pos_f32.y as f64);

        let camera_offset = mouse_screen_pos * self.view_size;
        let camara_rect_shrinkage = 0.5 * (1. - 1. / zoom);
        let center_offset = self.view_size * camara_rect_shrinkage;
        self.top_left_corner += camera_offset * camara_rect_shrinkage + center_offset;

        self.had_change = true;

        self.view_size /= zoom;
    }
    fn update_drag(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left)
            || is_mouse_button_pressed(MouseButton::Middle)
        {
            self.mouse_poss_at_middle_click = self.get_mouse_pos()
        }
        if is_mouse_button_down(MouseButton::Left) || is_mouse_button_down(MouseButton::Middle) {
            let delta = self.mouse_poss_at_middle_click - self.get_mouse_pos();
            self.top_left_corner += delta;
            self.had_change = true;
        }
    }
    pub fn update(&mut self) {
        self.had_change = false;
        if is_key_pressed(KeyCode::Space) {
            self.top_left_corner = dvec2(-2.0, -1.32);
            self.view_size = dvec2(3.0, 2.64);
            self.had_change = true;
        }
        if mouse_wheel().1 == 1. {
            self.zoom(self.zoom_factor);
        } else if mouse_wheel().1 == -1. {
            self.zoom(1. / self.zoom_factor);
        }
        self.update_drag();
    }
    fn get_mouse_pos(&self) -> DVec2 {
        let mouse_screen_pos_f32 = 0.5 * mouse_position_local() + Vec2::ONE;
        let mouse_screen_pos =
            DVec2::new(mouse_screen_pos_f32.x as f64, mouse_screen_pos_f32.y as f64);
        let mouse_offset = mouse_screen_pos * self.view_size;

        self.top_left_corner + mouse_offset
    }
    pub fn get_view_rect(&self) -> (DVec2, DVec2) {
        (self.top_left_corner, self.view_size)
    }
    pub fn draw_rect(&self, corner: DVec2, size: DVec2, color: Color) {
        let corner = (corner - self.top_left_corner) / self.view_size;
        let size = size / self.view_size;
        let screen_mult = vec2(screen_width(), screen_height());
        let screen_corner = vec2(corner.x as f32, corner.y as f32) * screen_mult;
        let screen_size = vec2(size.x as f32, size.y as f32) * screen_mult;
        draw_rectangle(
            screen_corner.x as f32,
            screen_corner.y as f32,
            screen_size.x as f32,
            screen_size.y as f32,
            color,
        );
    }
    pub fn had_change(&self) -> bool {
        self.had_change
    }
}
