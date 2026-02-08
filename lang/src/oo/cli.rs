use std::io::Write;

use clap::{Parser, Subcommand};
use colored::Colorize;
use crate::core::util::error::Error;
use crate::core::lexer::lexer::Lexer;

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
    Repl {
        #[arg(short, long)]
        debug: bool,
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
        Command::Repl { debug: _debug } => repl(_debug),
        Command::Tokenize { filepath } => tokenize(filepath, true),
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
        let mut e = Error::new(0, 0, "Expected file, got directory.".to_string());
        e.disable_location();
        e.report();
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

pub fn repl(_debug: bool) {
    let mut input: String = String::new();
    loop {
        print!("ohl >>> ");
        std::io::stdout().flush();
        std::io::stdin().read_line(&mut input).expect("Failed to read line.");

        if input.is_empty() {
            break;
        }

        println!("{}", input);
    }
}

pub fn tokenize(path: String, _debug: bool) {
    validate_ohl_file(path.clone());
    let contents = std::fs::read_to_string(path).unwrap();
    let mut lexer = Lexer::new(contents);
    if _debug {
        lexer.print_tokens();
    }
}

pub fn parse(path: String, _debug: bool, print_tree: bool) {
    validate_ohl_file(path.clone());
    let contents = std::fs::read_to_string(path).unwrap();
    /* let lexer = Lexer::new(contents);
    let mut parser = OhlParser::new(lexer, _debug);
    let tree = parser.analyze();
    if print_tree {
        println!("\n\nParse Tree:\n");
        tree.print();
        println!();
    }
    tree */
}

