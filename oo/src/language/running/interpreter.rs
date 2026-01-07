#![warn(non_camel_case_types)]

use std::{clone, collections::HashMap};


use crate::language::analyzing::{operator::Operator, stree::STree, types::{FunctionType, VariableType}};
use crate::language::running::{environment::Environment, value::Value};



enum ControlFlow {
    NORMAL,
    RETURN(Value),
    BREAK,
    CONTINUE,
    REPEAT
}
// TODO: EXIT ControlFlow

#[derive(Clone)]
pub struct Function {
    pub function_type: FunctionType,
    pub return_type: VariableType,
    pub name: String,
    pub params: Vec<(String, VariableType)>,
    pub body: Box<STree>,
}

#[derive(Clone)]
pub enum RuntimeFunction {
    User(Function),
    Native(fn(Vec<Value>) -> Result<Value, String>),
}

// TODO: Error Enum

pub struct Interpreter {
    env: Environment,
    pub functions: HashMap<Vec<String>, RuntimeFunction>,
    call_depth: usize
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut interpreter = Interpreter {
            env: Environment::new(),
            functions: HashMap::new(),
            call_depth: 0
        };

        interpreter.register_native_functions();
        interpreter
    }

    pub fn execute(&mut self, tree: STree) -> Result<(), String> {
        if let STree::START { functions } = &tree {
            for function in functions {
                if let STree::FUNCTION {
                    function_type,
                    return_type,
                    name,
                    params,
                    body,
                } = function
                {
                    self.functions.insert(
                    vec![name.clone()],
                    RuntimeFunction::User(Function {
                        function_type: function_type.clone(),
                        return_type: return_type.clone(),
                        name: name.clone(),
                        params: params.clone(),
                        body: body.clone(),
                    }),
                );
                }
            }
        }

        self.call_function(&["main".to_string()], vec![])?;
        Ok(())
    }


    fn call_function(&mut self, path: &[String], args: Vec<Value>) -> Result<Value, String> {
        self.call_depth += 1;
        if self.call_depth > 50 {
            self.call_depth -= 1;
            return Err("Maximum recursion depth exceeded".to_string());
        }

        let result = (|| {
            let func = self.functions
                .get(path)
                .ok_or_else(|| format!("Function '{}' not found", path.join(".")))?
                .clone();


            match func {
                RuntimeFunction::Native(native) => {
                    native(args)
                }

                RuntimeFunction::User(func) => {
                    if func.params.len() != args.len() {
                        return Err(format!(
                            "Function '{}' expects {} arguments, got {}",
                            path.join("."),
                            func.params.len(),
                            args.len()
                        ));
                    }

                    self.env.push_scope();

                    for ((param_name, _), arg_value) in func.params.iter().zip(args.into_iter()) {
                        self.env.declare(param_name.clone(), arg_value, false);
                    }

                    let value = match self.execute_block(&func.body.clone())? {
                        ControlFlow::RETURN(v) => v,
                        ControlFlow::NORMAL => Value::NULL,
                        ControlFlow::BREAK
                        | ControlFlow::CONTINUE
                        | ControlFlow::REPEAT => {
                            return Err("Loop control used outside of loop".to_string());
                        }
                    };

                    self.pop_scope()?;
                    Ok(value)
                }
            }
        })();

        self.call_depth -= 1;
        result
    }



}

impl Interpreter {
    fn execute_block(&mut self, block: &STree) -> Result<ControlFlow, String> {
        if let STree::BLOCK { statements } = block {
            for stmt in statements {
                match self.execute_statement(stmt)? {
                    ControlFlow::NORMAL => {}
                    flow => return Ok(flow),
                }
            }
            Ok(ControlFlow::NORMAL)
        } else {
            Err("Expected STree::BLOCK".to_string())
        }
    }

