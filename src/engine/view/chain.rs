use super::View;
pub struct ViewChain {
    views: Vec<Box<dyn View>>,
}

impl ViewChain {
    pub fn new() -> ViewChain {
        ViewChain { views: Vec::new() }
    }

    pub fn add_view(&mut self, view: Box<dyn View>) {
        self.views.push(view);
    }
}

impl View for ViewChain {
    fn render(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for view in &self.views {
            view.render(canvas)?;
        }
        Ok(())
    }
}
