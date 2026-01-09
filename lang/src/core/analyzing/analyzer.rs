use std::collections::{HashMap, hash_map::Entry};
use crate::core::analyzing::operator::Operator;
use crate::core::analyzing::stree::STree;
use crate::core::logger::Logger;
use crate::core::analyzing::types::VariableType;
use crate::core::analyzing::symbol_table::SymbolTable;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Flow {
    CONTINUE,
    STOP,
}

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub parameters: Vec<VariableType>,
    pub return_type: VariableType,
    pub called: bool
}

pub struct Analyzer {
    pub functions: HashMap<Vec<String>, FunctionSignature>,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub log: Logger,
    loop_depth: usize,
    defer_depth: usize
}
// TODO Better Null Analyzing
impl Analyzer {
    pub fn new(_debug: bool) -> Analyzer {
        let log = Logger::new(_debug);
        let mut analyzer = Analyzer {
            functions: HashMap::new(),
            errors: vec![],
            warnings: vec![],
            log,
            loop_depth: 0,
            defer_depth: 0
        };

        analyzer.register_native_functions();
        analyzer
    }


    pub fn analyze(mut self, tree: &STree) -> Result<Vec<String>, Vec<String>> {
        self.collect_function_signatures(tree);
        self.visit(tree, &mut SymbolTable::new());

        for (path, sig) in &self.functions {
            if !sig.called {
                self.warnings.push(format!(
                    "Function '{}' is never called",
                    path.join(".")
                ));
            }
        }


        if self.errors.is_empty() {
            Ok(self.warnings)
        } else {
            Err(self.errors)
        }
    }
}

