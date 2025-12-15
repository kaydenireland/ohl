use std::collections::HashMap;
use std::collections::hash_map::Entry;
use crate::language::analyzing::operator::Operator;
use crate::language::analyzing::stree::STree;
use crate::language::logger::Logger;
use crate::language::analyzing::types::VariableType;
use crate::language::analyzing::symbol_table::SymbolTable;



#[derive(Debug, Clone)]
pub struct FunctionSignature {
    parameters: Vec<VariableType>,
    return_type: VariableType
}

pub struct Analyzer {
    pub functions: HashMap<String, FunctionSignature>,
    pub errors: Vec<String>,
    pub log: Logger,
    loop_depth: usize
}

impl Analyzer {
    pub fn new(_debug: bool) -> Analyzer {
        let log = Logger::new(_debug);
        Analyzer {
            functions: HashMap::new(),
            errors: vec![],
            log,
            loop_depth: 0
        }
    }
    
    pub fn analyze(mut self, tree: &STree) -> Result<VariableType, Vec<String>> {
        self.log.info("\nanalyze_tree()");
        self.log.indent_inc();

        self.collect_function_signatures(tree);
        let ty = self.visit(tree, &mut SymbolTable::new());

        if self.errors.is_empty() {
            Ok(ty)
        } else {
            Err(self.errors)
        }
    }
}

