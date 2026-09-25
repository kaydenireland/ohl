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
    Analyze {
        filepath: String,
        #[arg(short, long)]
        debug: bool
    },
    Lower {
        filepath: String,
        #[arg(short, long)]
        debug: bool,
    },
    Machine {
        filepath: String,
        #[arg(short, long)]
        debug: bool,
    },
    Generate {
        filepath: String,
        #[arg(short, long)]
        debug: bool,
    },
    Build {
        filepath: String,
        #[arg(short, long)]
        debug: bool,
        #[arg(short('s'), long)]
        asm: bool,
    },
    Run {
        filepath: String,
        #[arg(short, long)]
        debug: bool,
        #[arg(short('s'), long)]
        asm: bool,
        #[arg(short, long)]
        exe: bool,
    },
}

pub fn handle(cli: Cli) {
    match cli.command {
        Command::Print { filepath, numbered } => print_file_contents(filepath, numbered),
        Command::Write { filepath, content, extension } => _ = write_to_file(filepath, extension, content),
        Command::Size { filepath } => size(filepath),
        Command::Token { filepath } => {
            let contents = get_ohl_source(filepath);
            ohl::tokenize(contents, true);
        },
        Command::Parse { filepath, debug: _debug } => {
            let contents = get_ohl_source(filepath);
            let tree = ohl::parse(contents, _debug);
             {
                println!("\n\nParse Tree:\n");
                tree.print(_debug);
                println!();
            }
        },
        Command::Convert { filepath, debug: _debug } => {
            let contents = get_ohl_source(filepath);
            let stree = match ohl::convert(contents, _debug) {
                Ok(stree) => stree,
                Err(e) => {
                    eprintln!("{}: Semantic Conversion Failed \n{}\n", "ERROR".red(), e.to_string());
                    std::process::exit(1);
                }
            };
            println!("\n\nSemantic Tree:\n{:#?}\n", stree);

        },
        Command::Analyze { filepath, debug: _debug } => {
            let _ = analyze_and_print(filepath, _debug);
        },
        Command::Lower { filepath, debug: _debug } => {
            let stree = analyze_and_print(filepath, _debug);
            let mut inter: IntermediateProgram = ohl::lower(stree, _debug);
            inter.dump();
        },
        Command::Machine { filepath, debug: _debug } => {
            let stree = analyze_and_print(filepath, _debug);
            let inter: IntermediateProgram = ohl::lower(stree, _debug);
            let machine: MachineProgram = ohl::machine(inter, _debug);
            machine.dump();
        },
        Command::Generate { filepath, debug: _debug } => {
            let stree = analyze_and_print(filepath, _debug);
            let inter: IntermediateProgram = ohl::lower(stree, _debug);
            let machine: MachineProgram = ohl::machine(inter, _debug);
            let asm: String = ohl::generate(machine, _debug).unwrap();
            println!("\n{}", asm);
        },
        Command::Build { filepath, debug: _debug, asm } => {

            let (filename, _ext) = split_filename(&filepath);

            let asm_path = format!("{}.s", filename);
            let executable_path = format!("{}.exe", filename);

            let stree = analyze_and_print(filepath, _debug);
            let inter: IntermediateProgram = ohl::lower(stree, _debug);
            let machine: MachineProgram = ohl::machine(inter, _debug);
            let assembly = ohl::generate(machine, _debug).unwrap();

            fs::write(&asm_path, assembly).unwrap();


            let status: ExitStatus = TerminalCommand::new("gcc")
                .arg(&asm_path)
                .arg("-o")
                .arg(&executable_path)
                .status().unwrap();


            if !status.success() {
                eprintln!("GCC failed to build the program.");
                std::process::exit(1);
            }

            if !asm {
                let _ = fs::remove_file(&asm_path);
            }

            println!("Built {}", executable_path);
        },
        Command::Run { filepath, debug: _debug, asm, exe } => {
            let (filename, _ext) = split_filename(&filepath);

            let stree = analyze_and_print(filepath, _debug);
            let inter: IntermediateProgram = ohl::lower(stree, _debug);
            let machine: MachineProgram = ohl::machine(inter, _debug);
            let assembly = ohl::generate(machine, _debug).unwrap();

            let status = ohl::run(filename, assembly, _debug, asm, exe).unwrap();

            if !status.success() {
                eprintln!("Program exited with status: {}", status.to_string().red());
                std::process::exit(0);
            } else {
                println!("Program exited successfully.");
            }
        }
    }
}


// Utility Functions

pub fn print_file_contents(path: String, numbered: bool) {
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

pub fn write_to_file(filename: String, extension: String, content: String) -> std::io::Result<()> {
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

fn print_vec_string(strings: Vec<String>) {
    println!();
    for msg in strings {
        println!("{}", msg);
    }
}

fn get_ohl_source(filepath: String) -> String {
    ohl::validate_file_extension(filepath.clone(), "ohl".to_string());
    fs::read_to_string(filepath).unwrap()
}

fn analyze_and_print(filepath: String, _debug: bool) -> STree {
    let contents = get_ohl_source(filepath);
    match ohl::analyze(contents, _debug) {
        Ok((stree, warnings)) => {
            print_vec_string(warnings.clone());
            println!(
                "\nAnalysis complete with {} {}",
                warnings.len(),
                "warning(s)".yellow()
            );
            stree
        },
        Err(diagnostics) => {
            print_vec_string(diagnostics.warnings.clone());
            print_vec_string(diagnostics.errors.clone());
            eprintln!(
                "\nAnalysis complete with {} {} and {} {}",
                diagnostics.warnings.len(),
                "warning(s)".yellow(),
                diagnostics.errors.len(),
                "error(s)".red()
            );
            std::process::exit(0);
        }
    }
}
