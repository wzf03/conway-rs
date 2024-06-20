extern crate sdl2;

pub mod app;
pub mod error;
pub mod sdlcontext;
pub mod view;

use self::error::EngineError;
use self::sdlcontext::SdlContext;

use app::App;

enum EngineState {
    Init,
    Running,
    Stopped,
}
pub struct Engine {
    pub context: SdlContext,
    state: EngineState,
}

impl Engine {
    pub fn new(window_title: String, window_width: u32, window_height: u32) -> Engine {
        Engine {
            context: SdlContext::new(window_title, window_width, window_height),
            state: EngineState::Init,
        }
    }

    pub fn run_app(&mut self, app: Box<dyn App>) -> Result<(), Box<dyn std::error::Error>> {
        if let EngineState::Init = self.state {
            self.state = EngineState::Running;
            app.run(self)?;
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
}
