use macroquad::prelude::{
    is_key_down, is_key_pressed, is_mouse_button_pressed, mouse_position_local, set_camera,
    Camera2D, KeyCode, MouseButton, Rect, Vec2,
};

pub struct CameraManger {
    starting_line_width: f32,
    line_width: f32,
    zoom_factor: f32,
    zoom: f32,
    camera_rect: Rect,
}

impl CameraManger {
    pub fn new() -> Self {
        let manger = Self {
            camera_rect: Rect::new(-2.01, -1.26, 3.02, 2.52),
            starting_line_width: 0.002,
            zoom_factor: 2.,
            zoom: 1.,
            line_width: 0.002,
        };
        set_camera(&Camera2D::from_display_rect(manger.camera_rect));
        manger
    }
    pub fn update(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            let inverse_zoom_factor = 1. - 1. / self.zoom_factor;
            let mouse_pos = mouse_position_local();
            let positiv_mouse_pos = 0.5 * (mouse_pos + Vec2::new(1., 1.));
            let new_x =
                self.camera_rect.x + self.camera_rect.w * inverse_zoom_factor * positiv_mouse_pos.x;
            let new_y =
                self.camera_rect.y + self.camera_rect.h * inverse_zoom_factor * positiv_mouse_pos.y;
            let new_h = inverse_zoom_factor * self.camera_rect.h;
            let new_w = inverse_zoom_factor * self.camera_rect.w;
            self.camera_rect = Rect::new(new_x, new_y, new_w, new_h);
            self.zoom *= self.zoom_factor;
            self.line_width = self.starting_line_width / self.zoom;
            set_camera(&Camera2D::from_display_rect(self.camera_rect));
        }
        if is_key_pressed(KeyCode::Space) {
            self.camera_rect = Rect::new(-2.01, -1.26, 3.02, 2.52);
            self.zoom = 1.;
            self.line_width = self.starting_line_width / self.zoom;
            set_camera(&Camera2D::from_display_rect(self.camera_rect));
        }
        if is_key_down(KeyCode::U) {}
        if is_key_down(KeyCode::I) {}
    }
}
