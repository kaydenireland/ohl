use std::io::Error;
use crate::oil::machine::program::MachineProgram;

pub trait AssemblyGenerator {
    fn generate(&mut self, program: MachineProgram) -> Result<String, Error>;
}