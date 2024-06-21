use conway_rs::{conway::ConwayApp, engine::EngineBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = EngineBuilder::new()
        .window_title("Conway's Game of Life".into())
        .window_size(800, 600)
        .build()?;
    engine.run_app::<ConwayApp>()
}
