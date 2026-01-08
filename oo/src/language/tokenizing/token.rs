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

    // Conditional Operators
    QUESTION,
    NULL_COAL,

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
    DO,
    WHILE,
    LOOP,
    CONTINUE,
    BREAK,
    REPEAT,
    PRINT,
    RETURN,
    DEFER,
    MATCH,
    DEFAULT,

    CLASS,
    IMPL,
    ENUM,
    EXTENDS,

    // Identifiers
    ID { name: String },
    CALL,

    // Basic Types
    INT,
    FLOAT,
    CHAR,
    STRING,
    BOOLEAN,
    NULL,
    FUNC,

    RANGE_EXCL,
    RANGE_INCL,

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
    
    PARAM_LIST,
    PARAM,
    BLOCK,
    BLANK_STMT,
    IF_STMT,
    VAR_DECL,
    RTRN_STMT,
    EXPR,
    MATCH_ARM,
    MUTABLE,
    IMMUTABLE
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

    pub fn is_assignment_operator(&self) -> bool {
        matches!(
            self,
            Token::ASSIGN
            | Token::ADD_ASSIGN
            | Token::SUB_ASSIGN
            | Token::MULT_ASSIGN
            | Token::DIV_ASSIGN
            | Token::REM_ASSIGN
            | Token::POWER_ASSIGN
            | Token::ROOT_ASSIGN
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
