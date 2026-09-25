use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::{Command as TerminalCommand, ExitStatus};

use clap::{Parser as ClapParser, Subcommand};
use colored::Colorize;
use ohl::ohl::converter::stree::STree;
use ohl::oil::intermediate::program::IntermediateProgram;
use ohl::oil::machine::program::MachineProgram;

fn main() {
    let args: Cli = Cli::parse();
    handle(args);
}


#[derive(ClapParser)]
#[command(name = "ohlc", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Clone, Subcommand)]
pub enum Command {
    Test,
    Hello

}

pub fn handle(cli: Cli) {
    match cli.command {
        Command::Test => println!("test"),
        Command::Hello => println!("Hello, World!"),
    }
}