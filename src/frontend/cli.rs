use clap::{Parser, Subcommand};

use crate::backend::lexer::Lexer;
use crate::backend::mtree::MTree;
use crate::backend::parser::Parser as OhlParser;
use crate::backend::semantics::Converter;

#[derive(Parser)]
#[command(name = "oo", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Clone, Subcommand)]
pub enum Command {
    Print {
        filepath: String,
        #[arg(short, long)]
        numbered: bool,
    },
    Tokenize {
        filepath: String,
    },
    Parse {
        filepath: String,
        #[arg(short, long)]
        debug: bool,
    },
    Inspect {
        filepath: String,
        #[arg(short, long)]
        debug: bool,
    },
}

pub fn handle(cli: Cli) {
    match cli.command {
        Command::Print { filepath, numbered } => print(filepath, numbered),
        Command::Tokenize { filepath } => tokenize(filepath),
        Command::Parse { filepath, debug } => _ = parse(filepath, debug),
        Command::Inspect { filepath, debug } => inspect(filepath, debug),
    }
}

pub fn print(path: String, numbered: bool) {
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
    } else {
        println!("{}", contents);
    }
}

pub fn tokenize(path: String) {
    let contents = std::fs::read_to_string(path).unwrap();
    let mut lexer = Lexer::new(contents);
    lexer.print_tokens();
}

pub fn parse(path: String, debug: bool) -> MTree {
    let contents = std::fs::read_to_string(path).unwrap();
    let lexer = Lexer::new(contents);
    let mut parser = OhlParser::new(lexer, debug);
    let tree = parser.analyze();
    println!("\n\nParse Tree:\n");
    tree.print();
    tree
}


pub fn inspect(path: String, debug: bool) {
    let tree: MTree = parse(path, debug);
    let mut converter: Converter = Converter::new(debug);
    converter.convert_tree(&tree);
}

pub fn analyze(path: String, debug: bool) {

}
