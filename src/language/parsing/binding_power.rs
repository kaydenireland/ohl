use crate::language::tokenizing::token::Token;

pub struct BindingPower {
    pub left: u8,
    pub right: u8,
    pub unary: u8,
}

impl Token {
    pub fn is_prefix_operator(&self) -> bool {
        matches!(self, Token::SUB | Token::DIV | Token::NOT)
    }

    pub fn is_postfix_operator(&self) -> bool {
        matches!(self, Token::INCREMENT | Token::DECREMENT | Token::SQUARE)
    }

    pub fn is_identifier(&self) -> bool {
        matches!(self, Token::ID { .. })
    }

    pub fn binding_power(&self) -> BindingPower {
        match self {

            Token::ASSIGN |
            Token::ADD_ASSIGN |
            Token::SUB_ASSIGN |
            Token::MULT_ASSIGN |
            Token::DIV_ASSIGN |
            Token::REM_ASSIGN |
            Token::POWER_ASSIGN |
            Token::ROOT_ASSIGN => BindingPower { left: 5, right: 4, unary: 0 },


            Token::OR | Token::XOR => BindingPower { left: 15, right: 16, unary: 0 },
            Token::AND => BindingPower { left: 20, right: 21, unary: 0 },

            Token::EQUAL | Token::NEQ => BindingPower { left: 30, right: 31, unary: 0 },


            Token::LT |
            Token::GT |
            Token::NLT |
            Token::NGT =>  BindingPower { left: 32, right: 33, unary: 0 },

            Token::ADD => BindingPower { left: 40, right: 41, unary: 0 },
            Token::SUB => BindingPower { left: 40, right: 41, unary: 70 },
            Token::MULT |  Token::REM => BindingPower { left: 50, right: 51, unary: 0 },
            Token::DIV => BindingPower { left: 50, right: 51, unary: 70 },

            Token::POWER | Token::ROOT => BindingPower { left: 90, right: 89, unary: 0 },

            Token::NOT => BindingPower { left: 0, right: 0, unary: 70 },

            Token::INCREMENT | Token::DECREMENT | Token::SQUARE => BindingPower { left: 80, right: 0, unary: 0 },


            Token::ID { .. } |
            Token::LIT_CHAR { .. } |
            Token::LIT_INT { .. } |
            Token::LIT_FLOAT { .. } |
            Token::LIT_BOOL { .. } |
            Token::LIT_STRING { .. } => BindingPower { left: 0, right: 0, unary: 0 },

            Token::PAREN_L | Token::POINT => BindingPower { left: 0, right: 0, unary: 0 },


            Token::PAREN_R |
            Token::BRACKET_R |
            Token::BRACE_L |
            Token::BRACE_R |
            Token::COMMA |
            Token::COLON |
            Token::SEMICOLON |
            Token::EOI => BindingPower { left: 0, right: 0, unary: 0 },


            _ => BindingPower { left: 0, right: 0, unary: 0 },
        }
    }
}