    fn execute_statement(&mut self, stmt: &STree) -> Result<ControlFlow, String> {
        match stmt {

            // Variable declaration 
            STree::LET_STMT { id, var_type: _, mutable, expression } => {
                let value = self.evaluate_expression(expression)?;

                // Variables always mutable
                self.env.declare(id.clone(), value, *mutable);
                Ok(ControlFlow::NORMAL)
            }

            // Assignment 
            STree::ASSIGN_STMT { id, expression } => {
                let value = self.evaluate_expression(expression)?;
                self.env.set(id, value)?;
                Ok(ControlFlow::NORMAL)
            }

            // Return 
            STree::RETURN_STMT { expression } => {
                let value = if let Some(expr) = expression {
                    self.evaluate_expression(expr)?
                } else {
                    Value::NULL
                };
                Ok(ControlFlow::RETURN(value))
            }

            // Loop control
            STree::BREAK => Ok(ControlFlow::BREAK),
            STree::CONTINUE => Ok(ControlFlow::CONTINUE),
            STree::REPEAT => Ok(ControlFlow::REPEAT),

            // If
            STree::IF_EXPR { condition, then_block, else_block } => {
                let cond = self.evaluate_expression(condition)?.is_truthy();

                if cond {
                    self.env.push_scope();
                    let flow = self.execute_block(then_block)?;
                    self.pop_scope()?;
                    Ok(flow)
                } else if let Some(else_node) = else_block {
                    match else_node.as_ref() {
                        STree::BLOCK { .. } => {
                            self.env.push_scope();
                            let flow = self.execute_block(else_node)?;
                            self.pop_scope()?;
                            Ok(flow)
                        }
                        // else-if is another IF_EXPR
                        _ => self.execute_statement(else_node),
                    }
                } else {
                    Ok(ControlFlow::NORMAL)
                }
            }


            // While 
            STree::WHILE_EXPR { condition, body } => {
                loop {
                    let cond = self.evaluate_expression(condition)?.is_truthy();
                    if !cond { break; }

                    self.env.push_scope();
                    let flow = self.execute_block(body)?;
                    self.pop_scope()?;

                    match flow {
                        ControlFlow::NORMAL => {}
                        ControlFlow::CONTINUE | ControlFlow::REPEAT => continue,
                        ControlFlow::BREAK => break,
                        ControlFlow::RETURN(v) => return Ok(ControlFlow::RETURN(v)),
                    }
                }
                Ok(ControlFlow::NORMAL)
            }

            // Do-While 
            STree::DO_WHILE { body, condition } => {
                self.env.push_scope();

                let mut first = true;

                loop {
                    let flow = self.execute_block(body)?;

                    match flow {
                        ControlFlow::NORMAL => {}
                        ControlFlow::CONTINUE | ControlFlow::REPEAT => {}
                        ControlFlow::BREAK => break,
                        ControlFlow::RETURN(v) => {
                            self.pop_scope()?;
                            return Ok(ControlFlow::RETURN(v));
                        }
                    }

                    let cond = if first {
                        first = false;
                        self.evaluate_expression(condition)?.is_truthy()
                    } else {
                        self.evaluate_expression(condition)?.is_truthy()
                    };

                    if !cond {
                        break;
                    }
                }

                self.pop_scope()?;
                Ok(ControlFlow::NORMAL)
            }



            // Loop 
            STree::LOOP_EXPR { condition, body } => {
                let count_val = self.evaluate_expression(condition)?;
                let mut remaining = count_val.as_int()?; // must be int

                // TODO: -1 causes infinite loop
                if remaining <= 0 {
                    return Ok(ControlFlow::NORMAL);
                }

                while remaining > 0 {
                    remaining -= 1;

                    self.env.push_scope();
                    let flow = self.execute_block(body)?;
                    self.pop_scope()?;

                    match flow {
                        ControlFlow::NORMAL => {}

                        ControlFlow::CONTINUE => {}

                        ControlFlow::REPEAT => {
                            remaining += 1;
                        }

                        ControlFlow::BREAK => break,

                        ControlFlow::RETURN(v) => return Ok(ControlFlow::RETURN(v)),
                    }
                }

                Ok(ControlFlow::NORMAL)
            }


            // For
            STree::FOR_EXPR { init, condition, modifier, body } => {
                self.env.push_scope(); 

                if let Some(init_stmt) = init {
                    self.execute_statement(init_stmt)?;
                }

                loop {
                    let cond = self.evaluate_expression(condition)?.is_truthy();
                    if !cond { break; }

                    self.env.push_scope();
                    let flow = self.execute_block(body)?;
                    self.pop_scope()?;

                    match flow {
                        ControlFlow::RETURN(v) => { 
                            self.pop_scope()?; 
                            return Ok(ControlFlow::RETURN(v)); 
                        }
                        ControlFlow::BREAK => break,
                        ControlFlow::NORMAL | ControlFlow::CONTINUE => {}
                        ControlFlow::REPEAT => continue,
                    }

                    if let Some(mod_node) = modifier {
                        self.execute_statement(mod_node)?;
                    }
                }

                self.pop_scope()?;
                Ok(ControlFlow::NORMAL)
            }


            // For-each
            STree::FOR_EACH { variable, iterable, body } => {
                self.env.push_scope();

                match iterable.as_ref() {

                    // ---------- RANGE ----------
                    STree::RANGE { start, end, inclusive } => {
                        let start_val = self.evaluate_expression(start)?;
                        let end_val   = self.evaluate_expression(end)?;

                        match (start_val, end_val) {

                            // INT RANGE
                            (Value::INT(a), Value::INT(b)) => {
                                self.env.declare(variable.clone(), Value::INT(a), true);

                                let end_cmp = if *inclusive { b + 1 } else { b };
                                let step = if a <= b { 1 } else { -1 };

                                let mut i = a;
                                while (step > 0 && i < end_cmp) || (step < 0 && i > end_cmp) {
                                    self.env.set(variable, Value::INT(i))?;

                                    let flow = self.execute_block(body)?;
                                    match flow {
                                        ControlFlow::NORMAL => {}
                                        ControlFlow::CONTINUE | ControlFlow::REPEAT => {}
                                        ControlFlow::BREAK => break,
                                        ControlFlow::RETURN(v) => {
                                            self.pop_scope()?;
                                            return Ok(ControlFlow::RETURN(v));
                                        }
                                    }

                                    i += step;
                                }
                            }

                            // FLOAT RANGE
                            (Value::FLOAT(a), Value::FLOAT(b)) => {
                                let step = if a <= b { 1.0 } else { -1.0 };
                                let limit = if *inclusive { b + step } else { b };

                                self.env.declare(variable.clone(), Value::FLOAT(a), true);

                                let mut f = a;
                                while (step > 0.0 && f < limit) || (step < 0.0 && f > limit) {
                                    self.env.set(variable, Value::FLOAT(f))?;

                                    let flow = self.execute_block(body)?;
                                    match flow {
                                        ControlFlow::NORMAL => {}
                                        ControlFlow::CONTINUE | ControlFlow::REPEAT => {}
                                        ControlFlow::BREAK => break,
                                        ControlFlow::RETURN(v) => {
                                            self.pop_scope()?;
                                            return Ok(ControlFlow::RETURN(v));
                                        }
                                    }

                                    f += step;
                                }
                            }

                            // CHAR RANGE
                            (Value::CHAR(a), Value::CHAR(b)) => {
                                let mut c = a as u32;
                                let end = b as u32;
                                let step: i32 = if c <= end { 1 } else { -1 };
                                let limit = if *inclusive { end + step as u32 } else { end };

                                self.env.declare(variable.clone(), Value::CHAR(a), true);

                                while (step > 0 && c < limit) || (step < 0 && c > limit) {
                                    self.env.set(variable, Value::CHAR(char::from_u32(c).unwrap()))?;

                                    let flow = self.execute_block(body)?;
                                    match flow {
                                        ControlFlow::NORMAL => {}
                                        ControlFlow::CONTINUE | ControlFlow::REPEAT => {}
                                        ControlFlow::BREAK => break,
                                        ControlFlow::RETURN(v) => {
                                            self.pop_scope()?;
                                            return Ok(ControlFlow::RETURN(v));
                                        }
                                    }

                                    c = ((c as i32) + step) as u32;
                                }
                            }

                            _ => {
                                self.pop_scope()?;
                                return Err("Invalid range bounds for for-each".to_string());
                            }
                        }
                    }

                    // ---------- STRING ----------
                    _ => {
                        let iterable_val = self.evaluate_expression(iterable)?;

                        let chars = match iterable_val {
                            Value::STRING(s) => s.chars().collect::<Vec<_>>(),
                            _ => {
                                self.pop_scope()?;
                                return Err("for-each iterable must be a range or string".to_string());
                            }
                        };

                        self.env.declare(variable.clone(), Value::NULL, true);

                        for ch in chars {
                            self.env.set(variable, Value::CHAR(ch))?;

                            let flow = self.execute_block(body)?;
                            match flow {
                                ControlFlow::NORMAL => {}
                                ControlFlow::CONTINUE | ControlFlow::REPEAT => {}
                                ControlFlow::BREAK => break,
                                ControlFlow::RETURN(v) => {
                                    self.pop_scope()?;
                                    return Ok(ControlFlow::RETURN(v));
                                }
                            }
                        }
                    }
                }

                self.pop_scope()?;
                Ok(ControlFlow::NORMAL)
            }


            // Match Statement
            STree::MATCH_STMT { expression, arms } => {
                let scrutinee = self.evaluate_expression(expression)?;

                for arm in arms {
                    let STree::MATCH_ARM { expression: pattern, body } = arm else {
                        return Err("Invalid MATCH arm node".to_string());
                    };

                    if let Some(binding) = self.match_pattern(&scrutinee, pattern.as_ref())? {
                        self.env.push_scope();

                        if let Some((name, val)) = binding {
                            self.env.declare(name, val, false);
                        }

                        // body can be BLOCK or single statement
                        let flow = match body.as_ref() {
                            STree::BLOCK { .. } => self.execute_block(body)?,
                            _ => self.execute_statement(body)?,
                        };

                        self.pop_scope()?;
                        return Ok(flow);
                    }
                }

                // No arm matched
                Ok(ControlFlow::NORMAL)
            }


            // Range
            STree::RANGE { .. } => {
                Err("Range expressions cannot be evaluated as values".to_string())
            }


            // Defer
            STree::DEFER_STMT { body } => {
                self.env.defer(*body.clone());
                Ok(ControlFlow::NORMAL)
            }


            // Block 
            STree::BLOCK { .. } => {
                self.env.push_scope();
                let flow = self.execute_block(stmt)?;
                self.pop_scope()?;
                Ok(flow)
            }

            // Blank
            STree::BLANK_STMT => Ok(ControlFlow::NORMAL),

            // Expression statement 
            _ => {
                self.evaluate_expression(stmt)?;
                Ok(ControlFlow::NORMAL)
            }
        }
    }


}

