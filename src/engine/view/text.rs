use sdl2::{pixels::Color, rect::Rect, render::Texture};

use crate::engine::resource_manager::FontDetails;

use super::View;

pub struct TextView {
    bound: Rect,
    text: String,
    font: &'static [u8],
    size: u16,
    color: Color,
    texture: Option<Texture>,
    render_bound: Option<Rect>,
}

impl TextView {
    pub fn new(
        bound: Rect,
        text: String,
        font: &'static [u8],
        size: u16,
        color: Color,
    ) -> TextView {
        TextView {
            bound,
            text,
            font,
            size,
            color,
            texture: None,
            render_bound: None,
        }
    }

    fn destroy_texture(&mut self) {
        let texture = self.texture.take();
        if let Some(texture) = texture {
            unsafe {
                texture.destroy();
            }
        }
        self.render_bound.take();
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.destroy_texture();
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
        self.destroy_texture();
    }
}

impl Drop for TextView {
    fn drop(&mut self) {
        self.destroy_texture();
    }
}

impl View for TextView {
    fn render(
        &mut self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        font_manager: &mut crate::engine::resource_manager::FontManager,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let None = self.texture {
            let font = font_manager.load(&FontDetails {
                content: self.font,
                size: self.size,
            })?;
            let surface = font.render(&self.text).blended(self.color)?;

            self.texture = Some(texture_creator.create_texture_from_surface(&surface)?);
            self.render_bound = Some(Rect::new(
                self.bound.x(),
                self.bound.y(),
                surface.width().min(self.bound.width()),
                surface.height().min(self.bound.height()),
            ));
        }

        let render_bound = self.render_bound.unwrap();
        canvas.copy(
            &self.texture.as_ref().unwrap(),
            Rect::new(
                0,
                0,
                render_bound.width(),
                render_bound.height(),
            ),
            render_bound,
        )?;
        Ok(())
    }

    fn get_bound(&self) -> Rect {
        self.bound
    }
}
