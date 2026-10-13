use std::fs;
use std::process::{Command as TerminalCommand, ExitStatus};
use clap::{Parser as ClapParser, Subcommand};
use colored::Colorize;
use ohl::eos::intermediate::program::IntermediateProgram;
use ohl::eos::machine::program::MachineProgram;
use ohl::util::target::Target;

static EXT: &str = "heli";

fn main() {
    let args: Cli = Cli::parse();
    handle(args);
}


#[derive(ClapParser)]
#[command(name = "eos", version)]
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
    Selene {
        filepath: String,
        #[arg(short, long)]
        debug: bool,
    },
    Generate {
        filepath: String,
        #[arg(short, long)]
        debug: bool,
        #[arg(long)]
        target: Option<String>
    },
    Build {
        filepath: String,
        #[arg(short, long)]
        debug: bool,
        #[arg(short('s'), long)]
        asm: bool,
        #[arg(long)]
        target: Option<String>
    },
    Run {
        filepath: String,
        #[arg(short, long)]
        debug: bool,
        #[arg(short('s'), long)]
        asm: bool,
        #[arg(short, long)]
        exe: bool
    },

}

pub fn handle(cli: Cli) {
    match cli.command {
        Command::Token { filepath } => _ = {
            let contents = ohl::get_validated_file_contents(filepath, EXT);
            ohl::tokenize_helios(contents, true);
        },
        Command::Parse { filepath, debug: _debug  } => _ = {
            let contents = ohl::get_validated_file_contents(filepath, EXT);
            ohl::parse_helios(contents, _debug, true)
        },
        Command::Selene { filepath, debug: _debug } => {
            let contents = ohl::get_validated_file_contents(filepath, EXT);
            let inter: IntermediateProgram = ohl::parse_helios(contents, _debug, _debug);
            let machine: MachineProgram = ohl::lower_to_selene(inter, _debug);
            machine.dump();
        },
        Command::Generate { filepath, debug: _debug, target } => {
            let contents = ohl::get_validated_file_contents(filepath, EXT);
            let inter: IntermediateProgram = ohl::parse_helios(contents, _debug, _debug);
            let machine: MachineProgram = ohl::lower_to_selene(inter, _debug);
            let trgt = ohl::get_target(target);
            let asm: String = ohl::generate(machine, _debug, trgt).unwrap();
            println!("\n{}", asm);
        },
        Command::Build { filepath, debug: _debug, asm, target } => {

            let (filename, _ext) = ohl::split_filename(&filepath);

            let asm_path = format!("{}.s", filename);
            let executable_path = format!("{}.exe", filename);

            let contents = ohl::get_validated_file_contents(filepath, EXT);
            let inter: IntermediateProgram = ohl::parse_helios(contents, _debug, _debug);
            let machine: MachineProgram = ohl::lower_to_selene(inter, _debug);
            let trgt = ohl::get_target(target);
            let assembly = ohl::generate(machine, _debug, trgt).unwrap();

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

            let contents = ohl::get_validated_file_contents(filepath, EXT);
            let inter: IntermediateProgram = ohl::parse_helios(contents, _debug, _debug);
            let machine: MachineProgram = ohl::lower_to_selene(inter, _debug);
            let assembly = ohl::generate(machine, _debug, Target::host()).unwrap();

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