impl Interpreter {
    fn evaluate_expression(&mut self, expression: &STree) -> Result<Value, String> {
        match expression {
            STree::LIT_INT { value } => Ok(Value::INT(*value)),
            STree::LIT_FLOAT { value } => Ok(Value::FLOAT(*value)),
            STree::LIT_BOOL { value } => Ok(Value::BOOLEAN(*value)),
            STree::LIT_CHAR { value } => Ok(Value::CHAR(*value)),
            STree::LIT_STRING { value } => Ok(Value::STRING(value.clone())),
            STree::NULL => Ok(Value::NULL),

            STree::ID { name } => self.env.get(name),

            STree::CALL { path, arguments } => {
                let mut argument_values = Vec::new();
                for arg in arguments {
                    argument_values.push(self.evaluate_expression(arg)?);
                }
                self.call_function(path, argument_values)
            }



            STree::PRFX_EXPR { operator, right } => {
                let value = self.evaluate_expression(right)?;
                match operator {

                    Operator::NOT => Ok(Value::BOOLEAN(!value.as_boolean()?)),

                    Operator::NEGATIVE => {
                        match value {
                            Value::INT(i) => Ok(Value::INT(-i)),
                            Value::FLOAT(f) => Ok(Value::FLOAT(-f)),
                            _ => Err(format!("Expected numeric value, got {:?}", value))
                        }
                    }

                    Operator::RECIPROCAL => {
                        match value {
                            Value::INT(i) => Ok(Value::FLOAT(1.0 / i as f32)),
                            Value::FLOAT(f) => Ok(Value::FLOAT(1.0 / f)),
                            _ => Err(format!("Expected numeric value, got {:?}", value))
                        }
                    }

                    _ => Err(format!("Expected prefix expression, got {:?}", operator))
                }
            }

            STree::PTFX_EXPR { left, operator } => {
                match left.as_ref() {
                    STree::ID { name } => {
                        let current = self.env.get(name)?;

                        let new_value = match operator {
                            Operator::INCREMENT => match current {
                                Value::INT(i) => Value::INT(i + 1),
                                Value::FLOAT(f) => Value::FLOAT(f + 1.0),
                                _ => return Err("++ requires numeric variable".to_string()),
                            },

                            Operator::DECREMENT => match current {
                                Value::INT(i) => Value::INT(i - 1),
                                Value::FLOAT(f) => Value::FLOAT(f - 1.0),
                                _ => return Err("-- requires numeric variable".to_string()),
                            },

                            Operator::SQUARE => match current {
                                Value::INT(i) => Value::INT(i * i),
                                Value::FLOAT(f) => Value::FLOAT(f * f),
                                _ => return Err("** requires numeric variable".to_string()),
                            },

                            _ => return Err("Invalid postfix operator".to_string()),
                        };

                        self.env.set(name, new_value.clone())?;
                        Ok(new_value)
                    }

                    _ => Err("Postfix operator requires variable".to_string()),
                }
            }


            STree::EXPR { left, operator, right } => {
                match operator {
                    Operator::NULL_COAL => {
                        let lhs = self.evaluate_expression(left)?;
                        if !lhs.is_null() {
                            return Ok(lhs);
                        }
                        self.evaluate_expression(right)
                    }

                    Operator::AND => {
                        let lhs = self.evaluate_expression(left)?;
                        if !lhs.as_boolean()? {
                            return Ok(Value::BOOLEAN(false));
                        }
                        let rhs = self.evaluate_expression(right)?;
                        Ok(Value::BOOLEAN(rhs.as_boolean()?))
                    }

                    Operator::OR => {
                        let lhs = self.evaluate_expression(left)?;
                        if lhs.as_boolean()? {
                            return Ok(Value::BOOLEAN(true));
                        }
                        let rhs = self.evaluate_expression(right)?;
                        Ok(Value::BOOLEAN(rhs.as_boolean()?))
                    }

                    _ => {
                        let lhs = self.evaluate_expression(left)?;
                        let rhs = self.evaluate_expression(right)?;
                        self.evaluate_binary_expression(lhs, rhs, operator.clone())
                    }
                }
            }


            STree::ASSIGN_STMT { id, expression } => {
                let value = self.evaluate_expression(expression)?;
                self.env.set(id, value.clone())?;
                Ok(value)
            }

            _ => Err(format!("Cannot evaluate expression: {:?}", expression))
        }
    }

