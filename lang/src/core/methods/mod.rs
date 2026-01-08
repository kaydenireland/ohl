use crate::core::analyzing::analyzer::Analyzer;
use crate::core::running::interpreter::Interpreter;

pub mod system;
pub mod math;

impl Analyzer {
    pub fn register_native_functions(&mut self) {
        self.register_string_methods();
    }
}

impl Interpreter {

    pub fn register_native_functions(&mut self) {
        self.register_string_methods();
    }
}