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

const BACKGROUND_COLOR: Color = Color::WHITE;

pub struct ConwayApp {
    components: ViewChain,
    fps_limiter: Rc<RefCell<FrameLimiter>>,
    pause: Rc<Cell<bool>>,
    running: Rc<Cell<bool>>,
}

impl App for ConwayApp {
    fn create(_viewport: Rect) -> Self {
        let mut app = ConwayApp {
            components: ViewChain::new(),
            fps_limiter: Rc::new(RefCell::new(FrameLimiter::new(60, 1))),
            pause: Rc::new(Cell::new(false)),
            running: Rc::new(Cell::new(false)),
        };

        let board_bound = Rect::new(200, 0, 600, 600);
        let speed_text_bound = Rect::new(20, 20, 150, 40);
        let pause_button_bound = Rect::new(20, 100, 100, 40);
        let speedup_button_bound = Rect::new(20, 150, 100, 40);
        let speeddown_button_bound = Rect::new(20, 200, 100, 40);
        let clear_button_bound = Rect::new(20, 250, 100, 40);
        let load_button_bound = Rect::new(20, 300, 100, 40);
        let save_button_bound = Rect::new(20, 350, 100, 40);
        let exit_button_bound = Rect::new(20, 400, 100, 40);
        let logo_bound = Rect::new(20, 480, 100, 100);

        let board_view = BoardView::new(32, 32, board_bound).wrap();
        app.components.add_view(board_view.clone());

        let speed_text = TextView::new(
            speed_text_bound,
            format!("Speed: {}", app.fps_limiter.borrow().get_tick_rate()),
            &fonts::ARK_PIXEL_FONT,
            26,
            Color::BLACK,
        )
        .wrap();
        app.components.add_view(speed_text.clone());

        let speedup_button_text = TextView::new(
            speedup_button_bound,
            "Speed Up".to_string(),
            &fonts::ARK_PIXEL_FONT,
            18,
            Color::BLACK,
        )
        .wrap();
        app.components.add_view(speedup_button_text);
        let speedup_button = ButtonView::new(
            speedup_button_bound,
            Box::new({
                let speed_text = speed_text.clone();
                let fps_limiter = app.fps_limiter.clone();
                move || {
                    let current_speed = fps_limiter.borrow().get_tick_rate();
                    let next_speed = (current_speed + 1).min(5);
                    fps_limiter.borrow_mut().set_tick_rate(next_speed);
                    speed_text
                        .borrow_mut()
                        .set_text(format!("Speed: {}", next_speed));
                }
            }),
        )
        .wrap();
        app.components.add_view(speedup_button);

        let speeddown_button_text = TextView::new(
            speeddown_button_bound,
            "Speed Down".to_string(),
            &fonts::ARK_PIXEL_FONT,
            18,
            Color::BLACK,
        )
        .wrap();
        app.components.add_view(speeddown_button_text);
        let speeddown_button = ButtonView::new(
            speeddown_button_bound,
            Box::new({
                let speed_text = speed_text.clone();
                let fps_limiter = app.fps_limiter.clone();
                move || {
                    let current_speed = fps_limiter.borrow().get_tick_rate();
                    let next_speed = (current_speed - 1).max(1);
                    fps_limiter.borrow_mut().set_tick_rate(next_speed);
                    speed_text
                        .borrow_mut()
                        .set_text(format!("Speed: {}", next_speed));
                }
            }),
        )
        .wrap();
        app.components.add_view(speeddown_button);

        let pause_button_text = TextView::new(
            pause_button_bound,
            "Pause".to_string(),
            &fonts::ARK_PIXEL_FONT,
            18,
            Color::RGB(0, 0, 0),
        )
        .wrap();
        app.components.add_view(pause_button_text.clone());

        let pause_button = ButtonView::new(
            pause_button_bound,
            Box::new({
                let pause = app.pause.clone();
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
        )
        .wrap();
        app.components.add_view(pause_button);

        let clear_button_text = TextView::new(
            clear_button_bound,
            "Clear".to_string(),
            &fonts::ARK_PIXEL_FONT,
            18,
            Color::BLACK,
        )
        .wrap();
        app.components.add_view(clear_button_text);
        let clear_button = ButtonView::new(
            clear_button_bound,
            Box::new({
                let board_view = board_view.clone();
                move || {
                    board_view.borrow_mut().clear();
                }
            }),
        )
        .wrap();
        app.components.add_view(clear_button);

        let load_button_text = TextView::new(
            load_button_bound,
            "Load".to_string(),
            &fonts::ARK_PIXEL_FONT,
            18,
            Color::BLACK,
        )
        .wrap();
        app.components.add_view(load_button_text);
        let load_button = ButtonView::new(
            load_button_bound,
            Box::new({
                let board_view = board_view.clone();
                move || {
                    if let Err(e) = board_view.borrow_mut().import_from_file() {
                        println!("Error: {}", e);
                    }
                }
            }),
        )
        .wrap();
        app.components.add_view(load_button);

        let save_button_text = TextView::new(
            save_button_bound,
            "Save".to_string(),
            &fonts::ARK_PIXEL_FONT,
            18,
            Color::BLACK,
        )
        .wrap();
        app.components.add_view(save_button_text);
        let save_button = ButtonView::new(
            save_button_bound,
            Box::new({
                let board_view = board_view.clone();
                move || {
                    if let Err(e) = board_view.borrow().export_to_file() {
                        println!("Error: {}", e);
                    }
                }
            }),
        )
        .wrap();
        app.components.add_view(save_button);

        let exit_button_text = TextView::new(
            exit_button_bound,
            "Exit".to_string(),
            &fonts::ARK_PIXEL_FONT,
            18,
            Color::BLACK,
        )
        .wrap();
        app.components.add_view(exit_button_text);

        let exit_button = ButtonView::new(
            exit_button_bound,
            Box::new({
                let running = app.running.clone();
                move || {
                    running.set(false);
                }
            }),
        )
        .wrap();
        app.components.add_view(exit_button);

        // Put a logo to the top right corner
        let logo = ImageView::new(logo_bound, &images::RUST_LOGO_IMG).wrap();
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
        self.running.set(true);
        let mut event_pump = sdl_context.event_pump()?;

        canvas.set_draw_color(BACKGROUND_COLOR);
        canvas.clear();
        canvas.present();

        loop {
            let step_type = self.fps_limiter.borrow_mut().step()?;
            if let FrameStepType::Render | FrameStepType::RenderAndTick = step_type {
                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. }
                        | Event::KeyDown {
                            keycode: Some(Keycode::Escape),
                            ..
                        } => {
                            self.running.set(false);
                            break;
                        }
                        e @ _ => {
                            self.components.on_event(&e);
                        }
                    }
                }

                if !self.running.get() {
                    break;
                }

                // Refresh the screen.
                canvas.set_draw_color(BACKGROUND_COLOR);
                canvas.clear();

                self.components
                    .render(canvas, texture_creator, font_manager)?;

                canvas.present();
            }
            if let FrameStepType::Tick | FrameStepType::RenderAndTick = step_type {
                if !self.pause.get() {
                    self.components.on_tick();
                }
            }
        }
        Ok(())
    }
}