impl Analyzer {
    fn visit(&mut self, node: &STree, symbols: &mut SymbolTable) -> (Option<VariableType>, Flow) {
        match node {

            STree::START { functions } => {
                let mut flow = Flow::CONTINUE;
                for f in functions {
                    let (_, function_flow) = self.visit(f, symbols);
                    flow = function_flow;
                }
                (None, flow)
            }

            STree::FUNCTION { name, params, return_type, body, .. } => {
                self.log.info("analyze_function()");
                self.log.indent_inc();

                let mut local = SymbolTable::new();
                for (pname, ptype) in params {
                    let _ = local.declare_variable(pname.clone(), ptype.clone(), false);
                }

                let (body_ty_opt, body_flow) = self.visit(body, &mut local);

                if *return_type != VariableType::NULL {
                    if !self.has_return(body) {
                        self.errors.push(format!(
                            "Function '{}' declares return type {:?} but has no return statement",
                            name, return_type
                        ));
                    }
                }

                if let Some(body_type) = body_ty_opt {
                    if *return_type != VariableType::NULL && self.is_assignable(body_type.clone(), return_type.clone())
                    {
                        self.errors.push(format!(
                            "Function '{}' declared return type {:?}, but body returns {:?}",
                            name, return_type, body_type
                        ));
                    }
                }

                self.log.indent_dec();
                (None, body_flow)
            }


            STree::BLOCK { statements } => {
                self.log.info("analyze_block()");
                self.log.indent_inc();

                let mut local = SymbolTable::new_child(symbols);

                let mut flow = Flow::CONTINUE;
                for (idx, s) in statements.iter().enumerate() {
                    if flow == Flow::STOP {
                        self.warnings.push(format!(
                            "Unreachable code: statement {} in block can never execute",
                            idx + 1
                        ));
                        continue;
                    }
                    let (_, sflow) = self.visit(s, &mut local);
                    flow = sflow;
                }

                for (name, info) in local.variables {
                    if !info.used {
                        self.warnings.push(format!(
                            "Unused variable '{}'",
                            name
                        ));
                    }
                }


                self.log.indent_dec();
                (None, flow)
            }

            STree::LET_STMT { id, var_type, mutable, expression } => {
                self.log.info("analyze_let()");
                self.log.indent_inc();

                let inferred = if let e = expression {
                    let (et_opt, _) = self.visit(e, symbols);
                    let et = et_opt.unwrap_or(VariableType::NULL);

                    if *var_type != VariableType::NULL && !self.is_assignable(et.clone(), var_type.clone()) {
                        self.errors.push(format!(
                            "VariableType mismatch for '{}': expected {:?}, found {:?}",
                            id, var_type, et
                        ));
                    }

                    if *var_type == VariableType::NULL { et } else { var_type.clone() }
                } else {
                    var_type.clone()
                };

                if let Err(e) = symbols.declare_variable(id.clone(), inferred, mutable.clone()) {
                    self.errors.push(e);
                }

                self.log.indent_dec();
                (None, Flow::CONTINUE)
            }

            STree::ASSIGN_STMT { id, expression } => {
                self.log.info("analyze_assignment()");
                self.log.indent_inc();

                match symbols.check_variable(id) {
                    Ok(vt) => {
                        let (et_opt, _) = self.visit(expression, symbols);
                        let et = et_opt.unwrap_or(VariableType::NULL);

                        if !self.is_assignable(et.clone(), vt.clone()){
                            self.errors.push(format!(
                                "Assignment type mismatch for '{}': {:?} vs {:?}",
                                id, vt, et
                            ));
                        }
                    }
                    Err(e) => self.errors.push(e),
                }

                match symbols.check_mutability(id) {
                    Ok(mutable) => {
                        if !mutable {
                            self.errors.push(format!("Variable '{}' is immutable and cannot be modified", id));
                        }
                    }
                    Err(e) => self.errors.push(e),
                }

                self.log.indent_dec();
                (None, Flow::CONTINUE)
            }

            STree::RETURN_STMT { expression } => {
                self.log.info("analyze_return()");
                self.log.indent_inc();

                let ty = expression
                    .as_ref()
                    .map(|e| self.visit(e, symbols).0.unwrap_or(VariableType::NULL))
                    .unwrap_or(VariableType::NULL);

                self.log.indent_dec();
                (Some(ty), Flow::STOP)
            }

            STree::DEFER_STMT { body } => {
                self.log.info("analyze_defer()");
                self.log.indent_inc();

                self.defer_depth += 1;

                match body.as_ref() {
                    STree::BLOCK { .. } => {
                        let mut local = SymbolTable::new_child(symbols);
                        self.visit(body, &mut local);
                    }
                    _ => {
                        self.visit(body, symbols);
                    }
                }

                self.defer_depth -= 1;

                self.log.indent_dec();
                (None, Flow::CONTINUE)
            }

            STree::MATCH_STMT { expression, arms } => {
                self.log.info("analyze_match()");
                self.log.indent_inc();

                let scrutinee_ty = self.visit(expression, symbols).0
                    .unwrap_or(VariableType::NULL);

                let mut arm_return_type: Option<VariableType> = None;
                let mut all_stop = true;

                let mut seen_default = false;
                let mut default_count: usize = 0;

                for arm in arms {
                    let STree::MATCH_ARM { expression: pattern, body } = arm else {
                        unreachable!();
                    };

                    if seen_default {
                        self.warnings.push(format!(
                            "Unreachable match arm '{:?}': all arms after default are unreachable",
                            pattern
                        ));
                    }

                    if matches!(pattern.as_ref(), STree::DEFAULT) {
                        default_count += 1;
                        seen_default = true;
                    }


                    let mut arm_scope = SymbolTable::new_child(symbols);

                    self.analyze_match_pattern(pattern, scrutinee_ty.clone(), &mut arm_scope);

                    let (result, flow) = self.visit(body, &mut arm_scope);

                    if let Some(body_type) = result {
                        match &arm_return_type {
                            None => arm_return_type = Some(body_type),
                            Some(prev) if *prev == body_type => {}
                            Some(_) => {
                                self.errors.push(
                                    "Match arms return incompatible types".to_string()
                                );
                                arm_return_type = Some(VariableType::NULL);
                            }
                        }
                    }

                    if flow != Flow::STOP {
                        all_stop = false;
                    }
                }

                if default_count > 1 {
                    self.errors.push(format!("Match can have one default arm at most, currently {}", default_count));
                }

                self.log.indent_dec();

                (
                    arm_return_type,
                    if all_stop { Flow::STOP } else { Flow::CONTINUE }
                )
            }


            STree::MATCH_ARM { expression: _, body: _ } => {
                self.errors.push("MATCH_ARM must be analyzed inside MATCH_STMT".to_string());
                (None, Flow::CONTINUE)
            }

            STree::DEFAULT => {
                self.log.info("analyze_default()");
                (Some(VariableType::NULL), Flow::CONTINUE)
            }

            STree::IF_EXPR { condition, then_block, else_block } => {
                self.log.info("analyze_if()");
                self.log.indent_inc();

                self.visit(condition, symbols);

                let mut then_scope = SymbolTable::new_child(symbols);
                let (_, then_flow) = self.visit(then_block, &mut then_scope);

                let else_flow = if let Some(else_block) = else_block {
                    let mut else_scope = SymbolTable::new_child(symbols);
                    self.visit(else_block, &mut else_scope).1
                } else {
                    Flow::CONTINUE
                };

                let out_flow = if then_flow == Flow::STOP && else_flow == Flow::STOP {
                    Flow::STOP
                } else {
                    Flow::CONTINUE
                };

                self.log.indent_dec();
                (None, out_flow)
            }

            STree::WHILE_EXPR { condition, body } => {
                self.log.info("analyze_while()");
                self.log.indent_inc();

                self.visit(condition, symbols);

                self.loop_depth += 1;
                let mut local = SymbolTable::new_child(symbols);
                let _ = self.visit(body, &mut local);
                self.loop_depth -= 1;

                self.log.indent_dec();

                (None, Flow::CONTINUE)
            }

            STree::DO_WHILE { body, condition } => {
                self.log.info("analyze_do_while()");
                self.log.indent_inc();

                self.visit(condition, symbols);

                self.loop_depth += 1;
                let mut local = SymbolTable::new_child(symbols);
                let _ = self.visit(body, &mut local);
                self.loop_depth -= 1;

                self.log.indent_dec();

                (None, Flow::CONTINUE)
            }

            STree::LOOP_EXPR { condition, body } => {
                self.log.info("analyze_loop()");
                self.log.indent_inc();

                match condition.as_ref() {
                    STree::LIT_INT { .. } => {}
                    STree::ID { name } => match symbols.check_variable(name) {
                        Ok(VariableType::INT) => {}
                        Ok(other) => self.errors.push(format!(
                            "Loop condition variable '{}' must be INT, found {:?}",
                            name, other
                        )),
                        Err(e) => self.errors.push(e),
                    },
                    _ => {
                        let (ct_opt, _) = self.visit(condition, symbols);
                        let ct = ct_opt.unwrap_or(VariableType::NULL);
                        self.errors.push(format!(
                            "Loop condition must be an int literal or int variable, found {:?}",
                            ct
                        ));
                    }
                }

                self.loop_depth += 1;
                let mut local = SymbolTable::new_child(symbols);
                let _ = self.visit(body, &mut local);
                self.loop_depth -= 1;

                self.log.indent_dec();
                (None, Flow::CONTINUE)
            }

            STree::FOR_EXPR { init, condition, modifier, body } => {
                self.log.info("analyze_for()");
                self.log.indent_inc();

                let mut local = SymbolTable::new_child(symbols);

                if let Some(i) = init {
                    let _ = self.visit(i, &mut local);
                }

                let (ct_opt, _) = self.visit(condition, &mut local);
                let ct = ct_opt.unwrap_or(VariableType::NULL);
                if ct != VariableType::BOOLEAN && ct != VariableType::NULL {
                    self.errors.push(format!(
                        "For-loop condition must be Bool, found {:?}",
                        ct
                    ));
                }

                self.loop_depth += 1;

                let _ = self.visit(body, &mut local);

                if let Some(m) = modifier {
                    let _ = self.visit(m, &mut local);
                }

                self.loop_depth -= 1;

                self.log.indent_dec();
                (None, Flow::CONTINUE)
            }

            STree::FOR_EACH { variable, iterable, body } => {
                self.log.info("analyze_for_each()");
                self.log.indent_inc();

                let element_type = match iterable.as_ref() {
                    STree::RANGE { .. } => {
                        let (it_opt, _) = self.visit(iterable, symbols);
                        it_opt.unwrap_or(VariableType::NULL)
                    }

                    _ => {
                        let (it_opt, _) = self.visit(iterable, symbols);
                        match it_opt.unwrap_or(VariableType::NULL) {
                            VariableType::STRING => VariableType::CHAR,
                            other => {
                                self.errors.push(format!("Cannot iterate over type {:?}", other));
                                VariableType::NULL
                            }
                        }
                    }
                };

                let mut local = SymbolTable::new_child(symbols);
                if let Err(e) = local.declare_variable(variable.clone(), element_type, false) {
                    self.errors.push(e);
                }

                self.loop_depth += 1;
                let _ = self.visit(body, &mut local);
                self.loop_depth -= 1;

                self.log.indent_dec();
                (None, Flow::CONTINUE)
            }


            STree::BREAK => {
                self.log.info("analyze_break()");
                self.require_loop("break");
                (None, Flow::STOP)
            }

            STree::CONTINUE => {
                self.log.info("analyze_continue()");
                self.require_loop("continue");
                (None, Flow::STOP)
            }

            STree::REPEAT => {
                self.log.info("analyze_repeat()");
                self.require_loop("repeat");
                (None, Flow::STOP)
            }

            STree::RANGE { start, end, inclusive: _ } => {
                self.log.info("analyze_range()");
                self.log.indent_inc();

                let (start_type_opt, _) = self.visit(start, symbols);
                let (end_type_opt, _ ) = self.visit(end, symbols);
                let start_type = start_type_opt.unwrap_or(VariableType::NULL);
                let end_type = end_type_opt.unwrap_or(VariableType::NULL);

                if start_type != end_type {
                    self.errors.push(format!("Range bounds must have same types, currently {:?} and {:?}", start_type, end_type));
                }

                match start_type {
                    VariableType::CHAR
                    | VariableType::INT | VariableType::FLOAT => {},
                    _ => self.errors.push(format!("Range bounds must be char or numeric, currently {:?}", start_type)),
                }

                self.log.indent_dec();
                (Some(start_type), Flow::CONTINUE)
            }


            STree::EXPR { left, operator, right } => {
                self.log.info("analyze_expression()");
                self.log.indent_inc();

                let (lt_opt, _) = self.visit(left, symbols);
                let (rt_opt, _) = self.visit(right, symbols);
                let lt = lt_opt.unwrap_or(VariableType::NULL);
                let rt = rt_opt.unwrap_or(VariableType::NULL);

                let out = self.type_binary_operator(operator, lt, rt);
                self.log.indent_dec();
                (Some(out), Flow::CONTINUE)
            }

            STree::PRFX_EXPR { operator, right } => {
                self.log.info("analyze_prefix_expr()");
                self.log.indent_inc();

                let (rt_opt, _) = self.visit(right, symbols);
                let rt = rt_opt.unwrap_or(VariableType::NULL);

                let out = match operator {
                    Operator::NOT => {
                        if rt == VariableType::BOOLEAN || rt == VariableType::NULL {
                            VariableType::BOOLEAN
                        } else {
                            self.errors.push(format!("Unary NOT requires Bool, found {:?}", rt));
                            VariableType::NULL
                        }
                    }

                    Operator::NEGATIVE => {
                        if rt.is_numeric() {
                            rt
                        } else {
                            self.errors.push(format!("Unary minus requires numeric type, found {:?}", rt));
                            VariableType::NULL
                        }
                    }

                    Operator::INCREMENT | Operator::DECREMENT => {
                        match right.as_ref() {
                            STree::ID { .. } if rt.is_numeric() => rt,
                            _ => {
                                self.errors.push(format!(
                                    "Prefix {:?} requires a numeric variable",
                                    operator
                                ));
                                VariableType::NULL
                            }
                        }
                    }

                    _ => VariableType::NULL,
                };

                self.log.indent_dec();
                (Some(out), Flow::CONTINUE)
            }

            STree::PTFX_EXPR { left, operator } => {
                self.log.info("analyze_postfix_expr()");
                self.log.indent_inc();

                let (lt_opt, _) = self.visit(left, symbols);
                let lt = lt_opt.unwrap_or(VariableType::NULL);

                let out = match operator {
                    Operator::INCREMENT | Operator::DECREMENT => {
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
                };

                self.log.indent_dec();
                (Some(out), Flow::CONTINUE)
            }

            STree::CALL { path, arguments } => {
                self.log.info("analyze_function_call()");

                let arg_types: Vec<VariableType> = arguments
                    .iter()
                    .map(|a| self.visit(a, symbols).0.unwrap_or(VariableType::NULL))
                    .collect();

                if let Some(sig) = self.functions.get_mut(path) {
                    sig.called = true;

                    if !sig.parameters.is_empty() && sig.parameters.len() != arg_types.len() {
                        self.errors.push(format!(
                            "Function '{}': expects {} args but {} provided",
                            path.join("."),
                            sig.parameters.len(),
                            arg_types.len()
                        ));
                    }

                    if path.join(".") == "System.exit" {
                        return (Some(sig.return_type.clone()), Flow::STOP);
                    }

                    return (Some(sig.return_type.clone()), Flow::CONTINUE);
                }

                self.errors.push(format!(
                    "Call to unknown function '{}'",
                    path.join(".")
                ));
                (None, Flow::CONTINUE)
            }

            STree::CAST { expression, target } => {
                self.log.info("analyze_cast()");
                self.log.indent_inc();

                let (src_opt, _) = self.visit(expression, symbols);
                let src = src_opt.unwrap_or(VariableType::NULL);

                let out = self.analyze_cast(src, target.clone());

                self.log.indent_dec();
                (Some(out), Flow::CONTINUE)
            }



            STree::ID { name } => {
                match symbols.mark_used(name) {
                    Ok(t) => (Some(t), Flow::CONTINUE),
                    Err(e) => {
                        self.errors.push(e);
                        (None, Flow::CONTINUE)
                    }
                }
            }


            STree::LIT_INT { .. } => (Some(VariableType::INT), Flow::CONTINUE),
            STree::LIT_FLOAT { .. } => (Some(VariableType::FLOAT), Flow::CONTINUE),
            STree::LIT_BOOL { .. } => (Some(VariableType::BOOLEAN), Flow::CONTINUE),
            STree::LIT_CHAR { .. } => (Some(VariableType::CHAR), Flow::CONTINUE),
            STree::LIT_STRING { .. } => (Some(VariableType::STRING), Flow::CONTINUE),
            STree::NULL { .. } => (Some(VariableType::NULL), Flow::CONTINUE),

            STree::BLANK_STMT => (None, Flow::CONTINUE)
        }
    }
}


impl Analyzer {
    fn collect_function_signatures(&mut self, tree: &STree) {
        if let STree::START { functions } = tree {
            for func in functions {
                if let STree::FUNCTION { name, params, return_type, .. } = func {

                    let param_types = params
                        .iter()
                        .map(|(_, t)| t.clone())
                        .collect();

                    let key = vec![name.clone()];
                    let called = name == "main";

                    match self.functions.entry(key) {
                        Entry::Occupied(_) => {
                            self.errors.push(format!(
                                "Function '{}' already declared",
                                name
                            ));
                        }
                        Entry::Vacant(v) => {
                            v.insert(FunctionSignature {
                                parameters: param_types,
                                return_type: return_type.clone(),
                                called,
                            });
                        }
                    }
                }
            }
        }
    }


