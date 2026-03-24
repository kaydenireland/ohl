use std::collections::HashMap;
use std::fmt::format;
use std::ops::Deref;
use colored::Colorize;
use crate::core::analyzer::function::FunctionSignature;
use crate::core::analyzer::scope::Scope;
use crate::core::analyzer::variable::{VariableSignature, VariableType};
use crate::core::converter::stree::STree;
use crate::core::lexer::token_type::TokenType;
use crate::core::util::logger::Logger;

#[derive(Debug, Clone)]
pub struct Analyzer {
    pub functions: HashMap<String, FunctionSignature>,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub log: Logger,
    loop_depth: usize
}

impl Analyzer {
    pub fn new(_debug: bool) -> Analyzer {
        Analyzer {
            functions: HashMap::new(),
            errors: Vec::new(),
            warnings: Vec::new(),
            log: Logger::new(_debug),
            loop_depth: 0
        }
    }

    pub fn analyze(&mut self, tree: STree) -> Result<Vec<String>, (Vec<String>, Vec<String>)> {

        self.collect_function_signatures(&tree);
        self.visit(&tree, &mut Scope::new());

        self.print_function_table();

        // Detect unused Functions
        let function_map = self.functions.clone();
        for function in function_map.values() {
            if !function.called {
                self.create_warning_message(format!(
                        "Unused function '{}'", function.name
                    )
                )
            }
        }

        if !self.errors.is_empty() {
            Err((self.warnings.clone(), self.errors.clone()))
        } else {
            Ok(self.warnings.clone())
        }
    }

    fn visit(&mut self, node: &STree, scope: &mut Scope) -> Option<VariableType> {
        match node {

            STree::START { functions } => {
                self.log.info("analyze()");
                self.log.indent_inc();

                for function in functions {
                    self.visit(function, scope);
                }

                self.log.indent_dec();
                None
            }

            STree::FUNCTION { function_type, return_type, name, params, body } => {
                self.log.info("analyze_function()");
                self.log.indent_inc();

                let mut local = Scope::new();
                for (name, token_type) in params {
                    let _ =local.declare_variable(name.clone(), token_type.clone(), false);
                }

                self.visit(body, &mut local);

                if *return_type != VariableType::NULL {
                    if !self.has_return(body) {
                        self.create_error_message(format!(
                            "Function '{}' declares return type {:?} but has no return statement",
                            name, return_type
                        ));
                    }
                }

                self.log.indent_dec();
                Some(return_type.clone())
            }

            STree::BLOCK { statements } => {
                self.log.info("analyze_block()");
                self.log.indent_inc();

                let mut local = Scope::new_child(scope);

                for statement in statements {
                    self.visit(statement, &mut local);
                }

                if statements.is_empty() {
                    self.create_warning_message(
                        "Empty Block".to_string()
                    )
                }

                self.log.indent_dec();
                None
            }

            STree::VAR_DECL { id, var_type, mutable, expression} => {
                self.log.info("analyze_variable_declaration()");
                self.log.indent_inc();

                match scope.declare_variable(id.clone(), var_type.clone(), mutable.clone()) {
                    Ok(_) => {}
                    Err(_) => {
                        _ =scope.mark_used(id.as_str(), false);
                        _ = scope.mark_mutability(id.as_str(), mutable.clone());
                    }
                };

                self.visit(expression, scope);

                self.log.indent_dec();
                None
            }

            STree::WHILE_STMT { condition, body }
            | STree::DO_WHILE_STMT { condition, body } => {
                self.log.info("analyze_while()");
                self.log.indent_inc();

                self.visit(condition, scope);

                self.loop_depth += 1;
                let mut local = Scope::new();
                self.visit(body, &mut local);
                self.loop_depth -= 1;

                self.log.indent_dec();
                None
            },

            STree::BREAK | STree::CONTINUE | STree::REPEAT => {
                self.log.info("analyze_jump()");
                if self.loop_depth == 0 {
                    self.create_error_message("Jump statement used outside of loop".to_string())
                }
                None
            },

            STree::FUNCTION_CALL { callee, args } => {
                self.log.info("analyze_function_call()");
                self.log.indent_inc();

                let name = match callee.deref() {
                    STree::ID { name } => {
                        name
                    }
                    _ => {
                        self.create_error_message(format!("Callee '{:?}' is not a function call", callee));
                        self.log.indent_dec();
                        return None
                    }
                };

                let called_function_option = self.functions.get(name).cloned();
                let function = match called_function_option {
                    Some(func) => {
                        func
                    },
                    None => {
                        self.create_error_message(format!("Called function '{:?}' does not exist", name));
                        self.log.indent_dec();
                        return None
                    }
                };

                if let Some(f) = self.functions.get_mut(name) {
                    f.call();
                }

                if function.parameters.len() != args.len() {
                    self.create_error_message(format!(
                        "Function '{}' expects {} arguments, got {}",
                        name, function.parameters.len(), args.len()
                    ));
                }

                for (param, arg) in function.parameters.iter().zip(args.iter()) {
                    let arg_type = self.visit(arg, scope).unwrap_or(VariableType::NULL);
                    if *param != arg_type {
                        self.create_error_message(format!(
                            "Argument type mismatch in '{}': expected {:?}, got {:?}",
                            name, param, arg_type
                        ))
                    }
                }

                self.log.indent_dec();
                Some(function.return_type.clone())
            },

            STree::ID { name } => {
                if name.chars().nth(0).unwrap().is_ascii_uppercase() {
                    self.create_warning_message(format!("Variable name '{}' should not start with uppercase letter", name));
                }

                match scope.mark_used(name, true) {
                    Ok(_) => {}
                    Err(msg) => {
                        self.create_error_message(msg);
                    }
                }

                Some(scope.check_variable(name).unwrap_or(VariableType::NULL))
            },

            STree::LIT_INT { .. } => Some(VariableType::INT),
            STree::LIT_FLOAT { .. } => Some(VariableType::FLOAT),
            STree::LIT_CHAR { .. } => Some(VariableType::CHAR),
            STree::LIT_STRING { .. } => Some(VariableType::STRING),
            STree::LIT_BOOL { .. } => Some(VariableType::BOOLEAN),
            STree::NULL => Some(VariableType::NULL),

            STree::BLANK => {
                self.create_warning_message("Unnecessary semicolons".to_string());
                None
            }

            _ => None
        }
    }

}

