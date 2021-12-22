use std::sync::Arc;

use super::mandel_brot_render::MandelbrotRender;
use glam::{dvec2, DVec2};
use kludgine::core::easygpu::figures::SizedRect;
use kludgine::core::image::RgbaImage;
use kludgine::prelude::*;

use super::{HEIGHT, WIDTH};
const MAX_RECTS: usize = 15_000;

pub struct CameraManger<T: Updateable> {
    view_rect: ViewRect,
    mouse_poss_at_click: Option<DVec2>,
    mouse_pos: DVec2,
    mandel_background: Option<MandelbrotRender>,
    redraw_requester: Option<RedrawRequester>,
    generator: T,
}

impl<T> CameraManger<T>
where
    T: Updateable + Sync + Send + 'static,
{
    pub fn new(mandel_render: bool, generator: T) -> Self {
        Self {
            view_rect: ViewRect::default(),
            mouse_poss_at_click: None,
            mouse_pos: DVec2::ZERO,
            mandel_background: mandel_render.then(MandelbrotRender::new),
            redraw_requester: None,
            generator,
        }
    }
    fn zoom(&mut self, zoom: f64) {
        let camera_offset = (self.get_mouse_pos() - self.view_rect.top_left_corner) - self.view_rect.view_size / 2.;
        let camara_rect_shrinkage = 0.5 * (1. - 1. / zoom);
        let center_offset = self.view_rect.view_size * camara_rect_shrinkage;
        self.view_rect.top_left_corner += camera_offset * camara_rect_shrinkage * 2. + center_offset;

        self.view_rect.view_size /= zoom;
        
        self.request_redraw();
    }
    fn update_drag(&mut self) {
        if let Some(mouse_poss_at_middle_click) = self.mouse_poss_at_click {
            let delta = mouse_poss_at_middle_click - self.get_mouse_pos();
            self.view_rect.top_left_corner += delta;
            self.request_redraw();
        }
    }
    fn get_mouse_pos(&self) -> DVec2 {
        let mouse_offset = self.mouse_pos / dvec2(WIDTH as f64, HEIGHT as f64) * self.view_rect.view_size;
        self.view_rect.top_left_corner + mouse_offset
    }
    pub fn get_view_rect(&self) -> ViewRect {
        self.view_rect
    }
    pub fn reset_zoom(&mut self) {
        self.view_rect = ViewRect::default();
        self.request_redraw();
    }
    fn request_redraw(&mut self) {
        self.generator.update_view_rect(self.view_rect);
        self.redraw_requester.as_ref().unwrap().request_redraw();
    }
}

impl<T> Window for CameraManger<T>
where
    T: Updateable + Sync + Send + 'static,
{
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
            Event::Keyboard { key, .. } => {
                if key == Some(VirtualKeyCode::Space) {
                    self.reset_zoom()
                }
            }
            Event::MouseButton { state, .. } => match state {
                ElementState::Pressed => self.mouse_poss_at_click = Some(self.get_mouse_pos()),
                ElementState::Released => self.mouse_poss_at_click = None,
            },
            Event::MouseMoved { position } => {
                if let Some(position) = position {
                    self.mouse_pos = dvec2(position.x as f64, position.y as f64);
                    self.update_drag();
                }
            }
            Event::MouseWheel { delta, .. } => {
                if let MouseScrollDelta::LineDelta(_, y) = delta {
                    self.zoom(1. + y as f64 / 2.)
                }
            }
        }
        Ok(())
    }

    fn render(
        &mut self,
        scene: &Target,
        _status: &mut RedrawStatus,
        _window: WindowHandle,
    ) -> kludgine::app::Result<()> {
        let mut drawer = Drawer::new(self.view_rect, scene);
        let view_rect = self.get_view_rect();
        if let Some(manedel_backgroud) = self.mandel_background.as_mut() {
            drawer.draw_raw_pixels(manedel_backgroud.get_raw_pixels(view_rect));
        }
        self.generator.draw(&mut drawer);
        Ok(())
    }
    fn update(
        &mut self,
        _scene: &Target,
        status: &mut RedrawStatus,
        window: WindowHandle,
    ) -> kludgine::app::Result<()>
    where
        Self: Sized,
    {
        self.generator.update();
        status.set_needs_redraw();
        if self.generator.is_finished() {
            window.request_close();
            self.generator.finish();
        }
        Ok(())
    }
}

impl<T> WindowCreator for CameraManger<T>
where
    T: Updateable + Sync + Send + 'static,
{
    fn window_title(&self) -> String {
        "Mandelbrot".to_string()
    }
    fn initial_size(&self) -> Size<u32, kludgine::core::figures::Points> {
        Size::new(WIDTH as u32, HEIGHT as u32)
    }
}

pub struct Drawer<'a> {
    view_rect: ViewRect,
    scene: &'a Target,
    current_rect: usize,
}
impl<'a> Drawer<'a> {
    const fn new(view_rect: ViewRect, scene: &'a Target) -> Self {
        Drawer {
            view_rect,
            scene,
            current_rect: 0,
        }
    }
    pub fn draw_raw_pixels(&mut self, rgba_pixels: Vec<u8>) {
        let image = RgbaImage::from_raw(WIDTH as u32, HEIGHT as u32, rgba_pixels).unwrap();
        let sprite = SpriteSource::entire_texture(Texture::new(Arc::new(image)));
        sprite.render_at(
            self.scene,
            Point::<_, Pixels>::new(0.0_f32, 0.0_f32),
            SpriteRotation::none(),
        );
    }
    // return true when drawing was succsesful
    pub fn draw_rect(&mut self, corner: DVec2, size: DVec2) -> bool {
        if self.current_rect >= MAX_RECTS {
            return false;
        }
        self.current_rect += 1;

        let corner = (corner - self.view_rect.top_left_corner) / self.view_rect.view_size;
        let size = size / self.view_rect.view_size;
        let screen_mult = dvec2(WIDTH as f64, HEIGHT as f64);
        let screen_corner = dvec2(corner.x, corner.y) * screen_mult;
        let screen_size = dvec2(size.x, size.y) * screen_mult;

        let rect: SizedRect<_, Pixels> = Rect::new(
            Point::new(screen_corner.x as f32, screen_corner.y as f32),
            Size::new(screen_size.x as f32, screen_size.y as f32),
        );

        let rect = Shape::rect(rect).fill(Fill::new(Color::GREEN));

        rect.render(self.scene);
        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ViewRect {
    pub top_left_corner: DVec2,
    pub view_size: DVec2,
}
impl ViewRect {
    pub fn get_screen_scale(&self) -> DVec2 {
        dvec2(WIDTH as f64, HEIGHT as f64) / self.view_size
    }
    pub fn get_bottom_right_corner(&self) -> DVec2 {
        self.top_left_corner + self.view_size
    }
}
impl Default for ViewRect {
    fn default() -> Self {
        Self {
            top_left_corner: dvec2(-2.0, -1.32),
            view_size: dvec2(3.0, 2.64),
        }
    }
}

pub trait Updateable {
    fn update(&mut self) {}
    fn draw(&mut self, _drawer: &mut Drawer) {}
    fn is_finished(&self) -> bool {
        false
    }
    fn finish(&mut self) {}
    fn update_view_rect(&mut self, _view_rect: ViewRect) {}
}
