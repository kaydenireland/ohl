use std::fmt::Display;
use crate::util::target::Architecture::X86_64;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Architecture {
    X86_64
}

impl Display for Architecture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "x86_64")
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum OS {
    WINDOWS,
    LINUX
}

impl Display for OS {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            OS::WINDOWS => write!(f, "windows"),
            OS::LINUX => write!(f, "linux"),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Syntax {
    NASM,
    MASM
}

impl Syntax {
    pub fn from_string(value: Option<&str>) -> Syntax {

        let result: &str = match value {
            Some(value) => value,
            None => return Syntax::NASM
        };

        match result {
            "nasm" => Syntax::NASM,
            "att" => Syntax::NASM,

            "masm" => Syntax::MASM,
            "intel" => Syntax::MASM,

            _ => panic!("Unknown Syntax: {}", result)
        }
    }
}

pub struct Target {
    pub architecture: Architecture,
    pub os: OS,
}

impl Target {
    pub fn new(architecture: Architecture, os: OS) -> Target {
        Target { architecture, os }
    }

    pub fn host() -> Target {
        let architecture = if cfg!(target_arch = "x86_64") {
            X86_64
        } else {
            panic!("Unsupported host architecture");
        };

        let os = if cfg!(target_os = "windows") {
            OS::WINDOWS
        } else if cfg!(target_os = "linux") {
            OS::LINUX
        } else {
            panic!("Unsupported host operating system");
        };

        Target { architecture, os }
    }

    pub fn parse(value: &str) -> Result<Self, String> {
        let mut parts = value.split('-');

        let architecture = match parts.next() {
            Some("x86_64") => X86_64,
            Some("x64") => X86_64,
            Some(arch) => return Err(format!("Unknown architecture: {arch}")),
            None => return Err("Missing architecture".to_string()),
        };

        let os = match parts.next() {
            Some("windows") => OS::WINDOWS,
            Some("linux") => OS::LINUX,
            Some(os) => return Err(format!("Unknown operating system: {os}")),
            None => return Err("Missing operating system".to_string()),
        };

        if parts.next().is_some() {
            return Err(format!("Invalid target: {value}"));
        }

        Ok(Target { architecture, os })
    }

    pub fn executable_extension(&self) -> String {
        match self.os {
            OS::WINDOWS => ".exe".to_string(),
            OS::LINUX => "".to_string(),
        }
    }
}

impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}-{}", self.architecture, self.os)
    }
}
