use sdl2::{rect::Rect, render::TextureCreator, video::WindowContext};

use crate::engine::resource_manager::FontManager;

use super::View;
pub struct ViewChain {
    views: Vec<Box<dyn View>>,
    bound: Rect,
}

impl ViewChain {
    pub fn new() -> ViewChain {
        ViewChain {
            views: Vec::new(),
            bound: Rect::new(0, 0, 0, 0),
        }
    }

    pub fn add_view(&mut self, view: Box<dyn View>) {
        self.bound.union(view.get_bound());
        self.views.push(view);
    }
}

impl View for ViewChain {
    fn render(
        &mut self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        texture_creator: &TextureCreator<WindowContext>,
        font_manager: &mut FontManager,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for view in &mut self.views {
            view.render(canvas, texture_creator, font_manager)?;
        }
        Ok(())
    }

    fn get_bound(&self) -> Rect {
        self.bound
    }

    fn on_key_down(&self, _key: sdl2::keyboard::Keycode) {
        for view in &self.views {
            view.on_key_down(_key);
        }
    }

    fn on_mouse_button_down(&self, button: sdl2::mouse::MouseButton, x: i32, y: i32) {
        for view in &self.views {
            view.on_mouse_button_down(button, x, y);
        }
    }

    fn on_mouse_motion(&self, _x: i32, _y: i32) {
        for view in &self.views {
            view.on_mouse_motion(_x, _y);
        }
    }
}

impl From<Vec<Box<dyn View>>> for ViewChain {
    fn from(views: Vec<Box<dyn View>>) -> ViewChain {
        let bound = Rect::new(0, 0, 0, 0);
        for view in &views {
            bound.union(view.get_bound());
        }
        ViewChain { views, bound }
    }
}
