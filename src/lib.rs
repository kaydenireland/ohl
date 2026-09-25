pub mod ohl;
pub mod oil;
pub mod util;


use std::{fs, io};
use std::io::Write;
use colored::Colorize;
use std::process::{Command as TerminalCommand, ExitStatus};

pub use util::logger::LOGGER;
use crate::ohl::analyzer::analyzer::Analyzer;
use oil::codegen::codegen::AssemblyGenerator;
use oil::codegen::nasn::x64::X64CodeGenerator;
use oil::intermediate::program::IntermediateProgram;
use crate::ohl::converter::converter::Converter;
use crate::ohl::converter::stree::STree;
use crate::ohl::parser::mtree::MTree;
use crate::ohl::parser::parser::Parser;
use crate::ohl::lexer::lexer::Lexer;
use oil::machine::program::MachineProgram;
use util::error::error::OhlError;
use util::error::diagnostics::Diagnostics;

pub fn tokenize(src: String, _debug: bool) -> Lexer {
    // expect source input to already be validated
    let mut lexer = Lexer::new(src);
    if _debug {
        lexer.print_tokens();
        lexer.reset();
    }

    lexer
}

pub fn parse(src: String, _debug: bool) -> MTree {
    let lexer = tokenize(src, _debug);
    let mut parser = Parser::new(lexer, _debug);
    let tree = parser.analyze();

    tree
}

pub fn convert(src: String, _debug: bool) -> Result<STree, OhlError> {
    let mtree = parse(src, _debug);

    let mut converter: Converter = Converter::new(_debug);
    match converter.convert_tree(&mtree) {
        Ok(s) => Ok(s),
        Err(e) => Err(OhlError::from(e))
    }
}

pub fn analyze(src: String, _debug: bool) -> Result<(STree, Vec<String>), Diagnostics> {
    let mut analyzer = Analyzer::new(_debug);
    let stree = match convert(src, _debug) {
        Ok(s) => s,
        Err(e) => return Err(Diagnostics { warnings: Vec::new(), errors: Vec::new() })
        // TODO: Temporary Error Handling, OhlError needs rewrite, maybe with ErrorType enum field
    };

    match analyzer.analyze(stree.clone()) {
        Ok(warnings) => Ok((stree, warnings)),
        Err(diag) => Err(diag),
    }
}

pub fn lower(stree: STree, _debug: bool) -> IntermediateProgram {
    IntermediateProgram::lower_tree(stree, _debug)
}

pub fn machine(inter: IntermediateProgram, _debug: bool) -> MachineProgram {
    MachineProgram::lower(inter, _debug)
}

pub fn generate(machine: MachineProgram, _debug: bool) -> io::Result<String> {
    let mut codegen = X64CodeGenerator::new(_debug);
    let asm: String =codegen.generate(machine)?;

    Ok(asm)
}

pub fn build(filename: String, assembly: String, _debug: bool, asm: bool) -> io::Result<String> {

    let asm_path = format!("{}.s", filename);
    let executable_path = format!("{}.exe", filename);

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

    let exe_path = build(filename.clone(), assembly, _debug, asm)?;

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

pub fn validate_file_extension(path: String, ext: String) {
    // TODO: Propoer Error Handling
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