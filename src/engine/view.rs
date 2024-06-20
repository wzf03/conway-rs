pub mod chain;
pub use self::chain::ViewChain;

pub trait View {
    fn render(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
