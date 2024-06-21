use std::error;

pub mod engine;
mod fonts;
mod images;

use engine::{
    app::App,
    view::{ButtonView, ImageView, TextView, View, ViewChain},
    EngineBuilder,
};
use fonts::ARK_PIXEL_FONT;
use images::RUST_LOGO_IMG;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};

struct DemoView {
    bound: Rect,
}

impl View for DemoView {
    fn render(
        &mut self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        _texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        _font_manager: &mut engine::resource_manager::FontManager,
    ) -> Result<(), Box<dyn std::error::Error>> {
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.draw_rect(self.bound)?;
        Ok(())
    }

    fn get_bound(&self) -> Rect {
        self.bound
    }

    fn on_key_down(&self, key: sdl2::keyboard::Keycode) {
        println!("Key down: {:?}", key);
    }
}

struct GameApp {
    view_root: ViewChain,
}

impl GameApp {
    fn new() -> GameApp {
        GameApp {
            view_root: GameApp::init_view(),
        }
    }

    fn init_view() -> ViewChain {
        let demo_view = DemoView {
            bound: Rect::new(128, 128, 128, 128),
        };
        let button = ButtonView::new(
            demo_view.get_bound(),
            Box::new(|| println!("Button clicked")),
        );
        let text = TextView::new(
            demo_view.get_bound(),
            "Hello World!".to_string(),
            &ARK_PIXEL_FONT,
            16,
            Color::RGBA(255, 0, 0, 255),
        );
        let image = ImageView::new(demo_view.get_bound(), &RUST_LOGO_IMG);
        return vec![image.into(), demo_view.into(), button.into(), text.into()].into();
    }
}

impl App for GameApp {
    fn create() -> Self {
        return GameApp::new();
    }

    fn run(
        &mut self,
        sdl_context: &mut sdl2::Sdl,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        font_manager: &mut engine::resource_manager::FontManager,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut event_pump = sdl_context.event_pump()?;
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        canvas.present();

        'mainloop: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'mainloop,
                    e @ _ => {
                        self.view_root.on_event(e);
                    }
                }
            }

            // Refresh the screen.
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas.clear();

            self.view_root
                .render(canvas, texture_creator, font_manager)?;

            canvas.present();
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut engine = EngineBuilder::new()
        .window_title("Conway's Game of Life".into())
        .window_size(800, 600)
        .build()?;
    engine.run_app::<GameApp>()
}
