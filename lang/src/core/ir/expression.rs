use inkwell::values::BasicValueEnum;
use crate::core::converter::stree::STree;
use crate::core::ir::codegen::CodeGen;
use crate::core::lexer::token_type::TokenType;

impl<'ctx> CodeGen<'ctx> {

    pub fn compile_expression(&mut self, node: &STree) -> Result<BasicValueEnum<'ctx>, String> {
        self.logger.info("compile_expression()");
        self.logger.info(&format!("compile_expression node = {:?}", node));

        match node {
            STree::LIT_INT { value } => Ok(BasicValueEnum::IntValue(self.context.i32_type().const_int(*value as u64, false))),
            STree::LIT_FLOAT { value } => Ok(BasicValueEnum::FloatValue(self.context.f32_type().const_float(*value as f64))),

            STree::LIT_CHAR { value } => Ok(BasicValueEnum::IntValue(self.context.i8_type().const_int(*value as u64, false))),
            STree::LIT_STRING { value } => {
                let str_val = self.builder.build_global_string_ptr(value, "str").unwrap();
                Ok(str_val.as_pointer_value().into())
            },

            STree::LIT_BOOL { value } => Ok(BasicValueEnum::IntValue(self.context.bool_type().const_int(*value as u64, false))),

            STree::NULL => {
                let null_ptr = self.context
                    .i8_type()
                    .ptr_type(inkwell::AddressSpace::default())
                    .const_null();

                Ok(null_ptr.into())
            },

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

                        _ => Err("Unsupported Type for Not, must be Int/Bool".to_string())
                    } ,
                    _ => Err("Unsupported prefix op".into())
                }
            },

            STree::EXPR { left, operator, right } => {
                let lhs = self.compile_expression(left)?;
                let rhs = self.compile_expression(right)?;

                match (lhs, rhs) {
                    (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                        self.compile_int_expression(l, r, operator)
                    },
                    (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => {
                        self.compile_float_expression(l, r, operator)
                    },
                    _ => Err(format!("Type mismatch in expression: {:?}", operator)),
                }
            },

            STree::FUNCTION_CALL { callee, args } => {
                match self.compile_function_call(callee, args)? {
                    Some(v) => Ok(v),
                    None => Err("Void function cannot be used in expression".into()),
                }
            },

            _ => Err(format!("Invalid Expression Node: {:?}", node)),
        }
    }

}