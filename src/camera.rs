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
    fn zoom(&mut self, zoom: f32) {
        let mouse_screen_pos = mouse_position_local();

        let camera_corner = Vec2::new(self.camera_rect.x, self.camera_rect.y);
        let camera_size = Vec2::new(self.camera_rect.w, self.camera_rect.h);

        let camera_offset = mouse_screen_pos * camera_size;
        let camara_rect_shrinkage = 0.5 * (1. - 1. / zoom);
        let center_offset = camera_size * camara_rect_shrinkage;
        let new_camera_corner =
            camera_corner + camera_offset * camara_rect_shrinkage + center_offset;

        let new_h = self.camera_rect.h / zoom;
        let new_w = self.camera_rect.w / zoom;
        self.camera_rect = Rect::new(new_camera_corner.x, new_camera_corner.y, new_w, new_h);
        self.zoom *= zoom;
        self.line_width = self.starting_line_width / self.zoom;
        set_camera(&Camera2D::from_display_rect(self.camera_rect));
    }
    pub fn update(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            self.zoom(self.zoom_factor);
        }
        if is_mouse_button_pressed(MouseButton::Right) {
            self.zoom(1. / self.zoom_factor);
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
