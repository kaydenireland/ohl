use crate::core::analyzing::analyzer::{Analyzer, FunctionSignature};
use crate::core::analyzing::types::VariableType;
use crate::core::running::interpreter::{Interpreter, RuntimeFunction};
use crate::core::running::value::Value;


impl Analyzer {
    pub fn register_math_functions(&mut self) {
        self.functions.insert(
            vec!["Math".to_string(), "abs".to_string()],
            FunctionSignature {
                parameters: vec![],              
                return_type: VariableType::NULL, 
                called: true
            },
        );

        self.functions.insert(
            vec!["Math".to_string(), "factorial".to_string()],
            FunctionSignature {
                parameters: vec![VariableType::INT],              
                return_type: VariableType::INT, 
                called: true
            },
        );

        self.functions.insert(
            vec!["Math".to_string(), "signum".to_string()],
            FunctionSignature {
                parameters: vec![],              
                return_type: VariableType::INT, 
                called: true
            },
        );
    }
}

impl Interpreter {

    pub fn register_math_functions(&mut self) {
        self.functions.insert(
            vec!["Math".to_string(), "abs".to_string()],
            RuntimeFunction::Native(Self::abs),
        );

        self.functions.insert(
            vec!["Math".to_string(), "factorial".to_string()],
            RuntimeFunction::Native(Self::factorial),
        );

        self.functions.insert(
            vec!["Math".to_string(), "signum".to_string()],
            RuntimeFunction::Native(Self::signum),
        );
    }
}

impl Interpreter {

    fn abs(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("Math.abs expects exactly one argument".to_string());
        }

        match &args[0] {
            Value::INT(i) => Ok(Value::INT(i.abs())),
            Value::FLOAT(f) => Ok(Value::FLOAT(f.abs())),
            _ => Err("Math.abs expects a numeric type".to_string())
        }

    }

    fn factorial(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("Math.factorial expects exactly one argument".to_string());
        }

        let num = match args[0] {
            Value::INT(i) => i,
            _ => return Err("Math.factorial expects an int".to_string()),
        };

        let mut result = 1;

        if num < 0 {
            return Err("Math.factorial is undefined for negative numbers".to_string());
        } else if num > 0 {
            result = (1..=num).product();
        }

        
        Ok(Value::INT(result))

    }

    fn signum(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("Math.signum expects exactly one argument".to_string());
        }

        match &args[0] {
            Value::INT(i) => {
                Ok(Value::INT(
                    if *i > 0 { 1 }
                    else if *i < 0 { -1 }
                    else { 0 }
                ))
            }

            Value::FLOAT(f) => {
                Ok(Value::INT(
                    if *f > 0.0 { 1 }
                    else if *f < 0.0 { -1 }
                    else { 0 }
                ))
            }

            _ => Err("Math.signum expects a numeric type".to_string()),
        }
    }

}