    fn evaluate_binary_expression(&mut self, lhs: Value, rhs: Value, operator: Operator) -> Result<Value, String> {
        match operator {

            // Arithmetic

            Operator::ADD => match (lhs, rhs) {
                (Value::INT(a), Value::INT(b)) => Ok(Value::INT(a + b)),
                (Value::FLOAT(a), Value::FLOAT(b)) => Ok(Value::FLOAT(a + b)),
                (Value::INT(a), Value::FLOAT(b)) => Ok(Value::FLOAT(a as f32 + b)),
                (Value::FLOAT(a), Value::INT(b)) => Ok(Value::FLOAT(a + b as f32)),
                _ => Err("Invalid operands for '+'".to_string()),
            },

            Operator::SUBTRACT => match (lhs, rhs) {
                (Value::INT(a), Value::INT(b)) => Ok(Value::INT(a - b)),
                (Value::FLOAT(a), Value::FLOAT(b)) => Ok(Value::FLOAT(a - b)),
                (Value::INT(a), Value::FLOAT(b)) => Ok(Value::FLOAT(a as f32 - b)),
                (Value::FLOAT(a), Value::INT(b)) => Ok(Value::FLOAT(a - b as f32)),
                _ => Err("Invalid operands for '-'".to_string()),
            },

            Operator::MULTIPLY => match (lhs, rhs) {
                (Value::INT(a), Value::INT(b)) => Ok(Value::INT(a * b)),
                (Value::FLOAT(a), Value::FLOAT(b)) => Ok(Value::FLOAT(a * b)),
                (Value::INT(a), Value::FLOAT(b)) => Ok(Value::FLOAT(a as f32 * b)),
                (Value::FLOAT(a), Value::INT(b)) => Ok(Value::FLOAT(a * b as f32)),
                _ => Err("Invalid operands for '*'".to_string()),
            },

            Operator::DIVIDE => match (lhs, rhs) {
                (_, Value::INT(0)) | (_, Value::FLOAT(0.0)) =>
                    Err("Division by zero".to_string()),

                (Value::INT(a), Value::INT(b)) => Ok(Value::INT(a / b)),
                (Value::FLOAT(a), Value::FLOAT(b)) => Ok(Value::FLOAT(a / b)),
                (Value::INT(a), Value::FLOAT(b)) => Ok(Value::FLOAT(a as f32 / b)),
                (Value::FLOAT(a), Value::INT(b)) => Ok(Value::FLOAT(a / b as f32)),
                _ => Err("Invalid operands for '/'".to_string()),
            },

            Operator::REMAINDER => match (lhs, rhs) {
                (Value::INT(a), Value::INT(b)) => Ok(Value::INT(a % b)),
                _ => Err("Remainder requires integer operands".to_string()),
            },

            Operator::POWER => match (lhs, rhs) {
                (Value::INT(a), Value::INT(b)) => Ok(Value::INT(a.pow(b as u32))),
                (Value::FLOAT(a), Value::FLOAT(b)) => Ok(Value::FLOAT(a.powf(b))),
                _ => Err("Invalid operands for power".to_string()),
            },

            Operator::ROOT => match (lhs, rhs) {
                (Value::INT(a), Value::INT(b)) => Ok(Value::FLOAT((a as f32).powf(1.0 / b as f32))),
                (Value::FLOAT(a), Value::FLOAT(b)) => Ok(Value::FLOAT(a.powf(1.0 / b))),
                _ => Err("Invalid operands for root".to_string()),
            },


            // Logical

            Operator::AND => Ok(Value::BOOLEAN(lhs.as_boolean()? && rhs.as_boolean()?)),
            Operator::OR  => Ok(Value::BOOLEAN(lhs.as_boolean()? || rhs.as_boolean()?)),
            Operator::XOR => Ok(Value::BOOLEAN(lhs.as_boolean()? ^ rhs.as_boolean()?)),


            // Comparison

            Operator::EQUAL => match (lhs, rhs) {
                (Value::INT(a), Value::INT(b)) => Ok(Value::BOOLEAN(a == b)),
                (Value::FLOAT(a), Value::FLOAT(b)) => Ok(Value::BOOLEAN(a == b)),
                (Value::BOOLEAN(a), Value::BOOLEAN(b)) => Ok(Value::BOOLEAN(a == b)),
                (Value::CHAR(a), Value::CHAR(b)) => Ok(Value::BOOLEAN(a == b)),
                (Value::NULL, Value::NULL) => Ok(Value::BOOLEAN(true)),
                (other, Value::NULL) => Ok(Value::BOOLEAN(other.is_null())),
                (Value::NULL, other) => Ok(Value::BOOLEAN(other.is_null())),
                _ => Err("Invalid operands for '=='".to_string()),
            },

            Operator::NOT_EQUAL => match (lhs, rhs) {
                (Value::INT(a), Value::INT(b)) => Ok(Value::BOOLEAN(a != b)),
                (Value::FLOAT(a), Value::FLOAT(b)) => Ok(Value::BOOLEAN(a != b)),
                (Value::BOOLEAN(a), Value::BOOLEAN(b)) => Ok(Value::BOOLEAN(a != b)),
                (Value::CHAR(a), Value::CHAR(b)) => Ok(Value::BOOLEAN(a != b)),
                (Value::NULL, Value::NULL) => Ok(Value::BOOLEAN(false)),
                (other, Value::NULL) => Ok(Value::BOOLEAN(!other.is_null())),
                (Value::NULL, other) => Ok(Value::BOOLEAN(!other.is_null())),
                _ => Err("Invalid operands for '!='".to_string()),
            },

            Operator::LESS_THAN => match (lhs, rhs) {
                (Value::INT(a), Value::INT(b)) => Ok(Value::BOOLEAN(a < b)),
                (Value::FLOAT(a), Value::FLOAT(b)) => Ok(Value::BOOLEAN(a < b)),
                _ => Err("Invalid operands for '<'".to_string()),
            },

            Operator::GREATER_THAN => match (lhs, rhs) {
                (Value::INT(a), Value::INT(b)) => Ok(Value::BOOLEAN(a > b)),
                (Value::FLOAT(a), Value::FLOAT(b)) => Ok(Value::BOOLEAN(a > b)),
                _ => Err("Invalid operands for '>'".to_string()),
            },

            Operator::NOT_GREATER_THAN => match (lhs, rhs) {
                (Value::INT(a), Value::INT(b)) => Ok(Value::BOOLEAN(a <= b)),
                (Value::FLOAT(a), Value::FLOAT(b)) => Ok(Value::BOOLEAN(a <= b)),
                _ => Err("Invalid operands for '<='".to_string()),
            },

            Operator::NOT_LESS_THAN => match (lhs, rhs) {
                (Value::INT(a), Value::INT(b)) => Ok(Value::BOOLEAN(a >= b)),
                (Value::FLOAT(a), Value::FLOAT(b)) => Ok(Value::BOOLEAN(a >= b)),
                _ => Err("Invalid operands for '>='".to_string()),
            },

            Operator::NULL_COAL => match (lhs, rhs) {
                (Value::NULL, other) => Ok(other),
                (other, _) => Ok(other)
            },

            _ => Err(format!("Expected binary operator, got {:?}", operator))

        }
    }

