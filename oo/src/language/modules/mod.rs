use crate::language::analyzing::analyzer::Analyzer;
use crate::language::running::interpreter::Interpreter;

pub mod system;
pub mod math;
pub mod random;

impl Analyzer {
    pub fn register_native_functions(&mut self) {
        self.register_system_functions();
        self.register_math_functions();
        self.register_random_functions();
    }
}

impl Interpreter {

    pub fn register_native_functions(&mut self) {
        self.register_system_functions();
        self.register_math_functions();
        self.register_random_functions();
    }
}