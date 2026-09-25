use log::error;
use crate::ohl::converter::operator::Operator::{ADD, BIT_AND, BIT_OR, BIT_XOR, COMPLEMENT, DIVIDE, MULTIPLY, NEGATE, POWER, RECIPRICOL, REMAINDER, SHIFT_LEFT, SHIFT_RIGHT, SUBTRACT};
use crate::ohl::lexer::token_type::TokenType;

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

    // Relational
    EQUAL,
    NOT_EQUAL,
    GREATER_THAN,
    GREATER_THAN_EQUAL,
    LESS_THAN,
    LESS_THAN_EQUAL,


    // Bitwise
    COMPLEMENT,
    BIT_AND,
    BIT_OR,
    BIT_XOR,
    SHIFT_LEFT,
    SHIFT_RIGHT
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

            // Relational
            TokenType::EQUAL => Operator::EQUAL,
            TokenType::NOT_EQUAL => Operator::NOT_EQUAL,
            TokenType::GREATER => Operator::GREATER_THAN,
            TokenType::GREATER_EQUAL => Operator::GREATER_THAN_EQUAL,
            TokenType::LESS => Operator::LESS_THAN,
            TokenType::LESS_EQUAL => Operator::LESS_THAN_EQUAL,

            // Bitwise
            TokenType::BITWISE_COMPLEMENT => COMPLEMENT,
            TokenType::BITWISE_AND => BIT_AND,
            TokenType::BITWISE_OR => BIT_OR,
            TokenType::BITWISE_XOR => BIT_XOR,
            TokenType::BITWISE_SHIFT_LEFT => SHIFT_LEFT,
            TokenType::BITWISE_SHIFT_RIGHT => SHIFT_RIGHT,

            _ => panic!("Unknown operator {:?}", typ),
        }
    }
}