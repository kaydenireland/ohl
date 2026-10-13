use crate::util::error::location::Location;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {

    // Keywords
    FN,
    RET,
    COPY,
    JMP,
    JMPZ,
    JMPN,
    LABL,

    // Unary operators
    NEG,
    NOT,

    // Binary operators
    ADD,
    SUB,
    MLT,
    DIV,
    MOD,

    AND,
    OR,
    XOR,
    SLT,
    SRT,

    EQ,
    NEQ,
    GT,
    GE,
    LT,
    LE,

    // Constants
    INT(i32),

    ID(String),


    // Separators
    NEWLINE,
    ASSIGN,

    // Meta
    EOI
}

impl TokenType {
    pub fn is_unary_operator(&self) -> bool {
        matches!(self, TokenType::NEG | TokenType::NOT)
    }

    pub fn is_binary_operator(&self) -> bool {
        match self {
            TokenType::ADD | TokenType::SUB => true,
            TokenType::MLT | TokenType::DIV | TokenType::MOD => true,

            TokenType::AND | TokenType::OR | TokenType::XOR => true,
            TokenType::SLT | TokenType::SRT => true,

            TokenType::EQ | TokenType::NEQ => true,
            TokenType::LT | TokenType::LE => true,
            TokenType::GT | TokenType::GE => true,

            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub location: Location,
}

impl Token {
    pub fn new(token_type: TokenType, location: Location) -> Token {
        Token {
            token_type,
            location
        }
    }

    pub fn from(token_type: TokenType) -> Token {
        Token {
            token_type,
            location: Location::empty()
        }
    }

    pub fn using_location(token_type: TokenType, token: Token) -> Token {
        Token {
            token_type,
            location: token.location
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} {:?}", self.location.to_string(), self.token_type)
    }

}