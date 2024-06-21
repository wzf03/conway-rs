use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use board::BoardView;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};

use crate::{
    engine::{
        app::App,
        frame_limiter::{FrameLimiter, FrameStepType},
        view::{ButtonView, TextView, View, ViewChain},
    },
    fonts,
};

mod board;
mod game_state;

pub struct ConwayApp {
    board_view: BoardView,
    components: ViewChain,
    pause: Rc<Cell<bool>>,
}

impl App for ConwayApp {
    fn create(viewport: Rect) -> Self {
        let mut app = ConwayApp {
            board_view: BoardView::new(64, 64, Rect::from_center(viewport.center(), 512, 512)),
            components: ViewChain::new(),
            pause: Rc::new(Cell::new(false)),
        };

        let pause = app.pause.clone();

        let pause_button_text = Rc::new(RefCell::new(TextView::new(
            Rect::new(700, 260, 80, 40),
            "Pause".to_string(),
            &fonts::ARK_PIXEL_FONT,
            16,
            Color::RGB(0, 0, 0),
        )));
        app.components.add_view(pause_button_text.clone());

        let pause_button = Rc::new(RefCell::new(ButtonView::new(
            Rect::new(700, 260, 80, 40),
            Box::new(move || {
                let is_paused = pause.get();
                if is_paused {
                    pause.set(false);
                    pause_button_text.borrow_mut().set_text("Pause".to_string());
                } else {
                    pause.set(true);
                    pause_button_text
                        .borrow_mut()
                        .set_text("Resume".to_string());
                }
            }),
        )));
        app.components.add_view(pause_button);

        app
    }
    fn run(
        &mut self,
        sdl_context: &mut sdl2::Sdl,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        font_manager: &mut crate::engine::resource_manager::FontManager,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut event_pump = sdl_context.event_pump()?;
        let mut fps_limiter = FrameLimiter::new(60, 1);

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        canvas.present();

        'mainloop: loop {
            let step_type = fps_limiter.step()?;
            if let FrameStepType::Render | FrameStepType::RenderAndPhysics = step_type {
                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. }
                        | Event::KeyDown {
                            keycode: Some(Keycode::Escape),
                            ..
                        } => break 'mainloop,
                        e @ _ => {
                            self.board_view.on_event(&e);
                            self.components.on_event(&e);
                        }
                    }
                }

                // Refresh the screen.
                canvas.set_draw_color(Color::RGB(255, 255, 255));
                canvas.clear();

                self.board_view
                    .render(canvas, texture_creator, font_manager)?;
                self.components
                    .render(canvas, texture_creator, font_manager)?;

                canvas.present();
            }
            if let FrameStepType::Physics | FrameStepType::RenderAndPhysics = step_type {
                if !self.pause.get() {
                    self.board_view.step();
                }
            }
        }
        Ok(())
    }
}
