#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {

    // Containers
    PAREN_L,
    PAREN_R,
    BRACE_L,
    BRACE_R,

    // Separators
    COMMA,
    DOT,
    SEMICOLON,

    // Arithmetic Symbols
    PLUS,
    DASH,
    STAR,
    SLASH,

    // Assignment
    ASSIGN,

    // Relational Operators
    EQUAL,
    NOT_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Logical Operators
    NOT,
    AND,
    OR,
    XOR,

    // Keywords
    PRINT,

    // Variables
    VAR,
    NULL,

    // Literals
    ID { name: String },
    LIT_STRING { value: String },
    LIT_FLOAT { value: f32 },
    TRUE,
    FALSE,

    // Meta
    EOI,
    BLOCK,

}

impl TokenType {

    pub fn is_logical_operator(&self) -> bool {
        match self { 
            TokenType::NOT => true,
            TokenType::AND => true,
            TokenType::OR => true,
            TokenType::XOR => true,
            _ => false
        }
    }
    
    pub fn is_arithmetic_operator(&self) -> bool {
        match self {
            TokenType::PLUS => true,
            TokenType::DASH => true,
            TokenType::STAR => true,
            TokenType::SLASH => true,
            _ => false
        }
    }
    
    pub fn is_assignment_operator(&self) -> bool {
        match self {
            TokenType::ASSIGN => true,
            _ => false
        }
    }
    
    pub fn is_relational_operator(&self) -> bool {
        match self {
            TokenType::EQUAL => true,
            TokenType::NOT_EQUAL => true,
            TokenType::GREATER => true,
            TokenType::GREATER_EQUAL => true,
            TokenType::LESS => true,
            TokenType::LESS_EQUAL => true,
            _ => false
        }
    }

}