use std::collections::HashMap;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicMetadataTypeEnum, BasicType};
use inkwell::values::{FunctionValue, IntValue, PointerValue};
use inkwell::IntPredicate;

use crate::core::converter::stree::STree;
use crate::core::lexer::token_type::TokenType;
use crate::core::util::logger::Logger;


pub struct CodeGen<'ctx> {
    logger: Logger,
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    // Map from variable names to their stack allocations
    variables: HashMap<String, PointerValue<'ctx>>,
    // Map from function names to LLVM functions
    functions: HashMap<String, FunctionValue<'ctx>>,
    // Current function being compiled
    current_fn: Option<FunctionValue<'ctx>>,
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
            let alloca = self.create_entry_block_alloca(function, param_name);
            self.builder.build_store(alloca, param_val).unwrap();
            self.variables.insert(param_name.clone(), alloca);
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

    fn compile_statement(&mut self, node: &STree) -> Result<Option<IntValue<'ctx>>, String> {
        self.logger.info("compile_statement()");
        self.logger.indent_inc();

        match node {

            STree::RETURN_STMT { expression } => {
                let val = if let Some(expr) = expression {
                    self.compile_expression(expr)?
                } else {
                    self.context.i32_type().const_int(0, false)
                };
                self.builder.build_return(Some(&val)).unwrap();
                self.logger.indent_dec();
                Ok(Some(val))
            },

            STree::VAR_STMT { id, expression, .. } => {
                let val = self.compile_expression(expression)?;
                let func = self.current_fn.unwrap();
                let alloca = self.create_entry_block_alloca(func, id);
                self.builder.build_store(alloca, val).unwrap();
                self.variables.insert(id.clone(), alloca);
                self.logger.indent_dec();
                Ok(Some(val))
            },

            STree::ASSIGN_STMT { id, expression } => {
                let val = self.compile_expression(expression)?;
                let ptr = self.variables.get(id).ok_or(format!("Undefined var {}", id))?;
                self.builder.build_store(*ptr, val).unwrap();
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

    fn compile_expression(&mut self, node: &STree) -> Result<IntValue<'ctx>, String> {
        self.logger.info("compile_expression()");
        self.logger.info(&format!("compile_expression node = {:?}", node));

        match node {
            STree::LIT_INT { value } => Ok(self.context.i32_type().const_int(*value as u64, false)),
            STree::LIT_FLOAT { value } => Err("Float literals not supported in int context yet".into()),

            STree::LIT_BOOL { value } => Ok(self.context.bool_type().const_int(*value as u64, false)),

            STree::ID { name } => {
                let ptr = self.variables.get(name).ok_or(format!("Undefined var {}", name))?;
                let v = self.builder.build_load(self.context.i32_type(), *ptr, name).unwrap();
                Ok(v.into_int_value())
            },

            STree::PRFX_EXPR { operator, right } => {
                let val = self.compile_expression(right)?;
                match operator {
                    TokenType::DASH => Ok(self.builder.build_int_neg(val,"neg").unwrap()),
                    TokenType::NOT  => Ok(self.builder.build_not(val,"not").unwrap()),
                    _ => Err("Unsupported prefix op".into())
                }
            },

            STree::EXPR { left, operator, right } => {
                let lhs = self.compile_expression(left)?;
                let rhs = self.compile_expression(right)?;

                match operator {
                    TokenType::PLUS  => Ok(self.builder.build_int_add(lhs, rhs,"add").unwrap()),
                    TokenType::DASH => Ok(self.builder.build_int_sub(lhs, rhs,"sub").unwrap()),
                    TokenType::STAR  => Ok(self.builder.build_int_mul(lhs, rhs,"mul").unwrap()),
                    TokenType::SLASH => Ok(self.builder.build_int_signed_div(lhs, rhs,"div").unwrap()),
                    TokenType::PERCENT => Ok(self.builder.build_int_signed_rem(lhs, rhs,"mod").unwrap()),

                    TokenType::LESS => {
                        let cmp = self.builder
                                .build_int_compare(IntPredicate::SLT, lhs, rhs, "lt")
                                .unwrap();
                        Ok(self.builder
                                .build_int_z_extend(cmp, self.context.i32_type(), "ext")
                                .unwrap()
                        )
                    },

                    TokenType::GREATER => {
                        let cmp = self.builder
                                .build_int_compare(IntPredicate::SGT, lhs, rhs, "gt")
                                .unwrap();
                        Ok(self.builder
                                .build_int_z_extend(cmp, self.context.i32_type(), "ext")
                                .unwrap()
                        )
                    },

                    TokenType::LESS_EQUAL => {
                        let cmp = self.builder
                                .build_int_compare(IntPredicate::SLE, lhs, rhs, "le")
                                .unwrap();
                        Ok(self.builder
                                .build_int_z_extend(cmp, self.context.i32_type(), "ext")
                                .unwrap()
                        )
                    },

                    TokenType::GREATER_EQUAL => {
                        let cmp = self.builder
                                .build_int_compare(IntPredicate::SGE, lhs, rhs, "ge")
                                .unwrap();
                        Ok(self.builder
                                .build_int_z_extend(cmp, self.context.i32_type(), "ext")
                                .unwrap()
                        )
                    },

                    TokenType::EQUAL => {
                        let cmp = self.builder
                                .build_int_compare(IntPredicate::EQ, lhs, rhs, "eq")
                                .unwrap();
                        Ok(self.builder
                                .build_int_z_extend(cmp, self.context.i32_type(), "ext")
                                .unwrap()
                        )
                    },

                    TokenType::NOT_EQUAL => {
                        let cmp = self.builder
                                .build_int_compare(IntPredicate::NE, lhs, rhs, "ne")
                                .unwrap();
                        Ok(self.builder
                                .build_int_z_extend(cmp, self.context.i32_type(), "ext")
                                .unwrap()
                        )
                    },

                    _ => Err("Unsupported binary op".into())
                }
            },

            _ => Err("Invalid Expression Node".to_string())
        }

    }

    fn llvm_type(&self, ty: &TokenType) -> Result<inkwell::types::BasicTypeEnum<'ctx>, String> {
        match ty {
            TokenType::INT   => Ok(self.context.i32_type().into()),
            TokenType::FLOAT => Ok(self.context.f64_type().into()),
            TokenType::BOOLEAN  => Ok(self.context.bool_type().into()),
            TokenType::STRING => Ok(self.context.i8_type().ptr_type(inkwell::AddressSpace::default()).into()),

            // TokenType::NULL => Ok(self.context.void_type().into()),

            _ => Err(format!("Unsupported type in codegen: {:?}", ty)),
        }
    }

    fn create_entry_block_alloca(&self, function: FunctionValue<'ctx>, name: &str) -> PointerValue<'ctx> {

        let builder = self.context.create_builder();
        let entry = function.get_first_basic_block().unwrap();

        match entry.get_first_instruction() {
            Some(inst) => builder.position_before(&inst),
            None => builder.position_at_end(entry),
        }

        let i32t = self.context.i32_type(); // default storage
        builder.build_alloca(i32t, name).unwrap()
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
