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
    COLON,
    ARROW,     // ->
    BIG_ARROW, // =>

    // Arithmetic Operators
    ADD,       // +
    INCREMENT, // ++
    SUB,       // -
    DECREMENT, // --
    MULT,      // '*'
    SQUARE,    // '**'
    DIV,       // /
    REM,       // %
    POWER,     // ^
    ROOT,      // ^/

    ADD_ASSIGN,   // +=
    SUB_ASSIGN,   // -=
    MULT_ASSIGN,  // '*='
    DIV_ASSIGN,   // /=
    REM_ASSIGN,   // %=
    POWER_ASSIGN, // ^=
    ROOT_ASSIGN,  // ^/=

    // Relational Operators
    EQUAL, // Equal (==)
    NEQ,   // Not Equal (!=)
    LT,    // Less Than (<)
    NGT,   // Not Greater Than (<=)
    GT,    // Greater Than (>)
    NLT,   // Not Less Than (>=)

    // Logical Operators
    NOT, // '!' or keyword not
    AND, // '&&' or keyword and
    OR,  // '||' or keyword or
    XOR, // '^^' or keyword xor

    // Assignment
    ASSIGN, // '='

    // Keywords
    IMPORT,
    FROM,
    AS,
    PUBLIC,
    PRIVATE,
    PROTECTED,
    LET,
    IF,
    ELSE,
    FOR,
    EACH,
    IN,
    WHILE,
    LOOP,
    CONTINUE,
    BREAK,
    REPEAT,
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

    // End of Input, Error
    ERROR { msg: String },
    EOI,

    // Metadata Nonterminals
    START,
    FUNC_DECL,
    ASSIGN_STMT,
    PARAM_LIST,
    PARAM,
    BLOCK,
    IF_STMT,
    VAR_DECL,
    RTRN_STMT,
    EXPR,
    MATCH_ARM,
}

impl Token {
    pub fn is_type(&self) -> bool {
        matches!(
            self,
            Token::INT | Token::FLOAT | Token::CHAR | Token::STRING | Token::BOOLEAN
        )
    }

    pub fn is_function_type(&self) -> bool {
        matches!(self, Token::PUBLIC | Token::PRIVATE | Token::PROTECTED)
    }

    pub fn is_literal(&self) -> bool {
        matches!(
            self,
            Token::LIT_INT { .. }
                | Token::LIT_FLOAT { .. }
                | Token::LIT_CHAR { .. }
                | Token::LIT_STRING { .. }
                | Token::LIT_BOOL { .. }
                | Token::NULL
        )
    }
}

impl Token {
    pub fn id() -> Token {
        Token::ID {
            name: String::new(),
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}