    fn pop_scope(&mut self) -> Result<(), String> {
        let deferred = match self.env.peek_scope() {
            Some(scope) => scope.deferred.clone(),
            None => Vec::new(),
        };

        for stmt in deferred.into_iter().rev() {
            match self.execute_statement(&stmt)? {
                ControlFlow::NORMAL => {}
                _ => return Err("Deferred code cannot affect control flow".to_string()),
            }
        }

        self.env.pop_scope();
        Ok(())
    }


    fn match_pattern(&self, scrutinee: &Value, pattern: &STree) -> Result<Option<Option<(String, Value)>>, String> {
        Ok(match pattern {
            // default
            STree::DEFAULT => Some(None),
            STree::NULL => match scrutinee {
                Value::NULL => Some(None),
                _ => None,
            },

            // binding pattern
            STree::ID { name } => Some(Some((name.clone(), scrutinee.clone()))),

            // literal patterns
            STree::LIT_INT { value } => match scrutinee {
                Value::INT(i) if i == value => Some(None),
                _ => None,
            },

            STree::LIT_FLOAT { value } => match scrutinee {
                Value::FLOAT(f) if f == value => Some(None),
                _ => None,
            },

            STree::LIT_BOOL { value } => match scrutinee {
                Value::BOOLEAN(b) if b == value => Some(None),
                _ => None,
            },

            STree::LIT_CHAR { value } => match scrutinee {
                Value::CHAR(c) if c == value => Some(None),
                _ => None,
            },

            STree::LIT_STRING { value } => match scrutinee {
                Value::STRING(s) if s == value => Some(None),
                _ => None,
            },

            STree::RANGE { start, end, inclusive } => {
                let start_v = Self::literal_value(start.as_ref())?;
                let end_v   = Self::literal_value(end.as_ref())?;

                match (scrutinee, start_v, end_v) {
                    (Value::INT(x), Value::INT(a), Value::INT(b)) => {
                        let ok = if *inclusive { *x >= a && *x <= b } else { *x >= a && *x < b };
                        if ok { Some(None) } else { None }
                    }

                    (Value::FLOAT(x), Value::FLOAT(a), Value::FLOAT(b)) => {
                        let ok = if *inclusive { *x >= a && *x <= b } else { *x >= a && *x < b };
                        if ok { Some(None) } else { None }
                    }

                    (Value::CHAR(x), Value::CHAR(a), Value::CHAR(b)) => {
                        let x = *x as u32;
                        let a = a as u32;
                        let b = b as u32;

                        let ok = if *inclusive { x >= a && x <= b } else { x >= a && x < b };
                        if ok { Some(None) } else { None }
                    }

                    _ => None,
                }
            }



            _ => return Err(format!("Invalid match pattern: {:?}", pattern)),
        })
    }

    pub fn literal_value(pattern: &STree) -> Result<Value, String> {
        match pattern {
            STree::LIT_INT { value }    => Ok(Value::INT(*value)),
            STree::LIT_FLOAT { value }  => Ok(Value::FLOAT(*value)),
            STree::LIT_CHAR { value }   => Ok(Value::CHAR(*value)),
            STree::LIT_BOOL { value }   => Ok(Value::BOOLEAN(*value)),
            STree::LIT_STRING { value } => Ok(Value::STRING(value.clone())),
            STree::NULL => Ok(Value::NULL),

            _ => Err("Non-literal used in match pattern".to_string()),
        }
    }


}

