use crate::core::parser::mtree::MTree;
use crate::core::parser::parser::Parser;
use crate::core::tokenizer::token::Token;

// Pratt Expression Parser

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
        } else if token.is_identifier() || token.is_literal() || token.is_type() {
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
        MTree::new(atom)
    }

    pub fn parse_call_expression_from_tree(&mut self, callee: MTree) -> MTree {
        let mut node = MTree::new(Token::CALL);

        node.children.push(callee);

        self.expect(Token::PAREN_L);

        if !self.is(Token::PAREN_R) {
            node.children.push(self.parse_expression());
            while self.accept(Token::COMMA) {
                node.children.push(self.parse_expression());
            }
        }

        self.expect(Token::PAREN_R);
        node
    }


    pub fn parse_infix_expression(&mut self, mut left: MTree, rbl: u8) -> MTree {
        loop {
            let current = self.current();

            // call
            if current == Token::PAREN_L {
                left = self.parse_call_expression_from_tree(left);
                continue;
            }

            // member access
            if current == Token::POINT {
                self.advance();
                let id = self.current();
                self.expect(Token::id());

                left = MTree {
                    token: Token::POINT,
                    children: vec![left, MTree::new(id)],
                };
                continue;
            }

            // postfix ops (++ etc)
            if current.is_postfix_operator() {
                self.advance();
                left = MTree { token: current, children: vec![left] };
                continue;
            }

            // infix ops
            if rbl > current.binding_power().left {
                return left;
            }

            self.advance();

            if current == Token::AS {
                let ty = self.parse_type(); 
                left = MTree {
                    token: Token::AS,
                    children: vec![left, ty],
                };
                continue;
            }


            let right = self.parse_expression_token(current.binding_power().right);
            left = MTree { token: current, children: vec![left, right] };
        }
    }

    pub fn parse_type(&mut self) -> MTree {
        self.log.info("parse_type()");
        self.log.indent_inc();

        let node = match self.current() {
            t if t.is_type() => {
                let var_type = self.current();
                self.advance();
                MTree::new(var_type)
            }

            _ => {
                MTree::new(Token::ERROR {
                    msg: "Invalid type".into(),
                })
            }
        };

        self.log.indent_dec();
        node
    }

}