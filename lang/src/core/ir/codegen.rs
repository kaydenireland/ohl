use std::collections::HashMap;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum};
use inkwell::values::{BasicValueEnum, FloatValue, FunctionValue, IntValue, PointerValue};
use inkwell::{FloatPredicate, IntPredicate};

use crate::core::converter::stree::STree;
use crate::core::lexer::token_type::TokenType;
use crate::core::util::logger::Logger;


pub struct CodeGen<'ctx> {
    pub logger: Logger,
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    // Map from variable names to their stack allocations
    pub variables: HashMap<String, (PointerValue<'ctx>, BasicTypeEnum<'ctx>)>,
    // Map from function names to LLVM functions
    pub functions: HashMap<String, FunctionValue<'ctx>>,
    // Current function being compiled
    pub current_fn: Option<FunctionValue<'ctx>>,
}

impl<'ctx> CodeGen<'ctx> {

    pub fn new(context: &'ctx Context, module_name: &str, _debug: bool) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();

        CodeGen {
            logger: Logger::new(_debug),
            context,
            module,
            builder,
            variables: HashMap::new(),
            functions: HashMap::new(),
            current_fn: None,
        }
    }
    

    // Compile a program and return the module
    pub fn compile(&mut self, tree: &STree) -> Result<(), String> {
        self.logger.info("compile()");
        self.logger.indent_inc();
        
        self.declare_printf();
        

        if let STree::START { functions } = tree {
            // First pass: declare all functions
            for func in functions {
                if let STree::FUNCTION { function_type: _, return_type, name, params, .. } = func {
                    self.declare_function(name, params, return_type)?;
                }
            }

            // Second pass: compile function bodies only (not top-level expressions)
            for func in functions {
                if let STree::FUNCTION { function_type:_, return_type: _, name, params, body } = func {
                    self.compile_function(name, params, body)?;
                }
            }
        }

        self.logger.indent_dec();
        self.logger.info("finished compile()\n");

        // Verify module
        self.module
            .verify()
            .map_err(|e| format!("Module verification failed: {}", e.to_string()))?;

        Ok(())
    }

    fn declare_function(&mut self, name: &str, params: &Vec<(String, TokenType)>, return_type: &TokenType) -> Result<FunctionValue<'ctx>, String> {
        self.logger.info("declare_function()");
        self.logger.indent_inc();

        let ret_type = self.llvm_type(return_type)?;

        let param_types: Vec<BasicMetadataTypeEnum> = params
            .iter()
            .map(|(_, t)| self.llvm_type(t).unwrap().into())
            .collect();

        let fn_type = ret_type.fn_type(&param_types, false);
        let function = self.module.add_function(name, fn_type, None);

        // Set parameter names
        for (i, (param_name, _)) in params.iter().enumerate() {
            function
                .get_nth_param(i as u32)
                .unwrap()
                .set_name(param_name);
        }

        self.functions.insert(name.to_string(), function);
        
        self.logger.indent_dec();

