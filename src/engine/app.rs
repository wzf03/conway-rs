use super::Engine;

pub trait App {
    fn run(&self, engine: &mut Engine) -> Result<(), Box<dyn std::error::Error>>;
}

impl<T> From<T> for Box<dyn App>
where
    T: App + 'static,
{
    fn from(app: T) -> Box<dyn App> {
        Box::new(app)
    }
}
