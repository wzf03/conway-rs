use sdl2::{
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
    Sdl,
};

use super::resource_manager::FontManager;

pub trait App {
    fn create() -> Self;
    fn run(
        &mut self,
        sdl_context: &mut Sdl,
        canvas: &mut Canvas<Window>,
        texture_creator: &TextureCreator<WindowContext>,
        font_manager: &mut FontManager,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
