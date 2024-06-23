use std::{cell::RefCell, rc::Rc};

use sdl2::{rect::Rect, render::TextureCreator, video::WindowContext};

use crate::engine::resource_manager::FontManager;

use super::View;
pub struct ViewChain {
    views: Vec<Rc<RefCell<dyn View>>>,
    bound: Rect,
}

impl ViewChain {
    pub fn new() -> ViewChain {
        ViewChain {
            views: Vec::new(),
            bound: Rect::new(0, 0, 0, 0),
        }
    }

    pub fn add_view(&mut self, view: Rc<RefCell<dyn View>>) {
        self.bound.union(view.borrow().get_bound());
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
            view.borrow_mut()
                .render(canvas, texture_creator, font_manager)?;
        }
        Ok(())
    }

    fn get_bound(&self) -> Rect {
        self.bound
    }

    fn on_tick(&mut self) {
        for view in &mut self.views {
            view.borrow_mut().on_tick();
        }
    }

    fn on_key_down(&mut self, _key: sdl2::keyboard::Keycode) {
        for view in &mut self.views {
            view.borrow_mut().on_key_down(_key);
        }
    }

    fn on_mouse_button_down(&mut self, button: sdl2::mouse::MouseButton, x: i32, y: i32) {
        for view in &mut self.views {
            view.borrow_mut().on_mouse_button_down(button, x, y);
        }
    }

    fn on_mouse_motion(&mut self, _x: i32, _y: i32) {
        for view in &mut self.views {
            view.borrow_mut().on_mouse_motion(_x, _y);
        }
    }
}
