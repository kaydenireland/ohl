use crate::core::lexer::token_type::TokenType;
use crate::core::util::location::Location;

#[derive(Clone)]
pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) location: Location,
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