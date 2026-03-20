use crate::core::lexer::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub parameters: Vec<TokenType>,
    pub return_type: TokenType,
    pub called: bool
}

impl FunctionSignature {
    pub fn new(parameters: Vec<TokenType>, return_type: TokenType, called: bool) -> FunctionSignature {
        FunctionSignature {
            parameters,
            return_type,
            called
        }
    }
}