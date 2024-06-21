use sdl2::{image::LoadTexture, rect::Rect, render::Texture};

use super::View;

pub struct ImageView {
    bound: Rect,
    image: &'static [u8],
    texture: Option<Texture>,
}

impl ImageView {
    pub fn new(bound: Rect, image: &'static [u8]) -> ImageView {
        ImageView {
            bound,
            image,
            texture: None,
        }
    }

    fn destroy_texture(&mut self) {
        let texture = self.texture.take();
        if let Some(texture) = texture {
            unsafe {
                texture.destroy();
            }
        }
    }
}

impl Drop for ImageView {
    fn drop(&mut self) {
        self.destroy_texture();
    }
}

impl View for ImageView {
    fn render(
        &mut self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        _font_manager: &mut crate::engine::resource_manager::FontManager,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let None = self.texture {
            self.texture = Some(texture_creator.load_texture_bytes(&self.image)?);
        }

        canvas.copy(self.texture.as_ref().unwrap(), None, self.bound)?;

        Ok(())
    }

    fn get_bound(&self) -> Rect {
        self.bound
    }
}
