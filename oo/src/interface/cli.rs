use std::process::exit;

use clap::{Parser, Subcommand};
use colored::Colorize;

use crate::language::analyzing::analyzer::Analyzer;
use crate::language::analyzing::folder::ConstantFolder;
use crate::language::running::interpreter::Interpreter;
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
    Analyze {
        filepath: String,
        #[arg(short, long)]
        debug: bool,
    },
    Run {
        filepath: String,
        #[arg(short, long)]
        debug: bool,
        #[arg(short, long)]
        warnings: bool,
        #[arg(short, long)]
        time: bool
    },
}

pub fn handle(cli: Cli) {
    match cli.command {
        Command::Print { filepath, numbered } => print(filepath, numbered),
        Command::Size { filepath } => size(filepath),
        Command::Tokenize { filepath } => tokenize(filepath),
        Command::Parse { filepath, debug: _debug } => _ = parse(filepath, _debug, true),
        Command::Convert { filepath, debug: _debug } => _ = convert(filepath, _debug, true),
        Command::Analyze { filepath, debug: _debug } => _ = analyze(filepath, _debug, true),
        Command::Run { filepath, debug: _debug, warnings, time: _time } => run(filepath, _debug, warnings, _time),
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


pub fn convert(path: String, _debug: bool, print_tree: bool) -> STree {
    let mtree: MTree = parse(path, _debug, _debug);
    let mut converter: Converter = Converter::new(_debug);
    let result: Result<STree, String> = converter.convert_tree(&mtree);
    let stree = match result {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}: Semantic Conversion Failed \n{}/n", "ERROR".red(), e.red());
            std::process::exit(0)
        }
    };

    if print_tree {
        println!("\n\nSemantic Tree:\n{:#?}", stree);
    }

    stree
}

pub fn analyze(path: String, _debug: bool, show: bool) -> STree {
    let mut stree: STree = convert(path, _debug, _debug);

    let mut folder: ConstantFolder = ConstantFolder::new(_debug);
    folder.run(&mut stree);

    let analyzer = Analyzer::new(_debug);
    let result = analyzer.analyze(&stree);
    match result {
        Ok(warnings) => {
            if !warnings.is_empty() && show {
                println!("Analysis completed with {} {}:", warnings.len().to_string().yellow(), "warnings(s)".yellow());
                for (i, warning) in warnings.iter().enumerate() {
                    println!("  {}. {}", i + 1, warning);
                }
            }
        },
        Err(errors) => {
            println!("Analysis completed with {} {}:", errors.len().to_string().yellow(), "error(s)".red());
            for (i, error) in errors.iter().enumerate() {
                println!("  {}. {}", i + 1, error);
            }
            std::process::exit(0);
        }
    };
    
    stree
}

pub fn run(path: String, _debug: bool, hide_warnings: bool, _time: bool) {
    use std::time::Instant;

    let mut stree = analyze(path.clone(), _debug, !hide_warnings);
    let mut folder: ConstantFolder = ConstantFolder::new(_debug);
    folder.run(&mut stree);

    println!("\n{} {}\n", "Running".to_string().green(), &path.white());

    let mut interpreter = Interpreter::new();

    let start = if _time {
        Some(Instant::now())
    } else {
        None
    };


    let result = interpreter.execute(stree);

    match result {
        Ok(_) => {}
        Err(err) => {
            println!("{}: {}", "\nRuntime Error".red(), err);
            std::process::exit(0);
        }
    }

    if let Some(start) = start {
        let elapsed = start.elapsed();
        println!(
            "\n{} execution in {:.6}s",
            "Completed".green(),
            elapsed.as_secs_f64().to_string().cyan()
        );
    }
}
