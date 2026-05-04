use inkwell::FloatPredicate;
use inkwell::values::{BasicValueEnum, FloatValue};
use crate::core::ir::codegen::CodeGen;
use crate::core::lexer::token_type::TokenType;

impl<'ctx> CodeGen<'ctx> {
    
    pub fn float_cmp(&mut self, pred: FloatPredicate, l: FloatValue<'ctx>, r: FloatValue<'ctx>, name: &str) -> Result<BasicValueEnum<'ctx>, String> {
        let cmp = self.builder.build_float_compare(pred, l, r, name).unwrap();
        Ok(cmp.into())
    }
    
    pub fn compile_float_expression(&mut self, l: FloatValue<'ctx>, r: FloatValue<'ctx>, op: &TokenType) -> Result<BasicValueEnum<'ctx>, String> {

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
    
}