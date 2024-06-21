pub mod button;
pub mod chain;
pub mod image;
pub mod text;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use sdl2::{event::Event, mouse::MouseButton, rect::Rect};

pub use self::button::ButtonView;
pub use self::chain::ViewChain;
pub use self::image::ImageView;
pub use self::text::TextView;

use super::resource_manager::FontManager;

pub trait View {
    fn render(
        &mut self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        texture_creator: &TextureCreator<WindowContext>,
        font_manager: &mut FontManager,
    ) -> Result<(), Box<dyn std::error::Error>>;

    fn get_bound(&self) -> Rect;

    fn on_event(&mut self, event: &Event) {
        match event {
            &Event::KeyDown {
                keycode: Some(key), ..
            } => self.on_key_down(key),
            &Event::MouseButtonDown {
                x, y, mouse_btn, ..
            } => self.on_mouse_button_down(mouse_btn, x, y),
            &Event::MouseMotion { x, y, .. } => self.on_mouse_motion(x, y),
            _ => {}
        }
    }

    fn on_key_down(&mut self, key: sdl2::keyboard::Keycode) {
        let _ = key;
    }
    fn on_mouse_button_down(&mut self, button: MouseButton, x: i32, y: i32) {
        let _ = button;
        let _ = x;
        let _ = y;
    }
    fn on_mouse_motion(&mut self, x: i32, y: i32) {
        let _ = x;
        let _ = y;
    }
}

impl<T> From<T> for Box<dyn View>
where
    T: View + 'static,
{
    fn from(view: T) -> Box<dyn View> {
        Box::new(view)
    }
}