// Helpers
impl Analyzer {

    pub fn create_warning_message(&mut self, msg: String) {
        self.warnings.push(
            format!(
                "{}: {}",
                "Warning".yellow(),
                msg
            )
        );
    }
    pub fn create_error_message(&mut self, msg: String) {
        self.errors.push(
            format!(
                "{}: {}",
                "Error".red(),
                msg
            )
        );
    }

    pub fn print_function_table(&mut self) {
        self.log.info("\nFunction Table:");
        self.log.indent_inc();
        for function in self.functions.values() {
            let params = function.parameters.clone();
            self.log.info(format!("{}: {:?}", function.name, params).as_str());
        }
        self.log.indent_dec();
    }

    pub fn collect_function_signatures(&mut self, node: &STree) {
        match node {
            STree::START { functions} => {
                for function in functions {
                    self.collect_function_signatures(function);
                }
            },

            STree::FUNCTION { function_type, return_type, name, params, .. } => {
                let mut param_types = Vec::new();
                for (_, token_type) in params {
                    param_types.push(token_type.clone());
                }

                self.functions.insert(
                    name.to_string(),
                    FunctionSignature::new(
                        name.clone(),
                        param_types,
                        return_type.clone(),
                        name == "main"
                    )
                );
            },

            _ => {}
        }
    }

    fn has_return(&self, node: &STree) -> bool {
        match node {
            STree::RETURN_STMT { .. } => true,
            STree::BLOCK { statements } => statements.iter().any(|s| self.has_return(s)),
            STree::IF_STMT { then_block, else_block, .. } => {
                let then_has = self.has_return(then_block);
                let else_has = else_block.as_ref().map(|b| self.has_return(b)).unwrap_or(false);
                then_has || else_has
            }
            STree::FUNCTION { body, .. } => self.has_return(body),
            STree::START { functions } => functions.iter().any(|f| self.has_return(f)),
            _ => false,
        }
    }

}