use crate::core::lexer::token_type::TokenType;
use crate::core::util::location::Location;

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

    pub fn id(name: &str, location: Location) -> Token {
        Token {
            token_type: TokenType::ID { name: String::from(name) },
            location
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} {:?}", self.location.to_string(), self.token_type)
    }

}