use std::io::Error;
use crate::core::lowering::program::MachineProgram;

pub trait AssemblyGenerator {
    fn generate(&mut self, program: MachineProgram) -> Result<(), Error>;
}