extern crate sdl2;

pub mod app;
pub mod error;
pub mod frame_limiter;
pub mod resource_manager;
pub mod sdlcontext;
pub mod view;

use self::app::App;
use self::error::EngineError;
use self::sdlcontext::SdlContext;

use resource_manager::FontManager;
use sdl2::render::Canvas;
use sdl2::video::Window;

enum EngineState {
    Init,
    Running,
    Stopped,
}
pub struct Engine {
    context: SdlContext,
    canvas: Canvas<Window>,
    state: EngineState,
}

impl Engine {
    pub fn run_app<T: App>(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let EngineState::Init = self.state {
            self.state = EngineState::Running;

            let texture_creator = self.canvas.texture_creator();

            let mut font_manager = FontManager::new(&self.context.ttf_context);

            let mut app = T::create(self.canvas.viewport());
            app.run(
                &mut self.context.sdl_context,
                &mut self.canvas,
                &texture_creator,
                &mut font_manager,
            )?;
            Ok(())
        } else {
            Err(error::EngineError::StateError.into())
        }
    }

    pub fn stop(&mut self) -> Result<(), EngineError> {
        if let EngineState::Running = self.state {
            self.state = EngineState::Stopped;
            Ok(())
        } else {
            Err(error::EngineError::StateError)
        }
    }

    pub fn get_context(&mut self) -> &mut SdlContext {
        &mut self.context
    }

    pub fn get_canvas(&mut self) -> &mut Canvas<Window> {
        &mut self.canvas
    }

    pub fn get_context_and_canvas(&mut self) -> (&mut SdlContext, &mut Canvas<Window>) {
        (&mut self.context, &mut self.canvas)
    }

    pub fn get_event_pump(&mut self) -> Result<sdl2::EventPump, EngineError> {
        self.context
            .sdl_context
            .event_pump()
            .map_err(|e| EngineError::SdlError(e))
    }
}

pub struct EngineBuilder {
    window_title: Option<String>,
    window_width: Option<u32>,
    window_height: Option<u32>,
}

impl EngineBuilder {
    pub fn new() -> Self {
        EngineBuilder {
            window_title: None,
            window_width: None,
            window_height: None,
        }
    }

    pub fn window_title(mut self, title: String) -> Self {
        self.window_title = Some(title);
        self
    }

    pub fn window_size(mut self, width: u32, height: u32) -> Self {
        self.window_width = Some(width);
        self.window_height = Some(height);
        self
    }

    pub fn build(self) -> Result<Engine, Box<dyn std::error::Error>> {
        let sdl_context = sdl2::init()?;
        let ttf_context = sdl2::ttf::init()?;
        let image_context = sdl2::image::init(sdl2::image::InitFlag::PNG)?;

        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window(
                &self.window_title.unwrap_or("SDL2 Window".into()),
                self.window_width.unwrap_or(800),
                self.window_height.unwrap_or(600),
            )
            .position_centered()
            .opengl()
            .build()?;

        let canvas = window.into_canvas().build()?;

        let context = SdlContext::new(sdl_context, ttf_context, image_context, video_subsystem);

        Ok(Engine {
            context,
            canvas,
            state: EngineState::Init,
        })
    }
}
