use crate::core::analyzing::analyzer::{Analyzer, FunctionSignature};
use crate::core::analyzing::types::VariableType;
use crate::core::running::interpreter::{Interpreter, RuntimeFunction};
use crate::core::running::value::Value;

use std::io::{self, Write};


impl Analyzer {
    pub fn register_system_functions(&mut self) {
        self.functions.insert(
            vec!["System".to_string(), "print".to_string()],
            FunctionSignature {
                parameters: vec![],              // variadic (checked loosely)
                return_type: VariableType::NULL, // print returns null
                called: true,                    // never warn as unused
            },
        );

        self.functions.insert(
            vec!["System".to_string(), "println".to_string()],
            FunctionSignature {
                parameters: vec![],
                return_type: VariableType::NULL,
                called: true,
            },
        );

        self.functions.insert(
            vec!["System".to_string(), "input".to_string()],
            FunctionSignature {
                parameters: vec![],
                return_type: VariableType::STRING,
                called: true,
            },
        );

        self.functions.insert(
            vec!["System".to_string(), "exit".to_string()],
            FunctionSignature {
                parameters: vec![VariableType::INT],
                return_type: VariableType::NULL,
                called: true,
            },
        );

        self.functions.insert(
            vec!["System".to_string(), "flush".to_string()],
            FunctionSignature {
                parameters: vec![],
                return_type: VariableType::NULL,
                called: true,
            },
        );

        self.functions.insert(
            vec!["System".to_string(), "clear".to_string()],
            FunctionSignature {
                parameters: vec![],
                return_type: VariableType::NULL,
                called: true,
            },
        );

    }
}

impl Interpreter {

    pub fn register_system_functions(&mut self) {
        self.functions.insert(
            vec!["System".to_string(), "print".to_string()],
            RuntimeFunction::Native(Self::print),
        );

        self.functions.insert(
            vec!["System".to_string(), "println".to_string()],
            RuntimeFunction::Native(Self::println),
        );

        self.functions.insert(
            vec!["System".to_string(), "input".to_string()],
            RuntimeFunction::Native(Self::input),
        );

        self.functions.insert(
            vec!["System".to_string(), "exit".to_string()],
            RuntimeFunction::Native(Self::exit),
        );

        self.functions.insert(
            vec!["System".to_string(), "flush".to_string()],
            RuntimeFunction::Native(Self::flush),
        );

        self.functions.insert(
            vec!["System".to_string(), "clear".to_string()],
            RuntimeFunction::Native(Self::clear),
        );
    }
}

impl Interpreter {
    fn print(args: Vec<Value>) -> Result<Value, String> {
        use std::io::{self, Write};

        for (_, arg) in args.iter().enumerate() {
            match arg {
                Value::INT(v) => print!("{}", v),
                Value::FLOAT(v) => print!("{}", v),
                Value::BOOLEAN(v) => print!("{}", v),
                Value::CHAR(v) => print!("{}", v),
                Value::STRING(v) => print!("{}", v),
                Value::NULL => print!("null"),
            }
        }

        io::stdout().flush().map_err(|e| e.to_string())?;
        Ok(Value::NULL)
    }

    fn println(args: Vec<Value>) -> Result<Value, String> {
        println!();
        Self::print(args)
    }

    fn input(args: Vec<Value>) -> Result<Value, String> {
        if !args.is_empty() {
            return Err("System.in takes no arguments".to_string());
        }

        io::stdout().flush().map_err(|e| e.to_string())?;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| e.to_string())?;

        if input.ends_with('\n') {
            input.pop();
            if input.ends_with('\r') {
                input.pop();
            }
        }

        Ok(Value::STRING(input))
    }


    fn exit(args: Vec<Value>) -> Result<Value, String> {
        use colored::Colorize;

        if args.len() != 1 {
            return Err("System.exit expects exactly one argument".to_string());
        }

        let code = match args[0] {
            Value::INT(i) => i,
            _ => return Err("System.exit expects an int".to_string()),
        };

        if code == 0 {
            println!("\nProcess ended with code {}", code.to_string().green());
        } else {
            println!("\nProcess ended with code {}", code.to_string().red());
        }

        std::process::exit(0);
    }

    fn flush(args: Vec<Value>) -> Result<Value, String> {

        if !args.is_empty() {
            return Err("System.flush takes no arguments".to_string());
        }

        io::stdout().flush().map_err(|e| e.to_string())?;
        Ok(Value::NULL)
    }

    fn clear(args: Vec<Value>) -> Result<Value, String> {

        if !args.is_empty() {
            return Err("System.clear takes no arguments".to_string());
        }

        print!("\x1B[2J\x1B[1;1H");
        io::stdout().flush().map_err(|e| e.to_string())?;
        Ok(Value::NULL)
    }


}