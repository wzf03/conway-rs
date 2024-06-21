use sdl2::{image::Sdl2ImageContext, ttf::Sdl2TtfContext, Sdl, VideoSubsystem};

pub struct SdlContext {
    pub sdl_context: Sdl,
    pub ttf_context: Sdl2TtfContext,
    pub image_context: Sdl2ImageContext,
    pub video_subsystem: VideoSubsystem,
}

impl<'a> SdlContext {
    pub fn new(
        sdl_context: Sdl,
        ttf_context: Sdl2TtfContext,
        image_context: Sdl2ImageContext,
        video_subsystem: VideoSubsystem,
    ) -> SdlContext {
        SdlContext {
            sdl_context,
            ttf_context,
            image_context,
            video_subsystem,
        }
    }
}
