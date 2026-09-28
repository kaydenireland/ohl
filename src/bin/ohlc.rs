use std::fs;
use clap::{Parser as ClapParser, Subcommand};
use ohl::oil::lexer::lexer::Lexer;

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
    Token {
        filepath: String,
    },

}

pub fn handle(cli: Cli) {
    match cli.command {
        Command::Token { filepath } => {
            let contents = get_oil_source(filepath);
            tokenize(contents, true);
        }
    }
}

pub fn tokenize(src: String, _debug: bool) -> Lexer {
    // expect source input to already be validated
    let mut lexer = Lexer::new(src);
    if _debug {
        lexer.print_tokens();
        lexer.reset();
    }

    lexer
}

// Utility
fn get_oil_source(filepath: String) -> String {
    ohl::validate_file_extension(filepath.clone(), "oil".to_string());
    fs::read_to_string(filepath).unwrap()
}