impl Analyzer {
    fn visit(&mut self, node: &STree, symbols: &mut SymbolTable) -> VariableType {
        match node {
            STree::START { functions } => {
                for f in functions {
                    self.visit(f, symbols);
                }
                VariableType::NULL
            }

            STree::FUNCTION { function_type: _, name, params, return_type, body } => {
                self.log.info("analyze_function()");
                self.log.indent_inc();

                let mut local = SymbolTable::new();
                for (pname, ptype) in params {
                    let _ = local.declare_variable(pname.clone(), ptype.clone());
                }

                let body_type = self.visit(body, &mut local);

                if *return_type != VariableType::NULL
                    && body_type != *return_type
                    && body_type != VariableType::NULL
                {
                    self.errors.push(format!(
                        "Function '{}' declared return type {:?}, but body returns {:?}",
                        name, return_type, body_type
                    ));
                }

                if *return_type != VariableType::NULL && !self.has_return(body) {
                    self.errors.push(format!(
                        "Function '{}' declares return type {:?} but has no return statement",
                        name, return_type
                    ));
                }

                self.log.indent_dec();

                VariableType::NULL
            }

            STree::BLOCK { statements } => {
                self.log.info("analyze_block()");
                self.log.indent_inc();

                let mut local = SymbolTable::new_child(symbols);
                for s in statements {
                    self.visit(s, &mut local);
                }

                self.log.indent_dec();

                VariableType::NULL
            }

            STree::LET_STMT { id, var_type, expression } => {
                self.log.info("analyze_let()");
                self.log.indent_inc();

                let inferred = if let Some(e) = expression {
                    let et = self.visit(e, symbols);
                    if *var_type != VariableType::NULL && et != *var_type && et != VariableType::NULL {
                        self.errors.push(format!(
                            "VariableType mismatch for '{}': expected {:?}, found {:?}",
                            id, var_type, et
                        ));
                    }
                    if *var_type == VariableType::NULL { et } else { var_type.clone() }
                } else {
                    var_type.clone()
                };

                if let Err(e) = symbols.declare_variable(id.clone(), inferred) {
                    self.errors.push(e);
                }

                self.log.indent_dec();

                VariableType::NULL
            }

            STree::ASSIGN_STMT { id, expression } => {
                self.log.info("analyze_assignment()");
                self.log.indent_inc();

                match symbols.check_variable(id) {
                    Ok(vt) => {
                        let et = self.visit(expression, symbols);
                        if vt != et && vt != VariableType::NULL && et != VariableType::NULL {
                            self.errors.push(format!(
                                "Assignment type mismatch for '{}': {:?} vs {:?}",
                                id, vt, et
                            ));
                        }
                    }
                    Err(e) => self.errors.push(e),
                }

                self.log.indent_dec();

                VariableType::NULL
            }

            STree::RETURN_STMT { expression } => {
                self.log.info("analyze_return()");
                self.log.indent_inc();
                
                let vt = expression
                    .as_ref()
                    .map(|e| self.visit(e, symbols))
                    .unwrap_or(VariableType::NULL);
                
                self.log.indent_dec();

                vt
            }

            STree::LOOP_EXPR { condition, body } => {
                self.log.info("analyze_loop()");
                self.log.indent_inc();

                match condition.as_ref() {
                    STree::LIT_INT { .. } => {}

                    STree::ID { name } => {
                        match symbols.check_variable(name) {
                            Ok(VariableType::INT) => {}
                            Ok(other) => {
                                self.errors.push(format!(
                                    "Loop condition variable '{}' must be INT, found {:?}",
                                    name, other
                                ));
                            }
                            Err(e) => self.errors.push(e),
                        }
                    }

                    _ => {
                        let ct = self.visit(condition, symbols);
                        self.errors.push(format!(
                            "Loop condition must be an int literal or int variable, found {:?}",
                            ct
                        ));
                    }
                }

                self.loop_depth += 1;
                self.visit(body, symbols);
                self.loop_depth -= 1;

                self.log.indent_dec();
                VariableType::NULL
            }

            STree::WHILE_EXPR { condition, body } => {
                self.log.info("analyze_while()");
                self.log.indent_inc();

                let ct = self.visit(condition, symbols);
                if ct != VariableType::BOOLEAN && ct != VariableType::NULL {
                    self.errors.push(format!(
                        "While condition must be Bool, found {:?}",
                        ct
                    ));
                }

                self.loop_depth += 1;
                let mut local = SymbolTable::new_child(symbols);
                self.visit(body, &mut local);
                self.loop_depth -= 1;

                self.log.indent_dec();

                VariableType::NULL
            }

            STree::FOR_EXPR { init, condition, modifier, body } => {
                self.log.info("analyze_for()");
                self.log.indent_inc();

                let mut local = SymbolTable::new_child(symbols);

                // init
                if let Some(i) = init {
                    self.visit(i, &mut local);
                }

                // condition
                let ct = self.visit(condition, &mut local);
                if ct != VariableType::BOOLEAN && ct != VariableType::NULL {
                    self.errors.push(format!(
                        "For-loop condition must be Bool, found {:?}",
                        ct
                    ));
                }

                self.loop_depth += 1;

                // body
                self.visit(body, &mut local);

                // modifier
                if let Some(m) = modifier {
                    self.visit(m, &mut local);
                }

                self.loop_depth -= 1;

                self.log.indent_dec();

                VariableType::NULL
            }


            STree::FOR_EACH { variable, iterable, body } => {
                self.log.info("analyze_for_each()");
                self.log.indent_inc();

                let it_type = self.visit(iterable, symbols);

                let element_type = match it_type {
                    VariableType::STRING => VariableType::CHAR,

                    _ => {
                        self.errors.push(format!(
                            "Cannot iterate over type {:?}",
                            it_type
                        ));
                        VariableType::NULL
                    }
                };

                let mut local = SymbolTable::new_child(symbols);

                if let Err(e) = local.declare_variable(variable.clone(), element_type) {
                    self.errors.push(e);
                }

                self.loop_depth += 1;
                self.visit(body, &mut local);
                self.loop_depth -= 1;

                self.log.indent_dec();
                VariableType::NULL
            }

            STree::BREAK => {
                self.log.info("analyze_break()");
                self.require_loop("break");
                VariableType::NULL
            }

            STree::CONTINUE => {
                self.log.info("analyze_continue()");
                self.require_loop("continue");
                VariableType::NULL
            }

            STree::REPEAT => {
                self.log.info("analyze_repeat()");
                self.require_loop("repeat");
                VariableType::NULL
            }


            STree::IF_EXPR { condition, then_block, else_block } => {
                self.log.info("analyze_if()");
                self.log.indent_inc();

                let ct = self.visit(condition, symbols);
                if ct != VariableType::BOOLEAN && ct != VariableType::NULL {
                    self.errors.push(format!(
                        "If condition must be Bool, found {:?}",
                        ct
                    ));
                }

                let mut then_scope = SymbolTable::new_child(symbols);
                let tt = self.visit(then_block, &mut then_scope);

                let et = if let Some(else_block) = else_block {
                    let mut else_scope = SymbolTable::new_child(symbols);
                    self.visit(else_block, &mut else_scope)
                } else {
                    VariableType::NULL
                };

                if tt != VariableType::NULL && et != VariableType::NULL && tt != et {
                    self.errors.push(format!(
                        "If branches return different types: {:?} vs {:?}",
                        tt, et
                    ));
                }

                self.log.indent_dec();

                if tt != VariableType::NULL { tt } else { et }
            }


            STree::PRINT_STMT { expression } => {
                self.log.info("analyze_print()");
                self.log.indent_inc();

                self.visit(expression, symbols);

                self.log.indent_dec();

                VariableType::NULL
            }

            STree::EXPR { left, operator, right } => {
                self.log.info("analyze_expression()");

                let rt = self.visit(right, symbols);

                // Unary operators
                match operator {
                    Operator::NOT => {
                        if rt != VariableType::BOOLEAN && rt != VariableType::NULL {
                            self.errors.push(format!(
                                "Unary NOT requires Bool, found {:?}",
                                rt
                            ));
                        }
                        return VariableType::BOOLEAN;
                    }

                    Operator::NEGATIVE => {
                        if rt.is_numeric() {
                            return rt;
                        } else {
                            self.errors.push(format!(
                                "Unary minus requires numeric type, found {:?}",
                                rt
                            ));
                            return VariableType::NULL;
                        }
                    }

                    Operator::RECIPRICOL => {
                        if rt.is_numeric() {
                            return rt;
                        } else {
                            self.errors.push(format!(
                                "Unary slash requires numeric type, found {:?}",
                                rt
                            ));
                            return VariableType::NULL;
                        }
                    }

                    _ => {},
                }

                // Binary operators 
                let lt = self.visit(left, symbols);
                self.type_binary_operator(operator, lt, rt)
            }

            STree::PRFX_EXPR { operator, right } => {
                self.log.info("analyze_prefix_expr()");
                self.log.indent_inc();

                let rt = self.visit(right, symbols);

                match operator {
                    

                    Operator::NOT => {
                        if rt == VariableType::BOOLEAN || rt == VariableType::NULL {
                            VariableType::BOOLEAN
                        } else {
                            self.errors.push(format!(
                                "Unary NOT requires Bool, found {:?}",
                                rt
                            ));
                            VariableType::NULL
                        }
                    }

                    Operator::NEGATIVE => {
                        if rt.is_numeric() {
                            rt
                        } else {
                            self.errors.push(format!(
                                "Unary minus requires numeric type, found {:?}",
                                rt
                            ));
                            VariableType::NULL
                        }
                    }

                    _ => VariableType::NULL,
                }
            }

            STree::PTFX_EXPR { left, operator } => {
                self.log.info("analyze_postfix_expr()");
                self.log.indent_inc();

                let lt = self.visit(left, symbols);

                match operator {
                    Operator::INCREMENT | Operator::DECREMENT | Operator::SQUARE => {
                        match left.as_ref() {
                            STree::ID { .. } if lt.is_numeric() => lt,
                            _ => {
                                self.errors.push(format!(
                                    "Postfix {:?} requires a numeric variable",
                                    operator
                                ));
                                VariableType::NULL
                            }
                        }
                    }

                    _ => VariableType::NULL,
                }
            }



            STree::CALL { name, arguments } => {
                self.log.info("analyze_function_call()");

                let arg_types: Vec<_> = arguments
                    .iter()
                    .map(|a| self.visit(a, symbols))
                    .collect();

                if let Some(FunctionSignature { parameters, return_type}) = self.functions.get(name) {
                    if parameters.len() != arg_types.len() {
                        self.errors.push(format!(
                            "Function '{}' expects {} args but {} provided",
                            name,
                            parameters.len(),
                            arg_types.len()
                        ));
                    }
                    return_type.clone()
                } else {
                    self.errors.push(format!(
                        "Call to unknown function '{}'",
                        name
                    ));
                    VariableType::NULL
                }
            }

            STree::ID { name } => {
                self.log.info("analyze_identifier()");

                match symbols.check_variable(name) {
                    Ok(t) => t,
                    Err(e) => {
                        self.errors.push(e);
                        VariableType::NULL
                    }
                }
            }

            STree::LIT_INT { .. } => VariableType::INT,
            STree::LIT_FLOAT { .. } => VariableType::FLOAT,
            STree::LIT_BOOL { .. } => VariableType::BOOLEAN,
            STree::LIT_CHAR { .. } => VariableType::CHAR,
            STree::LIT_STRING { .. } => VariableType::STRING,

        }
    }
}