    fn has_return(&self, node: &STree) -> bool {
        match node {
            STree::RETURN_STMT { .. } => true,
            STree::BLOCK { statements } => statements.iter().any(|s| self.has_return(s)),
            STree::IF_EXPR { then_block, else_block, .. } => {
                let then_has = self.has_return(then_block);
                let else_has = else_block.as_ref().map(|b| self.has_return(b)).unwrap_or(false);
                then_has || else_has
            }
            STree::FUNCTION { body, .. } => self.has_return(body),
            STree::START { functions } => functions.iter().any(|f| self.has_return(f)),
            _ => false,
        }
    }

    fn type_binary_operator(&mut self, operator: &Operator, left: VariableType, right: VariableType) -> VariableType {


        if matches!(operator, Operator::EQUAL | Operator::NOT_EQUAL) {
            return VariableType::BOOLEAN;
        }

        if *operator == Operator::NULL_COAL {
            if left == VariableType::NULL {
                return right;
            }

            if right == VariableType::NULL {
                return left;
            }

            if left == right {
                return left;
            }

            self.errors.push(format!(
                "Null coalescing requires compatible types, found {:?} ?? {:?}",
                left, right
    ));
        }

        if left == VariableType::NULL || right == VariableType::NULL {
            self.errors.push(format!(
                "Possible null operand for {:?}: {:?} and {:?}",
                operator, left, right
            ));
            return VariableType::NULL;
        }


        match operator {
            Operator::ADD | Operator::SUBTRACT | Operator::MULTIPLY | Operator::DIVIDE
            | Operator::REMAINDER | Operator::POWER | Operator::ROOT => {
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

            Operator::LESS_THAN | Operator::GREATER_THAN | Operator::NOT_GREATER_THAN | Operator::NOT_LESS_THAN => {
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

            Operator::AND | Operator::OR | Operator::XOR => {
                if left == VariableType::BOOLEAN && right == VariableType::BOOLEAN {
                    VariableType::BOOLEAN
                } else {
                    self.errors.push(format!("Logical operator {:?} requires Bool operands", operator));
                    VariableType::NULL
                }
            }

            _ => VariableType::NULL,
        }

    }

    fn require_loop(&mut self, keyword: &str) {
        if self.loop_depth == 0 {
            self.errors.push(format!("'{}' used outside of a loop", keyword));
        }
    }

    fn is_assignable(&self, from: VariableType, to: VariableType) -> bool {
        if from == VariableType::NULL {
            return true;
        }
        from == to
    }

    fn analyze_match_pattern(&mut self, pattern: &STree, scrutinee_type: VariableType, scope: &mut SymbolTable) {
        match pattern {
            STree::ID { name } => {
                scope.declare_variable(name.clone(), scrutinee_type, false).ok();
            }

            STree::LIT_INT { .. } if scrutinee_type == VariableType::INT => {}
            STree::LIT_FLOAT { .. } if scrutinee_type == VariableType::FLOAT => {}
            STree::LIT_BOOL { .. } if scrutinee_type == VariableType::BOOLEAN => {}
            STree::LIT_CHAR { .. } if scrutinee_type == VariableType::CHAR => {}
            STree::LIT_STRING { .. } if scrutinee_type == VariableType::STRING => {}
            STree::NULL => {},
            STree::RANGE { start, end, .. } => {
                let (st_opt, _) = self.visit(start, scope);
                let (et_opt, _) = self.visit(end, scope);
                let st = st_opt.unwrap_or(VariableType::NULL);
                let et = et_opt.unwrap_or(VariableType::NULL);

                if st != scrutinee_type || et != scrutinee_type {
                    self.errors.push(format!("Range pattern bounds must match scrutinee type {:?}",scrutinee_type));
                }

                match scrutinee_type {
                    VariableType::INT | VariableType::FLOAT | VariableType::CHAR => {}
                    _ => self.errors.push(format!(
                        "Range patterns only valid for INT, FLOAT, or CHAR; got {:?}",
                        scrutinee_type
                    )),
                }
            },
            STree::DEFAULT  => {}


            _ => self.errors.push(format!(
                "Pattern {:?} does not match type {:?}",
                pattern, scrutinee_type
            )),
        }
    }

    fn analyze_cast(&mut self, from: VariableType, to: VariableType) -> VariableType {

        // Identity cast
        if from == to {
            return to;
        }

        // Null handling (decide policy)
        if from == VariableType::NULL {
            self.errors.push(format!(
                "Cannot cast NULL to {:?}",
                to
            ));
            return VariableType::NULL;
        }

        match (from.clone(), to.clone()) {
            // Numeric casts
            (VariableType::INT, VariableType::FLOAT) => VariableType::FLOAT,
            (VariableType::FLOAT, VariableType::INT) => VariableType::INT,

            (VariableType::BOOLEAN, VariableType::INT) => VariableType::INT,
            (VariableType::BOOLEAN, VariableType::FLOAT) => VariableType::FLOAT,

            (VariableType::INT, VariableType::BOOLEAN) => VariableType::BOOLEAN,
            (VariableType::FLOAT, VariableType::BOOLEAN) => VariableType::BOOLEAN,

            (VariableType::CHAR, VariableType::INT) => VariableType::INT,
            (VariableType::INT, VariableType::CHAR) => VariableType::CHAR,

            // String casts (very strict)
            (_, VariableType::STRING) | (VariableType::STRING, _) => {
                self.errors.push(format!(
                    "Casting between STRING and {:?} is not allowed",
                    from
                ));
                VariableType::NULL
            }

            // Everything else
            _ => {
                self.errors.push(format!(
                    "Illegal cast from {:?} to {:?}",
                    from, to
                ));
                VariableType::NULL
            }
        }
    }


}
