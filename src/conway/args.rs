extern crate clap;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(super) struct ConwayArgs {
    #[arg(long, default_value = "32")]
    pub width: usize,

    #[arg(long, default_value = "32")]
    pub height: usize,

    #[arg(short, long)]
    pub periodic: bool,
}
