#![allow(warnings)]

use std::mem::discriminant;

#[derive(Debug, Clone)]
pub enum Token {

    // Containers
    PAREN_L,
    PAREN_R,
    BRACKET_L,
    BRACKET_R,
    BRACE_L,
    BRACE_R,

    // Separators
    POINT,
    COMMA,
    SEMICOLON,
    ARROW,
    BIG_ARROW,

    // Arithmetic Operators
    ADD,
    INCREMENT,
    SUB,
    DECREMENT,
    MULT,
    SQUARE,
    DIV,
    REM,
    POWER,
    ROOT,

    // Relational Operators
    EQUAL,
    NEQ, // Not Equal (!=)
    LT,
    NGT, // Not Greater Than (<=)
    GT,
    NLT, // Not Less Than (>=)

    // Logical Operators
    NOT,
    AND,
    OR,
    XOR,

    // Assignment
    ASSIGN,

    // Keywords
    IMPORT,
    FROM,
    AS,
    PUBLIC,
    PRIVATE,
    PROTECTED,
    IF,
    ELSE,
    FOR,
    EACH,
    IN,
    WHILE,
    LOOP,
    CONTINUE,
    BREAK,
    PRINT,
    RETURN,
    MATCH,
    DEFAULT,

    CLASS,
    IMPL,
    ENUM,

    // Identifiers
    ID { name: String },

    // Basic Types
    INT,
    FLOAT,
    CHAR,
    STRING,
    BOOLEAN,
    NULL,
    // FUNCTION,

    // Literals
    LIT_INT { value: i32 },
    LIT_FLOAT { value: f32 },
    LIT_CHAR { value: char },
    LIT_STRING { value: String },
    LIT_BOOL { value: bool },

    // End of Input
    EOI,

    // Metadata Nonterminals
    START,
    FUNC_DECL,
    PARAM_LIST,
    PARAM,
    BLOCK,
    IF_STMT,
    VAR_DECL,
    RTRN_STMT,
    EXPR,
}

impl Token {
    pub fn is_type(&self) -> bool {
        matches!(self,
            Token::INT | Token::FLOAT | Token::CHAR | Token::STRING | Token::BOOLEAN
        )
    }

    pub fn is_function_type(&self) -> bool {
        matches!(self,
            Token::PUBLIC | Token::PRIVATE | Token::PROTECTED
        )
    }

    pub fn is_literal(&self) -> bool {
        matches!(self,
            Token::LIT_INT { .. } | Token::LIT_FLOAT { .. } | Token::LIT_CHAR { .. }
            | Token::LIT_STRING { .. } | Token::LIT_BOOL { .. } | Token::NULL
        )
    }
}

impl Token {
    pub fn id() -> Token {
        Token::ID { name: String::new() }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}