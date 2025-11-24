use clap::{Parser, Subcommand};

use crate::backend::lexer::Lexer;

#[derive(Parser)]
#[command(name = "oo", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command
}

#[derive(Clone, Subcommand)]
pub enum Command {
    Print { filepath: String, #[arg(short, long)] numbered: bool },
    Tokenize { filepath: String }
}

pub fn handle(cli: Cli){
    match cli.command {
        Command::Print { filepath, numbered } => print(filepath, numbered),
        Command::Tokenize { filepath } => tokenize(filepath),
    }
}


pub fn print(path: String, numbered: bool){
    let contents = std::fs::read_to_string(path).unwrap();

    if numbered {
    let total_lines = contents.lines().count();
        let width = total_lines.to_string().len();

        let mut counter = 0;
        for line in contents.lines() {
            counter += 1;
            let num_str = format!("{num:>width$}", num = counter, width = width);
            println!("{} {} {line}", num_str, "|", line = line);
        }
    }else {
        println!("{}", contents);
    }
}

pub fn tokenize(path: String) {
    let contents = std::fs::read_to_string(path).unwrap();
    let mut lexer = Lexer::new(contents);
    lexer.print_tokens();
}