use std::fmt::format;
use std::fs::File;
use std::io::{Write, Result};
use std::path::Path;

use clap::{Parser as ClapParser, Subcommand};
use colored::Colorize;
use inkwell::context::Context;
use crate::core::converter::converter::Converter;
use crate::core::converter::stree::STree;
use crate::core::ir::codegen::CodeGen;
use crate::core::parser::mtree::MTree;
use crate::core::parser::parser::Parser;
use crate::core::util::error::Error;
use crate::core::lexer::lexer::Lexer;

#[derive(ClapParser)]
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
    Write {
        filepath: String,
        extension: String,
        content: String,
    },
    Size {
        filepath: String,
    },
    Repl {
        #[arg(short, long)]
        debug: bool,
    },
    Token {
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
    Ir {
        filepath: String,
        #[arg(short, long)]
        debug: bool,
        #[arg(short, long)]
        out: bool
    }
}

pub fn handle(cli: Cli) {
    match cli.command {
        Command::Print { filepath, numbered } => print(filepath, numbered),
        Command::Write { filepath, content, extension } => _ = write_to_file(filepath, extension, content),
        Command::Size { filepath } => size(filepath),
        Command::Repl { debug: _debug } => repl(_debug),
        Command::Token { filepath } => _ = tokenize(filepath, true),
        Command::Parse { filepath, debug: _debug } => _ = parse(filepath, _debug, true),
        Command::Convert { filepath, debug: _debug } => _ = convert(filepath, _debug, true),
        Command::Ir { filepath, debug: _debug, out } => _ = codegen(filepath, _debug, out),
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

pub fn write_to_file(filename: String, extension: String, content: String) -> Result<()> {
    let full_name = format!("{}.{}", filename, extension);
    let mut file = File::create(full_name)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}


pub fn split_filename(path: &str) -> (String, String) {
    let p = Path::new(path);

    let filename = p.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string();

    let extension = p.extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string();

    (filename, extension)
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

    let mut lexer: Lexer = Lexer::new(String::new());
    let mut parser: Parser;
    let mut tree: MTree;

    loop {
        print!("ohl >>> ");
        let _ = std::io::stdout().flush();
        std::io::stdin().read_line(&mut input).expect("Failed to read line.");

        lexer.set_input(input.clone());

        if _debug {
            lexer.print_tokens();
            lexer.reset();
        }

        parser = Parser::new(lexer.clone(), _debug);
        tree = parser.analyze();
        if _debug {
            println!("\n\nParse Tree:\n");
            tree.print();
            println!();
        }

        if input.is_empty() {
            break;
        }

        println!("{}", input);
    }
}

pub fn tokenize(path: String, _debug: bool) -> Lexer {
    validate_ohl_file(path.clone());
    let contents = std::fs::read_to_string(path).unwrap();
    let mut lexer = Lexer::new(contents);
    if _debug {
        lexer.print_tokens();
        lexer.reset();
    }

    lexer
}

pub fn parse(path: String, _debug: bool, print_tree: bool) -> MTree {
    let lexer = tokenize(path, _debug);
    let mut parser = Parser::new(lexer, _debug);
    let tree = parser.analyze();
    if print_tree {
        println!("\n\nParse Tree:\n");
        tree.print();
        println!();
    }

    tree
}

pub fn convert(path: String, _debug: bool, print_tree: bool) -> STree {
    let mtree = parse(path, _debug, _debug);
    
    let mut converter: Converter = Converter::new(_debug);
    let stree = match converter.convert_tree(&mtree) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}: Semantic Conversion Failed \n{}\n", "ERROR".red(), e.red());
            std::process::exit(0)
        }
    };

    if print_tree {
        println!("\n\nSemantic Tree:\n{:#?}\n", stree);
    }

    stree
}

pub fn codegen(path: String, _debug: bool, out: bool) -> Result<String> {
    let stree = convert(path.clone(), _debug, _debug);

    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "ohl", _debug);
    
    match codegen.compile(&stree) {
        Ok(_) => println!("Compilation Complete"),
        Err(e) => println!("Compilation Error: {:?}", e)
    }

    let content = codegen.print_ir();

    if _debug {
        println!("{:?}, ", content);
    }

    if out {
        let (name, _) = split_filename(&path);
        write_to_file(name, "ll".to_string(), content.clone())?;
    }

    Ok(content)
}
