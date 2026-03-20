use crate::core::lexer::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct VariableSignature {
    pub var_type: TokenType,
    pub used: bool,
    pub mutable: bool
}

impl VariableSignature {
    pub fn new(var_type: TokenType, used: bool, mutable: bool) -> VariableSignature {
        VariableSignature {
            var_type,
            used,
            mutable
        }
    }
}