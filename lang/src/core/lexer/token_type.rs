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
    START,
    EOI,
    ERROR,
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

    pub fn is_prefix_operator(&self) -> bool {
        match self {
            TokenType::DASH => true,
            TokenType::SLASH => true,

            _ => false
        }
    }

    pub fn is_postfix_operator(&self) -> bool {
        false
    }

    pub fn is_identifier(&self) -> bool {
        match self {
            TokenType::ID { .. } => true,

            _ => false
        }
    }

    pub fn is_literal(&self) -> bool {
        match self {
            TokenType::LIT_STRING { .. } => true,
            TokenType::LIT_FLOAT { .. } => true,
            TokenType::TRUE | TokenType::FALSE => true,
            TokenType::NULL => true,

            _ => false
        }
    }

    pub fn id() -> TokenType {
        TokenType::ID {
            name: String::new(),
        }
    }

}

pub struct BindingPower {
    pub left: u8,
    pub right: u8,
    pub unary: u8,
}

impl TokenType {

    pub fn binding_power(&self) -> BindingPower {
        match self {

            TokenType::ASSIGN => BindingPower { left: 5, right: 4, unary: 0 },

            TokenType::OR | TokenType::XOR => BindingPower { left: 15, right: 16, unary: 0 },
            TokenType::AND => BindingPower { left: 20, right: 21, unary: 0 },

            TokenType::EQUAL | TokenType::NOT_EQUAL => BindingPower { left: 30, right: 31, unary: 0 },


            TokenType::LESS | TokenType::GREATER |
            TokenType::LESS_EQUAL | TokenType::GREATER_EQUAL =>  BindingPower { left: 32, right: 33, unary: 0 },

            TokenType::PAREN_R => BindingPower { left: 40, right: 41, unary: 0 },
            TokenType::DASH => BindingPower { left: 40, right: 41, unary: 70 },
            TokenType::STAR => BindingPower { left: 50, right: 51, unary: 0 },
            TokenType::SLASH => BindingPower { left: 50, right: 51, unary: 70 },

            TokenType::NOT => BindingPower { left: 0, right: 0, unary: 70 },

            TokenType::ID { .. } |
            TokenType::LIT_FLOAT { .. } |
            TokenType::TRUE |
            TokenType::FALSE |
            TokenType::LIT_STRING { .. } => BindingPower { left: 0, right: 0, unary: 0 },

            TokenType::PAREN_L => BindingPower { left: 100, right: 0, unary: 0 },
            TokenType::DOT => BindingPower { left: 100, right: 99, unary: 0 },


            _ => BindingPower { left: 0, right: 0, unary: 0 },
        }

    }
    
}