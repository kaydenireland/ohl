use crate::core::converter::operator::Operator::{ADD, COMPLEMENT, DIVIDE, MULTIPLY, NEGATE, POWER, RECIPRICOL, REMAINDER, SUBTRACT};
use crate::core::lexer::token_type::TokenType;

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    // Arithmetic

    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
    REMAINDER,
    POWER,
    ROOT,

    NEGATE,
    RECIPRICOL,

    // Logical
    AND,
    OR,
    XOR,
    NOT,


    // Bitwise
    COMPLEMENT,
}


impl Operator {
    pub fn from_token_type(typ: TokenType) -> Operator {
        match typ {
            // Arithmetic
            TokenType::PLUS => ADD,
            TokenType::DASH => {
                if typ.is_prefix_operator() {
                    NEGATE
                } else {
                    SUBTRACT
                }
            },
            TokenType::STAR => MULTIPLY,
            TokenType::SLASH => {
                if typ.is_prefix_operator() {
                    RECIPRICOL
                } else {
                    DIVIDE
                }
            },
            TokenType::PERCENT => REMAINDER,
            TokenType::POWER => POWER,
            TokenType::ROOT => Operator::ROOT,

            // Logical
            TokenType::NOT => Operator::NOT,
            TokenType::AND => Operator::AND,
            TokenType::OR => Operator::OR,
            TokenType::XOR => Operator::XOR,

            // Bitwise
            TokenType::TILDE => COMPLEMENT,

            _ => Operator::XOR
        }
    }
}