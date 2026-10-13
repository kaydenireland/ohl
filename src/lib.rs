pub mod ohl;
pub mod eos;
pub mod vertex;
pub mod util;


use std::{fs, io};
use std::io::Write;
use std::path::Path;
use colored::Colorize;
use std::process::{Command as TerminalCommand, ExitStatus};

pub use util::logger::LOGGER;
use crate::ohl::analyzer::analyzer::Analyzer;
use vertex::codegen::codegen::AssemblyGenerator;
use vertex::codegen::nasm::x64::X64CodeGenerator;
use eos::intermediate::program::IntermediateProgram;
use crate::ohl::converter::converter::Converter as OhlConverter;
use crate::ohl::converter::stree::STree;
use crate::ohl::parser::mtree::MTree;
use crate::ohl::parser::parser::Parser as OhlParser;
use crate::ohl::lexer::lexer::Lexer as OhlLexer;
use crate::eos::lexer::lexer::Lexer as HeliosLexer;
use crate::eos::parser::parser::Parser as HeliosParser;
use eos::machine::program::MachineProgram;
use util::error::error::OhlError;
use util::error::diagnostics::Diagnostics;
use crate::util::target::Target;

// Ohl Specific

pub fn tokenize_ohl(src: String, _debug: bool) -> OhlLexer {
    // expect source input to already be validated
    let mut lexer = OhlLexer::new(src);
    if _debug {
        lexer.print_tokens();
        lexer.reset();
    }

    lexer
}

pub fn parse_ohl(src: String, _debug: bool) -> MTree {
    let lexer = tokenize_ohl(src, _debug);
    let mut parser = OhlParser::new(lexer, _debug);
    let tree = parser.analyze();

    tree
}

pub fn convert_ohl(src: String, _debug: bool) -> Result<STree, OhlError> {
    let mtree = parse_ohl(src, _debug);

    let mut converter: OhlConverter = OhlConverter::new(_debug);
    match converter.convert_tree(&mtree) {
        Ok(s) => Ok(s),
        Err(e) => Err(OhlError::from(e))
    }
}

pub fn analyze_ohl(src: String, _debug: bool) -> Result<(STree, Vec<String>), Diagnostics> {
    let mut analyzer = Analyzer::new(_debug);
    let stree = match convert_ohl(src, _debug) {
        Ok(s) => s,
        Err(e) => return Err(Diagnostics { warnings: Vec::new(), errors: Vec::new() })
        // TODO: Temporary Error Handling, OhlError needs rewrite, maybe with ErrorType enum field
    };

    match analyzer.analyze(stree.clone()) {
        Ok(warnings) => Ok((stree, warnings)),
        Err(diag) => Err(diag),
    }
}

pub fn lower_ohl_to_helios(stree: STree, _debug: bool) -> IntermediateProgram {
    IntermediateProgram::lower_tree(stree, _debug)
}

// Oil Specific

pub fn tokenize_helios(src: String, _debug: bool) -> HeliosLexer {
    // Assume source is already validated
    let mut lexer = HeliosLexer::new(src);
    if _debug {
        lexer.print_tokens();
        lexer.reset();
    }

    lexer
}

pub fn parse_helios(filepath: String, _debug: bool, print_instructions: bool) -> IntermediateProgram {
    // expect source input to already be validated
    let lexer = tokenize_helios(filepath, _debug);
    let mut parser = HeliosParser::new(lexer, _debug);

    let program = parser.analyze().unwrap();

    if print_instructions {
        program.dump();
    }

    program
}


// Generic Pipeline

pub fn lower_to_selene(inter: IntermediateProgram, _debug: bool) -> MachineProgram {
    MachineProgram::lower(inter, _debug)
}

pub fn generate(machine: MachineProgram, _debug: bool, target: Target) -> io::Result<String> {
    let mut codegen = X64CodeGenerator::new(_debug, target);
    let asm: String = codegen.generate(machine)?;

    Ok(asm)
}

pub fn build(filename: String, assembly: String, _debug: bool, asm: bool, target: Target) -> io::Result<String> {

    let asm_path = format!("{}.s", filename);
    let executable_path = target.executable_extension();

    fs::write(&asm_path, assembly)?;


    TerminalCommand::new("gcc")
        .arg(&asm_path)
        .arg("-o")
        .arg(&executable_path)
        .status()?;


    if !asm {
        fs::remove_file(&asm_path)?;
    }

    Ok(executable_path)
}

pub fn run(filename: String, assembly: String, _debug: bool, asm: bool, exe: bool) -> io::Result<ExitStatus> {

    let exe_path = build(filename.clone(), assembly, _debug, asm, Target::host())?;

    let executable = if cfg!(target_os = "windows") {
        format!(".\\{}.exe", filename)
    } else {
        format!("./{}", filename)
    };

    let status = TerminalCommand::new(&executable).status()?;

    if !exe {
        fs::remove_file(&exe_path)?;
    }

    Ok(status)
}

// Utility

pub fn validate_file_extension(path: String, ext: &str) {
    // TODO: Proper Error Handling
    use std::path::Path;

    let p = Path::new(&path);

    if p.is_dir() {
        let mut e = OhlError::new(0, 0, "Expected file, got directory.".to_string());
        e.disable_location();
        e.report();
    }


    match p.extension().and_then(|e| e.to_str()) {
        Some(ext) => {}
        _ => {
            eprintln!(
                "{}: expected an .{} file, got '{}'",
                "Error".red(),
                ext.yellow(),
                path
            );
            std::process::exit(0);
        }
    }
}

pub fn get_validated_file_contents(filepath: String, extension: &str) -> String {
    validate_file_extension(filepath.clone(), extension);
    fs::read_to_string(filepath).unwrap()
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

pub fn get_target(option: Option<String>) -> Target {
    match option {
        Some(val) => Target::parse(&val).unwrap(),
        None => Target::host(),
    }
}