impl Analyzer {
    fn collect_function_signatures(&mut self, tree: &STree) {
        if let STree::START { functions } = tree {
            for func in functions {
                self.log.info("collecting_function_signature()");
                self.log.indent_inc();
                
                // TODO: Function Types
                if let STree::FUNCTION { return_type, name, params, .. } = func {
                    let param_types = params
                        .iter()
                        .map(|(_, t)| t.clone())
                        .collect();
                    match self.functions.entry(name.clone()) {
                        Entry::Occupied(_) => self.errors.push(format!("Function '{}' already declared", name)),
                        Entry::Vacant(v) => {
                            v.insert(FunctionSignature { parameters: param_types, return_type: return_type.clone()});
                        }
                        
                    }
                }
                self.log.indent_dec();
            }
        }
    }

    fn has_return(&self, node: &STree) -> bool {
        match node {
            STree::RETURN_STMT { .. } => true,
            STree::BLOCK { statements } => statements.iter().any(|s| self.has_return(s)),
            STree::IF_EXPR { then_block, else_block, .. } => {
                let then_has = self.has_return(then_block);
                let else_has = else_block
                    .as_ref()
                    .map(|b| self.has_return(b))
                    .unwrap_or(false);
                then_has || else_has
            }
            STree::FUNCTION { body, .. } => self.has_return(body),
            STree::START { functions } => functions.iter().any(|f| self.has_return(f)),
            _ => false,
        }
    }
}

