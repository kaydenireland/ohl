use inkwell::types::{BasicMetadataTypeEnum, BasicType};
use inkwell::values::{BasicValueEnum, FunctionValue};
use crate::core::converter::stree::STree;
use crate::core::ir::codegen::CodeGen;
use crate::core::lexer::token_type::TokenType;

impl<'ctx> CodeGen<'ctx> {
    
    pub fn declare_function(&mut self, name: &str, params: &Vec<(String, TokenType)>, return_type: &TokenType) -> Result<FunctionValue<'ctx>, String> {
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

    pub fn compile_function(&mut self, name: &str, params: &Vec<(String, TokenType)>, body: &Box<STree>) -> Result<(), String> {
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

    pub fn compile_function_call(&mut self, callee: &Box<STree>, args: &Vec<STree>) -> Result<BasicValueEnum<'ctx>, String> {

        let func_name = match callee.as_ref() {
            STree::ID { name } => name,
            _ => return Err("Only simple function calls supported (no methods yet)".into()),
        };

        let function = *self.functions
            .get(func_name)
            .ok_or(format!("Undefined function '{}'", func_name))?;

        // Compile arguments
        let mut compiled_args = Vec::new();

        for arg in args {
            let val = self.compile_expression(arg)?;
            compiled_args.push(val.into());
        }

        // Build call
        let call = self.builder
            .build_call(function, &compiled_args, "calltmp")
            .unwrap();

        match function.get_type().get_return_type() {
            Some(_) => {
                let call_site = self.builder
                    .build_call(function, &compiled_args, "calltmp")
                    .unwrap();

                let value = match call_site.try_as_basic_value() {
                    inkwell::values::ValueKind::Basic(v) => v,
                    inkwell::values::ValueKind::Instruction(_) => {
                        return Err("Expected function to return a value".into())
                    }
                };

                Ok(value)
            }
            None => Err("Void functions not supported in expressions yet".into()),
        }
    }
    
}