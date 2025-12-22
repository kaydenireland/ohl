pub mod interface;
pub mod language;

use clap::Parser;
use crate::interface::cli::{Cli, handle};

fn main() {
    let args: Cli = Cli::parse();
    handle(args);
}
