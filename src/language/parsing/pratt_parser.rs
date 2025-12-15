use crate::language::parsing::mtree::MTree;
use crate::language::parsing::parser::Parser;
use crate::language::tokenizing::token::Token;


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
        MTree{ token, children: vec![child]}
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
            child.children.push(self.parse_expression());
            while self.accept(Token::COMMA) {
                child.children.push(self.parse_expression());
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
                left = MTree { token: op_infix, children: vec![left] };
                continue;
            }

            self.advance();


            let right = self.parse_expression_token(op_infix.binding_power().right);
            left = MTree {
                token: op_infix,
                children: vec![
                    left,
                    right
                ]
            }
        }
    }

}