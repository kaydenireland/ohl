use crate::{indent_dec, indent_inc, info};
use crate::oil::intermediate::instruction::IntermediateInstruction;
use crate::oil::intermediate::operator::{IntermediateBinaryOperator, IntermediateUnaryOperator, Value};
use crate::oil::lexer::token::TokenType;
use crate::oil::parser::parser::Parser;

impl Parser {

    pub fn parse_return(&mut self) -> Result<IntermediateInstruction, String> {
        info!("parse_return()");
        indent_inc!();

        self.expect(TokenType::RET);
        let val = self.parse_value()?;

        indent_dec!();
        Ok(IntermediateInstruction::RETURN(val))
    }

    pub fn parse_copy(&mut self) -> Result<IntermediateInstruction, String> {
        info!("parse_copy()");
        indent_inc!();

        self.expect(TokenType::COPY);
        let src = self.parse_value()?;
        let dst = self.parse_value()?;

        indent_dec!();
        Ok(IntermediateInstruction::COPY { src, dst })
    }

    pub fn parse_jump(&mut self) -> Result<IntermediateInstruction, String> {
        info!("parse_jump()");
        indent_inc!();

        self.expect(TokenType::JMP);
        let target = self.parse_identifier()?;

        indent_dec!();
        Ok(IntermediateInstruction::JUMP { target })
    }

    pub fn parse_jump_conditional(&mut self, zero: bool) -> Result<IntermediateInstruction, String> {
        info!("parse_jump_conditional()");
        indent_inc!();

        if zero {
            self.expect(TokenType::JMPI);
        } else {
            self.expect(TokenType::JMPIN);
        }

        let condition = self.parse_value()?;
        let target = self.parse_identifier()?;

        indent_dec!();

        if zero {
            Ok(IntermediateInstruction::JUMP_IF_ZERO { condition, target })
        } else {
            Ok(IntermediateInstruction::JUMP_IF_NOT_ZERO { condition, target })
        }
    }

    pub fn parse_label(&mut self) -> Result<IntermediateInstruction, String> {
        info!("parse_label()");
        indent_inc!();

        self.expect(TokenType::LABL);
        let label = self.parse_identifier()?;

        indent_dec!();
        Ok(IntermediateInstruction::LABEL { label })
    }

    pub fn parse_assignment(&mut self) -> Result<IntermediateInstruction, String> {
        info!("parse_assignment()");
        indent_inc!();

        let dst = self.parse_value()?;

        self.expect(TokenType::ASSIGN);

        let current_tok = self.current().token_type.clone();
        let instruction: IntermediateInstruction = if current_tok.is_unary_operator() {
            self.parse_unary(dst)?
        } else if current_tok.is_binary_operator() {
            self.parse_binary(dst)?
        } else {
            return Err(format!(
                "Expected unary or binary operator, found {:?}",
                self.current().token_type
            ));
        };

        indent_dec!();
        Ok(instruction)
    }

    fn parse_unary(&mut self, dst: Value) -> Result<IntermediateInstruction, String> {
        info!("parse_unary()");
        indent_inc!();

        let operator = match self.current().token_type {
            TokenType::NEG => IntermediateUnaryOperator::NEGATE,
            TokenType::NOT => IntermediateUnaryOperator::NOT,
            _ => {
                return Err(format!(
                    "Expected unary operator, found {:?}",
                    self.current().token_type
                ));
            }
        };

        self.advance();
        let src = self.parse_value()?;

        indent_dec!();
        Ok(IntermediateInstruction::UNARY { operator, src, dst })
    }

    fn parse_binary(&mut self, dst: Value, ) -> Result<IntermediateInstruction, String> {
        info!("parse_binary()");
        indent_inc!();

        let src1 = self.parse_value()?;

        let operator = match self.current().token_type {
            TokenType::ADD => IntermediateBinaryOperator::ADD,
            TokenType::SUB => IntermediateBinaryOperator::SUBTRACT,
            TokenType::MLT => IntermediateBinaryOperator::MULTIPLY,
            TokenType::DIV => IntermediateBinaryOperator::DIVIDE,
            TokenType::MOD => IntermediateBinaryOperator::REMAINDER,

            TokenType::AND => IntermediateBinaryOperator::BIT_AND,
            TokenType::OR => IntermediateBinaryOperator::BIT_OR,
            TokenType::XOR => IntermediateBinaryOperator::BIT_XOR,
            TokenType::SLT => IntermediateBinaryOperator::SHIFT_LEFT,
            TokenType::SRT => IntermediateBinaryOperator::SHIFT_RIGHT,

            TokenType::EQ => IntermediateBinaryOperator::EQUAL,
            TokenType::NEQ => IntermediateBinaryOperator::NOT_EQUAL,
            TokenType::LT => IntermediateBinaryOperator::LESS_THAN,
            TokenType::LE => IntermediateBinaryOperator::LESS_OR_EQUAL,
            TokenType::GT => IntermediateBinaryOperator::GREATER_THAN,
            TokenType::GE => IntermediateBinaryOperator::GREATER_OR_EQUAL,

            _ => {
                return Err(format!(
                    "Expected binary operator, found {:?}",
                    self.current().token_type
                ));
            }
        };

        self.advance();
        let src2 = self.parse_value()?;

        indent_dec!();
        Ok(IntermediateInstruction::BINARY { operator, src1, src2, dst })
    }
}