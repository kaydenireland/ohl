use std::io::Error;
use crate::ohl::lowering::program::MachineProgram;

pub trait AssemblyGenerator {
    fn generate(&mut self, program: MachineProgram) -> Result<String, Error>;
}