use crate::backend::mtree::MTree;
use crate::backend::parser::Parser;
use crate::backend::token::Token;

use std::rc::Rc;

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

            Token::PAREN_L | Token::POINT => BindingPower { left: 80, right: 0, unary: 0 },


            Token::PAREN_R |
            Token::BRACKET_R |
            Token::BRACE_L |
            Token::BRACE_R |
            Token::COMMA |
            Token::COLON |
            Token::SEMICOLON |
            Token::EOI => BindingPower { left: 0, right: 0, unary: 0 },


            // ---------- Catch-all ----------
            _ => BindingPower { left: 0, right: 0, unary: 0 },
        }
    }
}


impl Parser {
    pub fn parse_expression(&mut self) -> MTree {
        self.log.info("parse_expression()");
        self.log.indent_inc();

        let child = self.parse_expression_token(1);
        self.log.indent_dec();
        child
    }

    pub fn parse_expression_token(&mut self, rbl: u8) -> MTree {
        let token = self.current();

        if token.is_prefix_operator() {
            let tree_prefix = self.parse_prefix_expression();
            self.parse_infix_expression(tree_prefix, rbl)
        } else if token == Token::PAREN_L {
            let tree_parens = self.parse_parenthesis_expression();
            self.parse_infix_expression(tree_parens, rbl)
        } else if token.is_identifier() || token.is_literal() {
            let tree_atom = self.parse_atom_expression();
            self.parse_infix_expression(tree_atom, rbl)
        } else {
            MTree::new(Token::ERROR { msg: "Invalid Expression".to_string() })
        }
    }

    pub fn parse_prefix_expression(&mut self) -> MTree {
        let token = self.current();
        self.advance();
        let child = self.parse_expression_token(token.binding_power().unary);
        MTree{ token, children: vec![Rc::new(child)]}
    }

    pub fn parse_parenthesis_expression(&mut self) -> MTree {
        self.expect(Token::PAREN_L);
        let child = self.parse_expression();
        self.expect(Token::PAREN_R);
        child
    }

    pub fn parse_atom_expression(&mut self) -> MTree {
        let atom = self.current();
        self.advance();
        if self.is(Token::PAREN_L) {
            self.parse_call_expression(atom)
        } else {
            MTree::new(atom)
        }
    }

    pub fn parse_call_expression(&mut self, token: Token) -> MTree {
        let mut child = MTree::new(token);
        self.expect(Token::PAREN_L);
        if ! self.is(Token::PAREN_R) {
            child.children.push(Rc::new(self.parse_expression()) );
            while self.accept(Token::COMMA) {
                child.children.push(Rc::new(self.parse_expression()) );
            }
        }
        self.expect(Token::PAREN_R);
        child
    }

    pub fn parse_infix_expression(&mut self, mut left: MTree, rbl: u8) -> MTree {
        loop {
            let op_infix = self.current();
            if rbl > op_infix.binding_power().left {
                return left;
            }

            

            if op_infix.is_postfix_operator() {
                self.advance();
                left = MTree { token: op_infix, children: vec![Rc::new(left)] };
                continue;
            }

            self.advance();


            let right = self.parse_expression_token(op_infix.binding_power().right);
            left = MTree {
                token: op_infix,
                children: vec![
                    Rc::new(left),
                    Rc::new(right),
                ]
            }
        }
    }

}