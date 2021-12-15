use super::mandel_brot_render::MandelbrotRender;
use glam::{dvec2, DVec2};
use kludgine::prelude::*;
use kludgine::core::easygpu::figures::SizedRect;
use kludgine::core::image::RgbaImage;


use super::{HEIGHT, WIDTH};

pub struct CameraManger {
    top_left_corner: DVec2,
    view_size: DVec2,
    mouse_poss_at_click: Option<DVec2>,
    mouse_pos: DVec2,
    mandel_background: Option<MandelbrotRender>,
    redraw_requester: Option<RedrawRequester>,
    // generator: Box<dyn Updateable>,
}

impl CameraManger {
    pub fn new(mandel_render: bool) -> Self {
        Self {
            top_left_corner: dvec2(-2.0, -1.32),
            view_size: dvec2(3.0, 2.64),
            mouse_poss_at_click: None,
            mouse_pos: DVec2::ZERO,
            mandel_background: mandel_render.then(|| MandelbrotRender::new()),
            redraw_requester: None,
            // generator,
        }
    }
    fn zoom(&mut self, zoom: f64) {
        let camera_offset = (self.get_mouse_pos() - self.top_left_corner) - self.view_size / 2.;
        let camara_rect_shrinkage = 0.5 * (1. - 1. / zoom);
        let center_offset = self.view_size * camara_rect_shrinkage;
        self.top_left_corner += camera_offset * camara_rect_shrinkage * 2. + center_offset;

        self.request_redraw();

        self.view_size /= zoom;
    }
    fn update_drag(&mut self) {
        if let Some(mouse_poss_at_middle_click) = self.mouse_poss_at_click {
            let delta = mouse_poss_at_middle_click - self.get_mouse_pos();
            self.top_left_corner += delta;
            self.request_redraw();
        }
    }
    fn get_mouse_pos(&self) -> DVec2 {
        let mouse_offset = self.mouse_pos / dvec2(WIDTH as f64, HEIGHT as f64) * self.view_size;
        self.top_left_corner + mouse_offset
    }
    pub fn get_view_rect(&self) -> (DVec2, DVec2) {
        (self.top_left_corner, self.view_size)
    }
    pub fn reset_zoom(&mut self) {
        self.top_left_corner = dvec2(-2.0, -1.32);
        self.view_size = dvec2(3.0, 2.64);
        self.request_redraw();
    }
    fn request_redraw(&self) {
        self.redraw_requester.as_ref().unwrap().request_redraw();
    }
}

impl Window for CameraManger {
    fn initialize(
        &mut self,
        _scene: &Target,
        redrawer: RedrawRequester,
        _window: WindowHandle,
    ) -> kludgine::app::Result<()>
    where
        Self: Sized,
    {
        self.redraw_requester = Some(redrawer);
        Ok(())
    }

    fn process_input(
        &mut self,
        input: InputEvent,
        _status: &mut RedrawStatus,
        _scene: &Target,
        _window: WindowHandle,
    ) -> kludgine::app::Result<()>
    where
        Self: Sized,
    {
        match input.event {
            Event::Keyboard {
                scancode,
                key,
                state,
            } => match key {
                Some(VirtualKeyCode::Space) => self.reset_zoom(),
                _ => (),
            },
            Event::MouseButton { state, .. } => {
                match state {
                    ElementState::Pressed => self.mouse_poss_at_click = Some(self.get_mouse_pos()),
                    ElementState::Released => self.mouse_poss_at_click = None,
                }
            },
            Event::MouseMoved { position } => {
                if let Some(position) = position {
                    self.mouse_pos = dvec2(position.x as f64, position.y as f64);
                    self.update_drag();
                }
            }
            Event::MouseWheel { delta, touch_phase } => match delta {
                MouseScrollDelta::LineDelta(_, y) => self.zoom(1. + y as f64 / 2.),
                _ => (),
            },
        }
        Ok(())
    }
    fn render(
        &mut self,
        scene: &Target,
        status: &mut RedrawStatus,
        _window: WindowHandle,
    ) -> kludgine::app::Result<()> {
        let view_rect = self.get_view_rect();
        if let Some(manedel_backgroud) = self.mandel_background.as_mut() {
            let drawer = Drawer::new(self.top_left_corner, self.view_size, scene);
            drawer.draw_raw_pixels(manedel_backgroud.get_raw_pixels(view_rect));
        }
        Ok(())
    }
}

impl WindowCreator for CameraManger {
    fn window_title(&self) -> String {
        "Mandelbrot".to_string()
    }
    fn initial_size(&self) -> Size<u32, kludgine::core::figures::Points> {
        Size::new(WIDTH as u32, HEIGHT as u32)
    }
}


pub struct Drawer<'a> {
    top_left_corner: DVec2,
    view_size: DVec2,
    scene: &'a Target,
}
impl<'a> Drawer<'a> {
    fn new(top_left_corner: DVec2, view_size: DVec2, scene: &'a Target) -> Self {
        Drawer {
            top_left_corner,
            view_size,
            scene,
        }
    }
    pub fn draw_raw_pixels(&self, rgba_pixels: Vec<u8>) {
        let image = RgbaImage::from_raw(WIDTH as u32, HEIGHT as u32, rgba_pixels).unwrap();
        let texture = Texture::new(std::sync::Arc::new(image));
        let sprite = SpriteSource::entire_texture(texture);

        let rect = Rect::<f32, Pixels>::new(
            Point::new(0., 0.),
            Size::new(WIDTH as f32, HEIGHT as f32),
        );
        sprite.render_raw_with_alpha_in_box(
            &self.scene,
            rect.as_extents(),
            SpriteRotation::none(),
            1.,
        );

    }
    pub fn draw_rect(&self, corner: DVec2, size: DVec2) {
        let corner = (corner - self.top_left_corner) / self.view_size;
        let size = size / self.view_size;
        let screen_mult = dvec2(WIDTH as f64, HEIGHT as f64);
        let screen_corner = dvec2(corner.x, corner.y) * screen_mult;
        let screen_size = dvec2(size.x, size.y) * screen_mult;


        let rect: SizedRect<_, Pixels> = Rect::new(
            Point::new(screen_corner.x as f32, screen_corner.y as f32),
            Size::new(screen_size.x as f32, screen_size.y as f32),
        );

        let rect = Shape::rect(rect);
        rect.render(&self.scene);
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
