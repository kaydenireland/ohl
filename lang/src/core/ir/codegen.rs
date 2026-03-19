use std::collections::HashMap;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum};
use inkwell::values::{FunctionValue, PointerValue};
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





    pub fn llvm_type(&self, ty: &TokenType) -> Result<inkwell::types::BasicTypeEnum<'ctx>, String> {
        match ty {
            TokenType::INT => Ok(self.context.i32_type().into()),
            TokenType::FLOAT => Ok(self.context.f32_type().into()),
            TokenType::BOOLEAN => Ok(self.context.bool_type().into()),
            TokenType::STRING => Ok(self.context.i8_type().ptr_type(inkwell::AddressSpace::default()).into()),
            TokenType::CHAR => Ok(self.context.i8_type().into()),

            // TokenType::NULL => Ok(self.context.void_type().into()),

            _ => Err(format!("Unsupported type in codegen: {:?}", ty)),
        }
    }

    pub fn create_entry_block_alloca(&self, function: FunctionValue<'ctx>, name: &str, ty: BasicTypeEnum<'ctx>) -> PointerValue<'ctx> {

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
