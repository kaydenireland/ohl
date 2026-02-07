use clap::{Parser, Subcommand};
use colored::Colorize;

use crate::core::tokenizer::lexer::Lexer;
use crate::core::parser::mtree::MTree;
use crate::core::parser::parser::Parser as OhlParser;


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
}

pub fn handle(cli: Cli) {
    match cli.command {
        Command::Print { filepath, numbered } => print(filepath, numbered),
        Command::Size { filepath } => size(filepath),
        Command::Tokenize { filepath } => tokenize(filepath),
        Command::Parse { filepath, debug: _debug } => _ = parse(filepath, _debug, true),
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
            println!("{} {} {line}", num_str.dimmed(), "|".dimmed(), line = line);
        }
    } else {
        println!("{}", contents);
    }
}

pub fn size(path: String) {
    use std::fs;
    let data = fs::metadata(path.clone()).unwrap_or_else( |e| {
        eprint!("Failed to get size of file at {}: {}\n", path.yellow(), e.to_string().red());
        std::process::exit(1);
    });

    print!("{} bytes", data.len().to_string().cyan());
}

pub fn validate_ohl_file(path: String) {
    use std::path::Path;

    let p = Path::new(&path);

    if p.is_dir() {
        eprintln!("Expected a file, got a directory");
        std::process::exit(0);
    }


    match p.extension().and_then(|e| e.to_str()) {
        Some("ohl") => {}
        _ => {
            eprintln!(
                "{}: expected an .ohl file, got '{}'",
                "Error".red(),
                path
            );
            std::process::exit(0);
        }
    }
}

pub fn tokenize(path: String) {
    validate_ohl_file(path.clone());
    let contents = std::fs::read_to_string(path).unwrap();
    let mut lexer = Lexer::new(contents);
    lexer.print_tokens();
}

pub fn parse(path: String, _debug: bool, print_tree: bool) -> MTree {
    validate_ohl_file(path.clone());
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

