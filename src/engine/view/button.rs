use sdl2::rect::Rect;

use super::View;

pub struct ButtonView {
    bound: Rect,
    callback: Box<dyn Fn()>,
}

impl ButtonView {
    pub fn new(bound: Rect, callback: Box<dyn Fn()>) -> ButtonView {
        ButtonView { bound, callback }
    }
}

impl View for ButtonView {
    fn render(
        &mut self,
        _canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        _texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        _font_manager: &mut crate::engine::resource_manager::FontManager,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn get_bound(&self) -> Rect {
        self.bound
    }

    fn on_mouse_button_down(&mut self, button: sdl2::mouse::MouseButton, x: i32, y: i32) {
        if let sdl2::mouse::MouseButton::Left = button {
            if self.bound.contains_point((x, y)) {
                (self.callback)();
            }
        }
    }
}