        Ok(function)
    }

    fn compile_function(&mut self, name: &str, params: &Vec<(String, TokenType)>, body: &Box<STree>) -> Result<(), String> {
        self.logger.info("compile_function()");
        self.logger.indent_inc();

        let function = *self.functions
            .get(name)
            .ok_or(format!("Function {} not declared", name))?;

        let entry = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry);

        self.current_fn = Some(function);
        self.variables.clear();

        // params
        for (i, (param_name, _)) in params.iter().enumerate() {
            let param_val = function.get_nth_param(i as u32).unwrap();
            let param_type = function.get_nth_param(i as u32).unwrap().get_type();
            let alloca = self.create_entry_block_alloca(function, param_name, param_type);

            self.builder.build_store(alloca, param_val).unwrap();
            self.variables.insert(param_name.clone(), (alloca, param_type));
        }

        // body
        let STree::BLOCK { statements } = body.as_ref() else {
            return Err(format!("Function {} body must be BLOCK", name));
        };

        for stmt in statements {
            self.compile_statement(stmt)?;
        }

        // implicit return 0 if none
        if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
            let zero = self.context.i32_type().const_int(0, false);
            self.builder.build_return(Some(&zero)).unwrap();
        }

        self.logger.indent_dec();

        Ok(())
    }

    fn compile_statement(&mut self, node: &STree) -> Result<Option<BasicValueEnum<'ctx>>, String> {
        self.logger.info("compile_statement()");
        self.logger.indent_inc();

        match node {

            STree::RETURN_STMT { expression } => {
                let val = if let Some(expr) = expression {
                    self.compile_expression(expr)?
                } else {
                    BasicValueEnum::IntValue(self.context.i32_type().const_int(0, false))
                };
                self.builder.build_return(Some(&val)).unwrap();
                self.logger.indent_dec();
                Ok(Some(val))
            },

            STree::VAR_STMT { id, expression, var_type, .. } => {
                let val = self.compile_expression(expression)?;
                let func = self.current_fn.unwrap();

                let llvm_type = self.llvm_type(var_type)?;
                let alloca = self.create_entry_block_alloca(func, id, llvm_type);

                self.builder.build_store(alloca, val).unwrap();
                self.variables.insert(id.clone(), (alloca, llvm_type));
                self.logger.indent_dec();
                Ok(Some(val))
            },

            STree::ASSIGN_STMT { id, expression } => {
                let val = self.compile_expression(expression)?;
                let (ptr, expected_typ) = self.variables.get(id).ok_or(format!("Undefined var {}", id))?;

                let store_val = match (val, expected_typ) {
                    (BasicValueEnum::IntValue(i), inkwell::types::BasicTypeEnum::IntType(t)) 
                        if *t == self.context.bool_type() => {
                            self.builder
                                .build_int_truncate(i, self.context.bool_type(), "to_bool")
                                .unwrap()
                                .into()
                    },
                    _ => val,
                };

                self.builder.build_store(*ptr, store_val).unwrap();
                self.logger.indent_dec();
                Ok(Some(val))
            },

            STree::BLOCK { statements } => {
                let mut last = None;
                for s in statements {
                    last = self.compile_statement(s)?;
                }
                self.logger.indent_dec();
                Ok(last)
            },

            STree::VAR_TYPE { .. } => Ok(None),
            STree::BLANK_STMT => Ok(None),
            STree::NULL => Ok(None),

            STree::PRINT { expression } => {
                let val = self.compile_expression(expression)?;
                self.build_print(&[val])?;
                Ok(None)
            },
            STree::LIT_INT { .. }
            | STree::LIT_FLOAT { .. }
            | STree::LIT_BOOL { .. }
            | STree::ID { .. }
            | STree::EXPR { .. }
            | STree::PRFX_EXPR { .. } => {
                let v = self.compile_expression(node)?;
                Ok(Some(v))
            }

            _ => {
                self.logger.indent_dec();
                Err(format!("Invalid statement node: {:?}", node))
            }
        }
    }

    fn compile_expression(&mut self, node: &STree) -> Result<BasicValueEnum<'ctx>, String> {
        self.logger.info("compile_expression()");
        self.logger.info(&format!("compile_expression node = {:?}", node));

        match node {
            STree::LIT_INT { value } => Ok(BasicValueEnum::IntValue(self.context.i32_type().const_int(*value as u64, false))),
            STree::LIT_FLOAT { value } => Ok(BasicValueEnum::FloatValue(self.context.f32_type().const_float(*value as f64))),

            STree::LIT_BOOL { value } => Ok(BasicValueEnum::IntValue(self.context.bool_type().const_int(*value as u64, false))),

            STree::ID { name } => {
                let (ptr, ty) = self.variables.get(name).ok_or(format!("Undefined var {}", name))?;
                let v = self.builder.build_load(*ty, *ptr, name).unwrap();
                Ok(v)
            },

            STree::PRFX_EXPR { operator, right } => {
                let val = self.compile_expression(right)?;
                match operator {
                    TokenType::DASH => match val {
                        BasicValueEnum::IntValue(i) => Ok(self.builder.build_int_neg(i,"neg").unwrap().into()),
                        BasicValueEnum::FloatValue(f) => Ok(self.builder.build_float_neg(f, "neg").unwrap().into()),

                        _ => Err("Unsuported Type for Negation".to_string())
                    },
                    TokenType::NOT  => match val {
                        BasicValueEnum::IntValue(i) => Ok(self.builder.build_not(i,"not").unwrap().into()),

                        _ => Err("Unsuported Type for Not, must be Int/Bool".to_string())
                    } ,
                    _ => Err("Unsupported prefix op".into())
                }
            },

            STree::EXPR { left, operator, right } => {
                let lhs = self.compile_expression(left)?;
                let rhs = self.compile_expression(right)?;

                match (lhs, rhs) {
                    (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                        self.compile_int_expr(l, r, operator)
                    },
                    (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => {
                        self.compile_float_expr(l, r, operator)
                    },
                    _ => Err(format!("Type mismatch in expression: {:?}", operator)),
                }
            },

            _ => Err(format!("Invalid Expression Node: {:?}", node)),
        }
    }

    fn compile_int_expr(&mut self, l: IntValue<'ctx>, r: IntValue<'ctx>, op: &TokenType) -> Result<BasicValueEnum<'ctx>, String> {

        let l = if l.get_type() == self.context.bool_type() {
            self.builder.build_int_z_extend(l, self.context.i32_type(), "bext").unwrap()
        } else { l };
        let r = if r.get_type() == self.context.bool_type() {
            self.builder.build_int_z_extend(r, self.context.i32_type(), "bext").unwrap()
        } else { r };

        match op {
            TokenType::PLUS    => Ok(self.builder.build_int_add(l, r, "add").unwrap().into()),
            TokenType::DASH    => Ok(self.builder.build_int_sub(l, r, "sub").unwrap().into()),
            TokenType::STAR    => Ok(self.builder.build_int_mul(l, r, "mul").unwrap().into()),
            TokenType::SLASH   => Ok(self.builder.build_int_signed_div(l, r, "div").unwrap().into()),
            TokenType::PERCENT => Ok(self.builder.build_int_signed_rem(l, r, "mod").unwrap().into()),

            TokenType::LESS           => self.int_cmp(IntPredicate::SLT, l, r, "lt"),
            TokenType::GREATER        => self.int_cmp(IntPredicate::SGT, l, r, "gt"),
            TokenType::LESS_EQUAL     => self.int_cmp(IntPredicate::SLE, l, r, "le"),
            TokenType::GREATER_EQUAL  => self.int_cmp(IntPredicate::SGE, l, r, "ge"),
            TokenType::EQUAL          => self.int_cmp(IntPredicate::EQ,  l, r, "eq"),
            TokenType::NOT_EQUAL      => self.int_cmp(IntPredicate::NE,  l, r, "ne"),

            _ => Err(format!("Unsupported int operator: {:?}", op)),
        }
    }

    fn compile_float_expr(&mut self, l: FloatValue<'ctx>, r: FloatValue<'ctx>, op: &TokenType) -> Result<BasicValueEnum<'ctx>, String> {
        match op {
            TokenType::PLUS  => Ok(self.builder.build_float_add(l, r, "fadd").unwrap().into()),
            TokenType::DASH  => Ok(self.builder.build_float_sub(l, r, "fsub").unwrap().into()),
            TokenType::STAR  => Ok(self.builder.build_float_mul(l, r, "fmul").unwrap().into()),
            TokenType::SLASH => Ok(self.builder.build_float_div(l, r, "fdiv").unwrap().into()),

            TokenType::LESS          => self.float_cmp(FloatPredicate::OLT, l, r, "flt"),
            TokenType::GREATER       => self.float_cmp(FloatPredicate::OGT, l, r, "fgt"),
            TokenType::LESS_EQUAL    => self.float_cmp(FloatPredicate::OLE, l, r, "fle"),
            TokenType::GREATER_EQUAL => self.float_cmp(FloatPredicate::OGE, l, r, "fge"),
            TokenType::EQUAL         => self.float_cmp(FloatPredicate::OEQ, l, r, "feq"),
            TokenType::NOT_EQUAL     => self.float_cmp(FloatPredicate::ONE, l, r, "fne"),

            _ => Err(format!("Unsupported float operator: {:?}", op)),
        }
    }

    fn llvm_type(&self, ty: &TokenType) -> Result<inkwell::types::BasicTypeEnum<'ctx>, String> {
        match ty {
            TokenType::INT   => Ok(self.context.i32_type().into()),
            TokenType::FLOAT => Ok(self.context.f32_type().into()),
            TokenType::BOOLEAN  => Ok(self.context.bool_type().into()),
            TokenType::STRING => Ok(self.context.i8_type().ptr_type(inkwell::AddressSpace::default()).into()),

            // TokenType::NULL => Ok(self.context.void_type().into()),

            _ => Err(format!("Unsupported type in codegen: {:?}", ty)),
        }
    }

    fn int_cmp(&mut self, pred: IntPredicate, l: IntValue<'ctx>, r: IntValue<'ctx>, name: &str) -> Result<BasicValueEnum<'ctx>, String> {
        let cmp = self.builder.build_int_compare(pred, l, r, name).unwrap();
        Ok(cmp.into())
    }

    fn float_cmp(&mut self, pred: FloatPredicate, l: FloatValue<'ctx>, r: FloatValue<'ctx>, name: &str) -> Result<BasicValueEnum<'ctx>, String> {
        let cmp = self.builder.build_float_compare(pred, l, r, name).unwrap();
        Ok(cmp.into())
    }

    fn create_entry_block_alloca(&self, function: FunctionValue<'ctx>, name: &str, ty: BasicTypeEnum<'ctx>) -> PointerValue<'ctx> {

        let builder = self.context.create_builder();
        let entry = function.get_first_basic_block().unwrap();

        match entry.get_first_instruction() {
            Some(inst) => builder.position_before(&inst),
            None => builder.position_at_end(entry),
        }

        builder.build_alloca(ty, name).unwrap()
    }

    // Get the compiled module
    pub fn get_module(&self) -> &Module<'ctx> {
        &self.module
    }

    // Print LLVM IR to string
    pub fn print_ir(&self) -> String {
        self.module.print_to_string().to_string()
    }

}
