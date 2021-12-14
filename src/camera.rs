use super::mandel_brot_render::MandelbrotRender;
use glam::{dvec2, DVec2};
use speedy2d::window::*;
use speedy2d::*;

use super::{WIDTH, HEIGHT};

pub struct CameraManger<T: Updateable> {
    top_left_corner: DVec2,
    view_size: DVec2,
    mouse_poss_at_middle_click: Option<DVec2>,
    had_change: bool,
    draw_cells: bool,
    mouse_pos: DVec2,
    zoom_delta: f64,
    mandel_background: Option<MandelbrotRender>,
    generator: T,
}

impl<T: Updateable> CameraManger<T> {
    pub fn new(mandel_render: bool, generator: T) -> Self {
        Self {
            top_left_corner: dvec2(-2.0, -1.32),
            view_size: dvec2(3.0, 2.64),
            mouse_poss_at_middle_click: None,
            had_change: true,
            draw_cells: false,
            mouse_pos: DVec2::ZERO,
            zoom_delta: 1.0,
            mandel_background: mandel_render.then(|| MandelbrotRender::new()),
            generator,
        }
    }
    pub fn new_only_mandel_render() -> CameraManger<Dummy> {
        CameraManger::new(true, Dummy {})
    }
    fn zoom(&mut self, zoom: f64) {
        if zoom == 1.0 {
            return;
        }
        let camera_offset = (self.get_mouse_pos() - self.top_left_corner) - self.view_size / 2.;
        let camara_rect_shrinkage = 0.5 * (1. - 1. / zoom);
        let center_offset = self.view_size * camara_rect_shrinkage;
        self.top_left_corner += camera_offset * camara_rect_shrinkage * 2. + center_offset;

        self.had_change = true;

        self.view_size /= zoom;
    }
    fn update_drag(&mut self) {
        if let Some(mouse_poss_at_middle_click) = self.mouse_poss_at_middle_click {
            let delta = mouse_poss_at_middle_click - self.get_mouse_pos();
            self.top_left_corner += delta;
            self.had_change = true;
        }
    }
    fn update(&mut self) {
        self.zoom(self.zoom_delta);
    }
    fn get_mouse_pos(&self) -> DVec2 {
        let mouse_offset = self.mouse_pos / dvec2(WIDTH as f64, HEIGHT as f64) * self.view_size;
        self.top_left_corner + mouse_offset
    }
    pub fn get_view_rect(&self) -> (DVec2, DVec2) {
        (self.top_left_corner, self.view_size)
    }
    pub fn draw_cells(&self) -> bool {
        self.draw_cells
    }
    pub fn had_change(&self) -> bool {
        self.had_change
    }
}

#[allow(unused_variables)]
impl<T: Updateable> WindowHandler for CameraManger<T> {
    fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        self.update();

        let mandel_renderer = std::mem::replace(&mut self.mandel_background, None);
        if let Some(mut mandel_renderer) = mandel_renderer {
            mandel_renderer.draw(self.had_change.then(|| self.get_view_rect()), graphics);
            self.mandel_background = Some(mandel_renderer);
        }
        self.had_change = false;
        self.generator.update();
        self.generator.draw(&mut RectDrawer::new(
            self.top_left_corner,
            self.view_size,
            graphics,
        ));
        if !self.generator.is_finished() {
            helper.request_redraw();
            // helper.terminate_loop();
        }
    }
    fn on_mouse_move(&mut self, helper: &mut WindowHelper<()>, position: dimen::Vector2<f32>) {
        self.mouse_pos = dvec2(position.x as f64, position.y as f64);
        self.update_drag();
    }
    fn on_mouse_button_down(&mut self, helper: &mut WindowHelper<()>, button: MouseButton) {
        match button {
            MouseButton::Left => self.zoom_delta = 1.1,
            MouseButton::Middle => self.mouse_poss_at_middle_click = Some(self.get_mouse_pos()),
            MouseButton::Right => self.zoom_delta = 0.9,
            MouseButton::Other(_) => {}
        }
    }
    fn on_mouse_button_up(&mut self, helper: &mut WindowHelper<()>, button: MouseButton) {
        match button {
            MouseButton::Left => self.zoom_delta = 1.0,
            MouseButton::Middle => self.mouse_poss_at_middle_click = None,
            MouseButton::Right => self.zoom_delta = 1.0,
            MouseButton::Other(_) => {}
        }
    }
    fn on_key_down(
        &mut self,
        helper: &mut WindowHelper<()>,
        virtual_key_code: Option<window::VirtualKeyCode>,
        scancode: KeyScancode,
    ) {
        if virtual_key_code == Some(window::VirtualKeyCode::Space) {
            self.top_left_corner = dvec2(-2.0, -1.32);
            self.view_size = dvec2(3.0, 2.64);
            self.had_change = true;
        }
        if virtual_key_code == Some(window::VirtualKeyCode::X) {
            self.draw_cells = !self.draw_cells;
        }
    }
}

pub struct RectDrawer<'a> {
    top_left_corner: DVec2,
    view_size: DVec2,
    graphics: &'a mut Graphics2D,
}
impl<'a> RectDrawer<'a> {
    fn new(top_left_corner: DVec2, view_size: DVec2, graphics: &'a mut Graphics2D) -> Self {
        RectDrawer {
            top_left_corner,
            view_size,
            graphics,
        }
    }
    pub fn draw_rect(&mut self, corner: DVec2, size: DVec2) {
        let corner = (corner - self.top_left_corner) / self.view_size;
        let size = size / self.view_size;
        let screen_mult = dvec2(WIDTH as f64, HEIGHT as f64);
        let screen_corner = dvec2(corner.x, corner.y) * screen_mult;
        let screen_size = dvec2(size.x, size.y) * screen_mult;

        let bottom_right = screen_corner + screen_size;
        let top_left = dimen::Vector2::new(screen_corner.x as f32, screen_corner.y as f32);

        let bottom_right = dimen::Vector2::new(bottom_right.x as f32, bottom_right.y as f32);
        let rect = shape::Rectangle::new(top_left, bottom_right);
        self.graphics.draw_rectangle(rect, color::Color::GREEN);
    }
}

pub trait Updateable {
    fn update(&mut self);
    fn draw(&mut self, rect_drawer: &mut RectDrawer);
    fn is_finished(&self) -> bool;
}
pub struct Dummy;
impl Updateable for Dummy {
    fn draw(&mut self, _rect_drawer: &mut RectDrawer) {}
    fn update(&mut self) {}
    fn is_finished(&self) -> bool {
        false
    }
}
