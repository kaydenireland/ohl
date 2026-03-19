use inkwell::IntPredicate;
use inkwell::values::{BasicValueEnum, IntValue};
use crate::core::ir::codegen::CodeGen;
use crate::core::lexer::token_type::TokenType;

impl<'ctx> CodeGen<'ctx> {
    
    pub fn int_cmp(&mut self, pred: IntPredicate, l: IntValue<'ctx>, r: IntValue<'ctx>, name: &str) -> Result<BasicValueEnum<'ctx>, String> {
        let cmp = self.builder.build_int_compare(pred, l, r, name).unwrap();
        Ok(cmp.into())
    }
    
    pub fn compile_int_expression(&mut self, l: IntValue<'ctx>, r: IntValue<'ctx>, op: &TokenType) -> Result<BasicValueEnum<'ctx>, String> {

        if l.get_type() == self.context.bool_type() || r.get_type() == self.context.bool_type() {
            return Err(format!(
                "Type error: boolean value cannot be used in arithmetic expression '{:?}'", op
            ));
        }

        let l = self.promote_int(l);
        let r = self.promote_int(r);

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

    fn promote_int(&self, v: IntValue<'ctx>) -> IntValue<'ctx> {
        if v.get_type() == self.context.i8_type() {
            // char to int
            self.builder
                .build_int_z_extend(v, self.context.i32_type(), "char_promote")
                .unwrap()
        } else {
            v
        }
    }
    
}