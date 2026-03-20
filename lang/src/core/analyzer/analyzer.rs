use std::collections::HashMap;
use colored::Colorize;
use crate::core::analyzer::function::FunctionSignature;
use crate::core::analyzer::scope::Scope;
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

        self.visit(&tree, &mut Scope::new());

        if !self.errors.is_empty() {
            Err((self.warnings.clone(), self.errors.clone()))
        } else {
            Ok(self.warnings.clone())
        }
    }

    fn visit(&mut self, node: &STree, scope: &mut Scope) {
        match node {

            STree::START { functions } => {
                self.log.info("analyze()");
                self.log.indent_inc();

                for function in functions {
                    self.visit(function, scope);
                }

                self.log.indent_dec();
            }

            STree::FUNCTION { function_type, return_type, name, params, body } => {
                self.log.info("analyze_function()");
                self.log.indent_inc();

                let mut local = Scope::new();
                for (name, token_type) in params {
                    let _ =local.declare_variable(name.clone(), token_type.clone(), false);
                }

                self.visit(body, &mut local);

                if *return_type != TokenType::NULL {
                    if !self.has_return(body) {
                        self.errors.push(self.create_error_message(format!(
                            "Function '{}' declares return type {:?} but has no return statement",
                            name, return_type
                        )));
                    }
                }

                self.log.indent_dec();
            }

            STree::BLOCK { statements } => {
                self.log.info("analyze_block()");
                self.log.indent_inc();

                let mut local = Scope::new_child(scope);

                for statement in statements {
                    self.visit(statement, &mut local);
                }

                if statements.is_empty() {
                    self.warnings.push(self.create_warning_message(
                        "Empty Block".to_string()
                    ))
                }

                self.log.indent_dec();
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
            },

            STree::BREAK | STree::CONTINUE | STree::REPEAT => {
                self.log.info("analyze_jump()");
                if self.loop_depth == 0 {
                    self.errors.push(
                        self.create_error_message("Jump Statement Used Outside of Loop".to_string())
                    )
                }
            }


            _ => {}
        }
    }
}

// Helpers
impl Analyzer {

    pub fn create_warning_message(&self, msg: String) -> String {
        format!(
            "{}: {}",
            "Warning".yellow(),
            msg
        )
    }
    pub fn create_error_message(&self, msg: String) -> String {
        format!(
            "{}: {}",
            "Error".red(),
            msg
        )
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