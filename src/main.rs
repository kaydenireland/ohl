pub mod frontend;
pub mod backend;

use clap::Parser;
use crate::frontend::cli::{Cli, handle};

fn main() {
    let args: Cli = Cli::parse();
    handle(args);
}
