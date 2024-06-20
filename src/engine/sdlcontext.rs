pub struct SdlContext {
    pub sdl_context: sdl2::Sdl,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl SdlContext {
    pub fn new(window_title: String, window_width: u32, window_height: u32) -> SdlContext {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(&window_title, window_width, window_height)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        SdlContext {
            sdl_context,
            video_subsystem,
            canvas,
        }
    }
}
