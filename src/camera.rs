use super::mandel_brot_render::MandelbrotRender;
use glam::{dvec2, DVec2};
use kludgine::prelude::*;

use super::{WIDTH, HEIGHT};

pub struct CameraManger {
    top_left_corner: DVec2,
    view_size: DVec2,
    mouse_poss_at_middle_click: Option<DVec2>,
    had_change: bool,
    draw_cells: bool,
    mouse_pos: DVec2,
    zoom_delta: f64,
    mandel_background: Option<MandelbrotRender>,
    // generator: Box<dyn Updateable>,
}

impl CameraManger {
    pub fn new(mandel_render: bool) -> Self {
        Self {
            top_left_corner: dvec2(-2.0, -1.32),
            view_size: dvec2(3.0, 2.64),
            mouse_poss_at_middle_click: None,
            had_change: true,
            draw_cells: false,
            mouse_pos: DVec2::ZERO,
            zoom_delta: 1.0,
            mandel_background: mandel_render.then(|| MandelbrotRender::new()),
            // generator,
        }
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

impl Window for CameraManger {

    // fn process_input(
    //     &mut self,
    //     _input: InputEvent,
    //     _status: &mut RedrawStatus,
    //     _scene: &Target,
    //     _window: WindowHandle,
    // ) -> kludgine::app::Result<()>
    // where
    //     Self: Sized,
    // {
    //     Ok(())
    // }

    fn render(
        &mut self,
        scene: &Target,
        status: &mut RedrawStatus,
        _window: WindowHandle,
    ) -> kludgine::app::Result<()> {
        let view_rect = Some(self.get_view_rect());
        if let Some(manedel_backgroud) = self.mandel_background.as_mut() {
            let image =  manedel_backgroud.to_image(view_rect);
            let text = Texture::new(std::sync::Arc::new(image));
            let sprite = SpriteSource::entire_texture(text);
            // Exten

            let rect = Rect::<f32, Pixels>::new(Point::new(0., 0.), Size::new(WIDTH as f32, HEIGHT as f32));
            sprite.render_raw_with_alpha_in_box(scene, rect.as_extents(), SpriteRotation::none(), 1.);
        }
        Ok(())
    }

    // fn update(
    //     &mut self,
    //     scene: &Target,
    //     status: &mut RedrawStatus,
    //     _window: WindowHandle,
    // ) -> kludgine::app::Result<()>
    // where
    //     Self: Sized,
    // {
    //     Ok(())
    // }
}

impl WindowCreator for CameraManger {
    fn window_title(&self) -> String {
        "Mandelbrot".to_string()
    }
}


// #[allow(unused_variables)]
// impl WindowHandler for CameraManger {
//     fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
//         self.update();

//         let mandel_renderer = std::mem::replace(&mut self.mandel_background, None);
//         if let Some(mut mandel_renderer) = mandel_renderer {
//             mandel_renderer.draw(self.had_change.then(|| self.get_view_rect()), graphics);
//             self.mandel_background = Some(mandel_renderer);
//         }
//         self.had_change = false;
//         self.generator.update();
//         self.generator.draw(&mut RectDrawer::new(
//             self.top_left_corner,
//             self.view_size,
//             graphics,
//         ));
//         helper.request_redraw();
//         if !self.generator.is_finished() {
//             helper.terminate_loop();
//         }
//     }
//     fn on_mouse_move(&mut self, helper: &mut WindowHelper<()>, position: dimen::Vector2<f32>) {
//         self.mouse_pos = dvec2(position.x as f64, position.y as f64);
//         self.update_drag();
//     }
//     fn on_mouse_button_down(&mut self, helper: &mut WindowHelper<()>, button: MouseButton) {
//         match button {
//             MouseButton::Left => self.zoom_delta = 1.1,
//             MouseButton::Middle => self.mouse_poss_at_middle_click = Some(self.get_mouse_pos()),
//             MouseButton::Right => self.zoom_delta = 0.9,
//             MouseButton::Other(_) => {}
//         }
//     }
//     fn on_mouse_button_up(&mut self, helper: &mut WindowHelper<()>, button: MouseButton) {
//         match button {
//             MouseButton::Left => self.zoom_delta = 1.0,
//             MouseButton::Middle => self.mouse_poss_at_middle_click = None,
//             MouseButton::Right => self.zoom_delta = 1.0,
//             MouseButton::Other(_) => {}
//         }
//     }
//     fn on_key_down(
//         &mut self,
//         helper: &mut WindowHelper<()>,
//         virtual_key_code: Option<window::VirtualKeyCode>,
//         scancode: KeyScancode,
//     ) {
//         if virtual_key_code == Some(window::VirtualKeyCode::Space) {
//             self.top_left_corner = dvec2(-2.0, -1.32);
//             self.view_size = dvec2(3.0, 2.64);
//             self.had_change = true;
//         }multiwindow
//         if virtual_key_code == Some(window::VirtualKeyCode::X) {
//             self.draw_cells = !self.draw_cells;
//         }
//     }
// }

pub struct RectDrawer {
    top_left_corner: DVec2,
    view_size: DVec2,
}
impl RectDrawer {
    fn new(top_left_corner: DVec2, view_size: DVec2) -> Self {
        RectDrawer {
            top_left_corner,
            view_size,
        }
    }
    pub fn draw_rect(&mut self, corner: DVec2, size: DVec2) {
        // let corner = (corner - self.top_left_corner) / self.view_size;
        // let size = size / self.view_size;
        // let screen_mult = dvec2(WIDTH as f64, HEIGHT as f64);
        // let screen_corner = dvec2(corner.x, corner.y) * screen_mult;
        // let screen_size = dvec2(size.x, size.y) * screen_mult;

        // let bottom_right = screen_corner + screen_size;
        // let top_left = dimen::Vector2::new(screen_corner.x as f32, screen_corner.y as f32);

        // let bottom_right = dimen::Vector2::new(bottom_right.x as f32, bottom_right.y as f32);
        // let rect = shape::Rectangle::new(top_left, bottom_right);
        // self.graphics.draw_rectangle(rect, color::Color::GREEN);
    }
}

// pub trait Updateable {
//     fn update(&mut self);
//     fn draw(&mut self, rect_drawer: &mut RectDrawer);
//     fn is_finished(&self) -> bool;
// }
// pub struct Dummy;
// impl Updateable for Dummy {
//     fn draw(&mut self, _rect_drawer: &mut RectDrawer) {}
//     fn update(&mut self) {}
//     fn is_finished(&self) -> bool {
//         false
//     }
// }
