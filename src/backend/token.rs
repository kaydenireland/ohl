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
    WHILE,
    LOOP,
    CONTINUE,
    BREAK,
    PRINT,
    RETURN,
    MATCH,

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
}