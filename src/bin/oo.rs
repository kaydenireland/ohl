use std::fs;

use clap::{Parser as ClapParser, Subcommand};

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
    New {
        name: String,
    }
}

pub fn handle(cli: Cli) {
    match cli.command {
        Command::New { name } => {
            match fs::create_dir_all(format!("{}/src", name.clone())) {
                Ok(..) => {},
                Err(msg) => println!("Could Not Create Project: {}", msg)
            }

            let contents: &str = 
"class Main {
    fun main() -> int {
        return 0;
    }
}";

            _ = fs::write(format!("{}/src/main.ohl", name.clone()), contents);
        }
    }
}