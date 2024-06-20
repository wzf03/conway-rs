use std::error;

pub mod engine;

use engine::app::App;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

struct GameApp {}

impl GameApp {
    fn new() -> GameApp {
        GameApp {}
    }
}

impl App for GameApp {
    fn run(&self, engine: &mut engine::Engine) -> Result<(), Box<dyn std::error::Error>> {
        let canvas = &mut engine.get_context().canvas;

        canvas.set_draw_color(Color::RGB(255, 255, 0));
        canvas.clear();
        canvas.present();

        let mut event_pump = engine.get_context().sdl_context.event_pump()?;

        'mainloop: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'mainloop,
                    _ => {}
                }
            }
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut engine = engine::Engine::new("Game".to_string(), 800, 600);
    engine.run_app(GameApp::new().into())
}
