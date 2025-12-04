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
        match self {    // TODO: Tweak BPs
            Token::EOI => BindingPower { left: 0, right: 0, unary: 0 },
            Token::ID { .. } => BindingPower { left: 1, right: 1, unary: 0 },

            Token::LIT_CHAR { .. } => BindingPower { left: 0, right: 0, unary: 0 },
            Token::LIT_INT { .. } => BindingPower { left: 0, right: 0, unary: 0 },
            Token::LIT_FLOAT { .. } => BindingPower { left: 0, right: 0, unary: 0 },
            Token::LIT_BOOL { .. } => BindingPower { left: 0, right: 0, unary: 0 },
            Token::LIT_STRING { .. } => BindingPower { left: 0, right: 0, unary: 0 },

            Token::ASSIGN => BindingPower { left: 5, right: 4, unary: 0 },

            Token::ADD_ASSIGN => BindingPower { left: 7, right: 6, unary: 0 },
            Token::SUB_ASSIGN => BindingPower { left: 7, right: 6, unary: 0 },
            Token::MULT_ASSIGN => BindingPower { left: 7, right: 6, unary: 0 },
            Token::DIV_ASSIGN => BindingPower { left: 7, right: 6, unary: 0 },
            Token::REM_ASSIGN => BindingPower { left: 7, right: 6, unary: 0 },
            Token::POWER_ASSIGN => BindingPower { left: 7, right: 6, unary: 0 },
            Token::ROOT_ASSIGN => BindingPower { left: 7, right: 6, unary: 0 },


            Token::OR => BindingPower { left: 10, right: 11, unary: 0 },
            Token::XOR => BindingPower { left: 10, right: 11, unary: 0 },
            Token::AND => BindingPower { left: 11, right: 12, unary: 0 }, 
            Token::NOT => BindingPower { left: 18, right: 19, unary: 100 },

            Token::LT => BindingPower { left: 30, right: 30, unary: 0 },
            Token::GT => BindingPower { left: 30, right: 30, unary: 0 },
            Token::NLT => BindingPower { left: 30, right: 30, unary: 0 },
            Token::NGT => BindingPower { left: 30, right: 30, unary: 0 },
            Token::EQUAL => BindingPower { left: 30, right: 30, unary: 0 },
            Token::NEQ => BindingPower { left: 30, right: 30, unary: 0 },

            Token::ADD =>  BindingPower { left: 30, right: 31, unary: 0 },
            Token::SUB =>  BindingPower { left: 30, right: 31, unary: 100 }, 
            Token::MULT =>  BindingPower { left: 31, right: 32, unary: 0 },           
            Token::DIV =>  BindingPower { left: 31, right: 32, unary: 100 },
            Token::REM =>  BindingPower { left: 31, right: 32, unary: 0 },

            Token::POWER =>  BindingPower { left: 40, right: 41, unary: 0 },
            Token::ROOT =>  BindingPower { left: 40, right: 41, unary: 0 },
            Token::INCREMENT => BindingPower { left: 50, right: 0, unary: 100 },
            Token::DECREMENT => BindingPower { left: 50, right: 0, unary: 100 },
            Token::SQUARE =>  BindingPower { left: 50, right: 0, unary: 0 },



            Token::PAREN_L => BindingPower { left: 0, right: 0, unary: 0 },
            Token::PAREN_R => BindingPower { left: 0, right: 0, unary: 0 },
            Token::BRACKET_L => BindingPower { left: 0, right: 0, unary: 0 },
            Token::BRACKET_R => BindingPower { left: 0, right: 0, unary: 0 },
            Token::BRACE_L => BindingPower { left: 0, right: 0, unary: 0 },
            Token::BRACE_R => BindingPower { left: 0, right: 0, unary: 0 },

            Token::POINT => BindingPower { left: 0, right: 0, unary: 0 },
            Token::COMMA => BindingPower { left: 0, right: 0, unary: 0 },
            Token::COLON => BindingPower { left: 0, right: 0, unary: 0 },
            Token::SEMICOLON => BindingPower { left: 0, right: 0, unary: 0 },

            _ => BindingPower { left: 0, right: 0, unary: 0 }
            
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