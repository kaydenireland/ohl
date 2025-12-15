use clap::{Parser, Subcommand};

use crate::language::tokenizing::lexer::Lexer;
use crate::language::parsing::mtree::MTree;
use crate::language::parsing::parser::Parser as OhlParser;
use crate::language::analyzing::converter::Converter;
use crate::language::analyzing::stree::STree;


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
    Size {
        filepath: String,
    },
    Tokenize {
        filepath: String,
    },
    Parse {
        filepath: String,
        #[arg(short, long)]
        debug: bool,
    },
    Convert {
        filepath: String,
        #[arg(short, long)]
        debug: bool,
    },
    Execute {
        filepath: String,
        #[arg(short, long)]
        debug: bool,
    },
}

pub fn handle(cli: Cli) {
    match cli.command {
        Command::Print { filepath, numbered } => print(filepath, numbered),
        Command::Size { filepath } => size(filepath),
        Command::Tokenize { filepath } => tokenize(filepath),
        Command::Parse { filepath, debug: _debug } => _ = parse(filepath, _debug, true),
        Command::Convert { filepath, debug: _debug } => _ = convert(filepath, _debug, true),
        Command::Execute { filepath, debug: _debug } => execute(filepath, _debug),
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

pub fn size(path: String) {
    use std::fs;
    let data = fs::metadata(path.clone()).unwrap_or_else( |e| {
        eprint!("Failed to get size of file at {}: {}", path, e);
        std::process::exit(1);
    });

    print!("{:?} bytes", data.len());
}

pub fn tokenize(path: String) {
    let contents = std::fs::read_to_string(path).unwrap();
    let mut lexer = Lexer::new(contents);
    lexer.print_tokens();
}

pub fn parse(path: String, _debug: bool, print_tree: bool) -> MTree {
    let contents = std::fs::read_to_string(path).unwrap();
    let lexer = Lexer::new(contents);
    let mut parser = OhlParser::new(lexer, _debug);
    let tree = parser.analyze();
    if print_tree {
        println!("\n\nParse Tree:\n");
        tree.print();
        println!();
    }
    tree
}


pub fn convert(path: String, _debug: bool, print_tree: bool) -> STree {
    let mtree: MTree = parse(path, _debug, _debug);
    let mut converter: Converter = Converter::new(_debug);
    let result: Result<STree, String> = converter.convert_tree(&mtree);
    let stree = match result {
        Ok(s) => s,
        Err(e) => {
            eprintln!("ERROR: Semantic Conversion Failed \n{}", e);
            std::process::exit(1)
        }
    };

    if print_tree {
        println!("\n\nSemantic Tree:\n{:#?}", stree);
    }

    stree
}

pub fn execute(path: String, _debug: bool) {
    println!("Coming Soon");
    convert(path, _debug, _debug);
}