impl Analyzer {
    fn type_binary_operator(&mut self, operator: &Operator, left: VariableType, right: VariableType) -> VariableType {

        if left == VariableType::NULL || right == VariableType::NULL {
            return VariableType::NULL;
        }

        match operator {
            // Arithmetic
            Operator::ADD | Operator::SUBTRACT | Operator::MULTIPLY | Operator::DIVIDE | Operator::REMAINDER 
            | Operator::POWER | Operator::ROOT => {
                if left.is_numeric() && right.is_numeric() {
                    if left == VariableType::FLOAT || right == VariableType::FLOAT {
                        VariableType::FLOAT
                    } else {
                        VariableType::INT
                    }
                } else if *operator == Operator::ADD && left == VariableType::STRING && right == VariableType::STRING {
                    VariableType::STRING
                } else {
                    self.errors.push(format!(
                        "Invalid operands for {:?}: {:?} and {:?}",
                        operator, left, right
                    ));
                    VariableType::NULL
                }
            }

            // Comparisons
            Operator::EQUAL | Operator::NOT_EQUAL => {
                if left == right {
                    VariableType::BOOLEAN
                } else {
                    self.errors.push(format!(
                        "Cannot compare {:?} with {:?}",
                        left, right
                    ));
                    VariableType::NULL
                }
            }

            Operator::LESS_THAN | Operator::GREATER_THAN 
            | Operator::NOT_GREATER_THAN | Operator::NOT_LESS_THAN => {
                if left.is_numeric() && right.is_numeric() {
                    VariableType::BOOLEAN
                } else {
                    self.errors.push(format!(
                        "Comparison requires numeric types, got {:?} and {:?}",
                        left, right
                    ));
                    VariableType::NULL
                }
            }

            // Boolean
            Operator::AND | Operator::OR | Operator::XOR => {
                if left == VariableType::BOOLEAN && right == VariableType::BOOLEAN {
                    VariableType::BOOLEAN
                } else {
                    self.errors.push(format!(
                        "Logical operator {:?} requires Bool operands",
                        operator
                    ));
                    VariableType::NULL
                }
            }

            _ => VariableType::NULL,
        }
    }

    fn require_loop(&mut self, keyword: &str) {
        if self.loop_depth == 0 {
            self.errors.push(format!(
                "'{}' used outside of a loop",
                keyword
            ));
        }
    }

}