pub mod oo;
pub mod core;

use clap::Parser;
use crate::oo::cli::{Cli, handle};

fn main() {
    let args: Cli = Cli::parse();
    handle(args);
}
