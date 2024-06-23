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
        view::{ButtonView, ImageView, TextView, View, ViewChain},
    },
    fonts, images,
};

mod board;
mod game_state;

pub struct ConwayApp {
    board_view: Rc<RefCell<BoardView>>,
    components: ViewChain,
    pause: Rc<Cell<bool>>,
    fps_limiter: Rc<RefCell<FrameLimiter>>,
}

impl App for ConwayApp {
    fn create(viewport: Rect) -> Self {
        let mut app = ConwayApp {
            board_view: Rc::new(RefCell::new(BoardView::new(
                32,
                32,
                Rect::from_center(viewport.center(), 512, 512),
            ))),
            components: ViewChain::new(),
            pause: Rc::new(Cell::new(false)),
            fps_limiter: Rc::new(RefCell::new(FrameLimiter::new(60, 1))),
        };

        let pause = app.pause.clone();

        let pause_button_bound = Rect::new(700, 260, 80, 40);

        let pause_button_text = Rc::new(RefCell::new(TextView::new(
            pause_button_bound,
            "Pause".to_string(),
            &fonts::ARK_PIXEL_FONT,
            16,
            Color::RGB(0, 0, 0),
        )));
        app.components.add_view(pause_button_text.clone());

        let pause_button = Rc::new(RefCell::new(ButtonView::new(
            pause_button_bound,
            Box::new({
                let pause = pause.clone();
                let pause_button_text = pause_button_text.clone();
                move || {
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
                }
            }),
        )));
        app.components.add_view(pause_button);

        let speed_text = Rc::new(RefCell::new(TextView::new(
            Rect::new(20, 20, 80, 40),
            format!("Speed: {}", app.fps_limiter.borrow().get_physics_rate()),
            &fonts::ARK_PIXEL_FONT,
            16,
            Color::BLACK,
        )));
        app.components.add_view(speed_text.clone());

        let speedup_button_bound = Rect::new(20, 100, 80, 40);
        let speedup_button_text = Rc::new(RefCell::new(TextView::new(
            speedup_button_bound,
            "Speed Up".to_string(),
            &fonts::ARK_PIXEL_FONT,
            16,
            Color::BLACK,
        )));
        app.components.add_view(speedup_button_text);
        let speedup_button = Rc::new(RefCell::new(ButtonView::new(
            speedup_button_bound,
            Box::new({
                let speed_text = speed_text.clone();
                let fps_limiter = app.fps_limiter.clone();
                move || {
                    let current_speed = fps_limiter.borrow().get_physics_rate();
                    let next_speed = (current_speed + 1).min(5);
                    fps_limiter.borrow_mut().set_physics_rate(next_speed);
                    speed_text
                        .borrow_mut()
                        .set_text(format!("Speed: {}", next_speed));
                }
            }),
        )));
        app.components.add_view(speedup_button);

        let speeddown_button_bound = Rect::new(20, 150, 80, 40);
        let speeddown_button_text = Rc::new(RefCell::new(TextView::new(
            speeddown_button_bound,
            "Speed Down".to_string(),
            &fonts::ARK_PIXEL_FONT,
            16,
            Color::BLACK,
        )));
        app.components.add_view(speeddown_button_text);
        let speeddown_button = Rc::new(RefCell::new(ButtonView::new(
            speeddown_button_bound,
            Box::new({
                let speed_text = speed_text.clone();
                let fps_limiter = app.fps_limiter.clone();
                move || {
                    let current_speed = fps_limiter.borrow().get_physics_rate();
                    let next_speed = (current_speed - 1).max(1);
                    fps_limiter.borrow_mut().set_physics_rate(next_speed);
                    speed_text
                        .borrow_mut()
                        .set_text(format!("Speed: {}", next_speed));
                }
            }),
        )));
        app.components.add_view(speeddown_button);

        let clear_button_bound = Rect::new(20, 200, 80, 40);
        let clear_button_text = Rc::new(RefCell::new(TextView::new(
            clear_button_bound,
            "Clear".to_string(),
            &fonts::ARK_PIXEL_FONT,
            16,
            Color::BLACK,
        )));
        app.components.add_view(clear_button_text);
        let clear_button = Rc::new(RefCell::new(ButtonView::new(
            clear_button_bound,
            Box::new({
                let board_view = app.board_view.clone();
                move || {
                    board_view.borrow_mut().clear();
                }
            }),
        )));
        app.components.add_view(clear_button);

        let load_button_bound = Rect::new(20, 250, 80, 40);
        let load_button_text = Rc::new(RefCell::new(TextView::new(
            load_button_bound,
            "Load".to_string(),
            &fonts::ARK_PIXEL_FONT,
            16,
            Color::BLACK,
        )));
        app.components.add_view(load_button_text);
        let load_button = Rc::new(RefCell::new(ButtonView::new(
            load_button_bound,
            Box::new({
                let board_view = app.board_view.clone();
                move || {
                    if let Err(e) = board_view.borrow_mut().import_from_file() {
                        println!("Error: {}", e);
                    }
                }
            }),
        )));
        app.components.add_view(load_button);

        let save_button_bound = Rect::new(20, 300, 80, 40);
        let save_button_text = Rc::new(RefCell::new(TextView::new(
            save_button_bound,
            "Save".to_string(),
            &fonts::ARK_PIXEL_FONT,
            16,
            Color::BLACK,
        )));
        app.components.add_view(save_button_text);
        let save_button = Rc::new(RefCell::new(ButtonView::new(
            save_button_bound,
            Box::new({
                let board_view = app.board_view.clone();
                move || {
                    if let Err(e) = board_view.borrow().export_to_file() {
                        println!("Error: {}", e);
                    }
                }
            }),
        )));
        app.components.add_view(save_button);

        // Put a logo to the top right corner
        let logo = Rc::new(RefCell::new(ImageView::new(
            Rect::new(700, 20, 80, 80),
            &images::RUST_LOGO_IMG,
        )));
        app.components.add_view(logo);

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

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        canvas.present();

        'mainloop: loop {
            let step_type = self.fps_limiter.borrow_mut().step()?;
            if let FrameStepType::Render | FrameStepType::RenderAndPhysics = step_type {
                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. }
                        | Event::KeyDown {
                            keycode: Some(Keycode::Escape),
                            ..
                        } => break 'mainloop,
                        e @ _ => {
                            self.board_view.borrow_mut().on_event(&e);
                            self.components.on_event(&e);
                        }
                    }
                }

                // Refresh the screen.
                canvas.set_draw_color(Color::RGB(255, 255, 255));
                canvas.clear();

                self.board_view
                    .borrow_mut()
                    .render(canvas, texture_creator, font_manager)?;
                self.components
                    .render(canvas, texture_creator, font_manager)?;

                canvas.present();
            }
            if let FrameStepType::Physics | FrameStepType::RenderAndPhysics = step_type {
                if !self.pause.get() {
                    self.board_view.borrow_mut().step();
                }
            }
        }
        Ok(())
    }
}
