extern crate clap;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "Conway's Game of Life in Rust!", long_about = None)]
pub(super) struct ConwayArgs {
    #[arg(long, short = 'W', default_value = "32", help = "The width of the board")]
    pub width: usize,

    #[arg(long, short = 'H', default_value = "32", help = "The height of the board")]
    pub height: usize,

    #[arg(short, long, help = "Enable periodic boundary conditions")]
    pub periodic: bool,
}
