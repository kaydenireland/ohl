use crate::core::analyzing::analyzer::{Analyzer, FunctionSignature};
use crate::core::analyzing::types::VariableType;
use crate::core::running::interpreter::{Interpreter, RuntimeFunction};
use crate::core::running::value::Value;

use rand::Rng;

impl Analyzer {
    pub fn register_random_functions(&mut self) {
        self.functions.insert(
            vec!["Random".to_string(), "nextInt".to_string()],
            FunctionSignature {
                parameters: vec![VariableType::INT, VariableType::INT],              
                return_type: VariableType::INT, 
                called: true
            },
        );

        self.functions.insert(
            vec!["Random".to_string(), "nextFloat".to_string()],
            FunctionSignature {
                parameters: vec![VariableType::FLOAT, VariableType::FLOAT],              
                return_type: VariableType::FLOAT, 
                called: true
            },
        );

        self.functions.insert(
            vec!["Random".to_string(), "nextChar".to_string()],
            FunctionSignature {
                parameters: vec![VariableType::CHAR, VariableType::CHAR],              
                return_type: VariableType::CHAR, 
                called: true
            },
        );

        self.functions.insert(
            vec!["Random".to_string(), "anyInt".to_string()],
            FunctionSignature {
                parameters: vec![],              
                return_type: VariableType::INT, 
                called: true
            },
        );

        self.functions.insert(
            vec!["Random".to_string(), "anyFloat".to_string()],
            FunctionSignature {
                parameters: vec![],              
                return_type: VariableType::FLOAT, 
                called: true
            },
        );

        self.functions.insert(
            vec!["Random".to_string(), "anyChar".to_string()],
            FunctionSignature {
                parameters: vec![],              
                return_type: VariableType::CHAR, 
                called: true
            },
        );

        self.functions.insert(
            vec!["Random".to_string(), "anyBoolean".to_string()],
            FunctionSignature {
                parameters: vec![],              
                return_type: VariableType::BOOLEAN, 
                called: true
            },
        );

        
    }

}

impl Interpreter {

    pub fn register_random_functions(&mut self) {
        self.functions.insert(
            vec!["Random".to_string(), "nextInt".to_string()],
            RuntimeFunction::Native(Self::next_int),
        );

        self.functions.insert(
            vec!["Random".to_string(), "nextFloat".to_string()],
            RuntimeFunction::Native(Self::next_float),
        );

        self.functions.insert(
            vec!["Random".to_string(), "nextChar".to_string()],
            RuntimeFunction::Native(Self::next_char),
        );

        self.functions.insert(
            vec!["Random".to_string(), "anyInt".to_string()],
            RuntimeFunction::Native(Self::any_int),
        );

        self.functions.insert(
            vec!["Random".to_string(), "anyFloat".to_string()],
            RuntimeFunction::Native(Self::any_float),
        );

        self.functions.insert(
            vec!["Random".to_string(), "anyChar".to_string()],
            RuntimeFunction::Native(Self::any_char),
        );

        self.functions.insert(
            vec!["Random".to_string(), "anyBoolean".to_string()],
            RuntimeFunction::Native(Self::any_boolean),
        );
    }
}

impl Interpreter {

    fn next_int(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 2 {
            return Err("Random.nextInt expects exactly two arguments".to_string());
        }

        let start: i32 = match &args[0] {
            Value::INT(i) => *i,
            Value::FLOAT(f) => *f as i32,
            _ => return Err("Random.nextInt expects numeric types".to_string())
        };

        let end: i32 = match &args[1] {
            Value::INT(i) => *i,
            Value::FLOAT(f) => *f as i32,
            _ => return Err("Random.nextInt expects numeric types".to_string())
        };

        let mut rng = rand::rng();
        let rand = rng.random_range(start..=end);

        Ok(Value::INT(rand))
    }

    fn next_float(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 2 {
            return Err("Random.nextFloat expects exactly two arguments".to_string());
        }

        let start: f32 = match &args[0] {
            Value::INT(i) => *i as f32,
            Value::FLOAT(f) => *f,
            _ => return Err("Random.nextFloat expects numeric types".to_string())
        };

        let end: f32 = match &args[1] {
            Value::INT(i) => *i as f32,
            Value::FLOAT(f) => *f,
            _ => return Err("Random.nextFloat expects numeric types".to_string())
        };

        let mut rng = rand::rng();
        let rand = rng.random_range(start..=end);

        Ok(Value::FLOAT(rand))
    }

    fn next_char(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 2 {
            return Err("Random.nextChar expects exactly two arguments".to_string());
        }

        let start: char = match &args[0] {
            Value::CHAR(c) => *c,
            _ => return Err("Random.nextChar expects chars".to_string())
        };

        let end: char = match &args[1] {
            Value::CHAR(c) => *c,
            _ => return Err("Random.nextChar expects chars".to_string())
        };

        let mut rng = rand::rng();
        let rand = rng.random_range(start..=end);

        Ok(Value::CHAR(rand))
    }

    fn any_int(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 0 {
            return Err("Random.anyInt expects no arguments".to_string());
        }

        let mut rng = rand::rng();
        let rand: i32 = rng.random();

        Ok(Value::INT(rand))
    }

    fn any_float(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 0 {
            return Err("Random.anyFloat expects no arguments".to_string());
        }

        let mut rng = rand::rng();
        let rand: f32 = rng.random();

        Ok(Value::FLOAT(rand))
    }

    fn any_char(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 0 {
            return Err("Random.anyChar expects no arguments".to_string());
        }

        let mut rng = rand::rng();
        let rand: char = rng.random();

        Ok(Value::CHAR(rand))
    }

    fn any_boolean(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 0 {
            return Err("Random.anyBoolean expects no arguments".to_string());
        }

        let mut rng = rand::rng();
        let rand: bool = rng.random();

        Ok(Value::BOOLEAN(rand))
    }

}