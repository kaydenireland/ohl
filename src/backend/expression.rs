use crate::backend::token::Token;

#[derive(Debug, Clone)]
pub enum Expression {
    LITERAL(Literal),
    VARIABLE(String),

    UNARY_OP {
        operator: Token,
        rhs: Box<Expression>,
    },
    BINARY_OP {
        lhs: Box<Expression>,
        operator: Token,
        rhs: Box<Expression>,
    },
    FUNCTION_CALL {
        callee: String,
        arguments: Vec<Expression>,
    },
    GROUPING(Box<Expression>),
    NULL,
}

#[derive(Debug, Clone)]
pub enum Literal {
    INTEGER(i64),
    FLOAT(f64),
    BOOLEAN(bool),
    CHAR(char),
    STRING(String),
}

#[repr(u8)]
#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub enum Precedence {
    NONE = 0,
    ASSIGNMENT = 1, // =
    OR = 2,         // ||, or, xor
    AND = 3,        // &&, and
    EQUALITY = 4,   // ==, !=
    COMPARE = 5,    // <, >, <=, >=
    SUM = 6,        // +, -
    PRODUCT = 7,    // '*, /, %'
    EXPONENT = 8,   // ^, **, ^/
    UNARY = 9,      // '!', - (prefix)
    POSTFIX = 10,   // ++, --
    CALL = 11,      // function calls (), .
    PRIMARY = 12,   // literals, identifiers
}

impl Precedence {
    pub fn from_u8(value: u8) -> Precedence {
        match value {
            0 => Precedence::NONE,
            1 => Precedence::ASSIGNMENT,
            2 => Precedence::OR,
            3 => Precedence::AND,
            4 => Precedence::EQUALITY,
            5 => Precedence::COMPARE,
            6 => Precedence::SUM,
            7 => Precedence::PRODUCT,
            8 => Precedence::EXPONENT,
            9 => Precedence::UNARY,
            10 => Precedence::POSTFIX,
            11 => Precedence::CALL,
            12 => Precedence::PRIMARY,
            _ => Precedence::NONE,
        }
    }
}

impl Token {
    pub fn precedence(&self) -> Precedence {
        match self {
            // Assignment
            Token::ASSIGN
            | Token::ADD_ASSIGN
            | Token::SUB_ASSIGN
            | Token::MULT_ASSIGN
            | Token::DIV_ASSIGN
            | Token::REM_ASSIGN
            | Token::POWER_ASSIGN
            | Token::ROOT_ASSIGN => Precedence::ASSIGNMENT,

            // Logical operators
            Token::OR | Token::XOR => Precedence::OR,
            Token::AND => Precedence::AND,
            Token::NOT => Precedence::UNARY,

            // Equality / comparison
            Token::EQUAL | Token::NEQ => Precedence::EQUALITY,
            Token::LT | Token::NGT | Token::GT | Token::NLT => Precedence::COMPARE,

            // Arithmetic
            Token::ADD | Token::SUB => Precedence::SUM,
            Token::MULT | Token::DIV | Token::REM => Precedence::PRODUCT,
            Token::SQUARE | Token::POWER | Token::ROOT => Precedence::EXPONENT,

            // Unary / postfix
            Token::INCREMENT | Token::DECREMENT => Precedence::POSTFIX,

            // Function calls or member access
            Token::POINT => Precedence::CALL,

            _ => Precedence::NONE,
        }
    }
}
