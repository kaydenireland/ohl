use std::fs;
use std::process::{Command as TerminalCommand, ExitStatus};
use clap::{Parser as ClapParser, Subcommand};
use colored::Colorize;
use ohl::oil::intermediate::program::IntermediateProgram;
use ohl::oil::lexer::lexer::Lexer;
use ohl::oil::machine::program::MachineProgram;
use ohl::oil::parser::parser::Parser;

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
    Parse {
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
        Command::Token { filepath } => _ = tokenize(filepath, true),
        Command::Parse { filepath, debug: _debug  } => _ = parse(filepath, _debug, true),
        Command::Machine { filepath, debug: _debug } => {
            let inter: IntermediateProgram = parse(filepath, _debug, _debug);
            let machine: MachineProgram = ohl::machine(inter, _debug);
            machine.dump();
        },
        Command::Generate { filepath, debug: _debug } => {
            let inter: IntermediateProgram = parse(filepath, _debug, _debug);
            let machine: MachineProgram = ohl::machine(inter, _debug);
            let asm: String = ohl::generate(machine, _debug).unwrap();
            println!("\n{}", asm);
        },
        Command::Build { filepath, debug: _debug, asm } => {

            let (filename, _ext) = ohl::split_filename(&filepath);

            let asm_path = format!("{}.s", filename);
            let executable_path = format!("{}.exe", filename);

            let inter: IntermediateProgram = parse(filepath, _debug, _debug);
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
            let (filename, _ext) = ohl::split_filename(&filepath);

            let inter: IntermediateProgram = parse(filepath, _debug, _debug);
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

pub fn tokenize(filepath: String, _debug: bool) -> Lexer {
    let src = get_oil_source(filepath);
    let mut lexer = Lexer::new(src);
    if _debug {
        lexer.print_tokens();
        lexer.reset();
    }

    lexer
}

pub fn parse(filepath: String, _debug: bool, print_instructions: bool) -> IntermediateProgram {
    // expect source input to already be validated
    let lexer = tokenize(filepath, _debug);
    let mut parser = Parser::new(lexer, _debug);

    let program = parser.analyze().unwrap();

    if print_instructions {
        program.dump();
    }

    program
}

// Utility
fn get_oil_source(filepath: String) -> String {
    ohl::validate_file_extension(filepath.clone(), "oil".to_string());
    fs::read_to_string(filepath).unwrap()
}