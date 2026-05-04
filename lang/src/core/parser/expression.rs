use crate::core::parser::mtree::MTree;
use crate::core::parser::parser::Parser;
use crate::core::lexer::token::Token;
use crate::core::lexer::token_type::TokenType;
use crate::core::util::error::Error;

// Pratt Expression Parser

impl Parser {
    pub fn parse_expression(&mut self) -> MTree {
        self.log.info("parse_expression()");
        self.log.indent_inc();

        let child = self.parse_expression_token(1);
        self.log.indent_dec();
        child
    }

    fn parse_expression_token(&mut self, rbl: u8) -> MTree {
        let token = self.current();
        let token_type = token.token_type;

        if token_type.is_prefix_operator() {
            let tree_prefix = self.parse_prefix_expression();
            self.parse_infix_expression(tree_prefix, rbl)
        } else if token_type == TokenType::PAREN_L {
            let tree_parens = self.parse_parenthesis_expression();
            self.parse_infix_expression(tree_parens, rbl)
        } else if token_type.is_identifier() || token_type.is_literal() {
            let tree_atom = self.parse_atom_expression();
            self.parse_infix_expression(tree_atom, rbl)
        } else {
            Error::new(token.location.get_line(), token.location.get_col(), "Invalid Expression".to_string()).report();
            MTree::new(Token::new(TokenType::ERROR, token.location))
        }
    }

    fn parse_prefix_expression(&mut self) -> MTree {
        let token = self.current();
        self.advance();
        let child = self.parse_expression_token(token.token_type.binding_power().unary);
        MTree{ token, children: vec![child]}
    }

    fn parse_parenthesis_expression(&mut self) -> MTree {
        self.expect(TokenType::PAREN_L);
        let child = self.parse_expression();
        self.expect(TokenType::PAREN_R);
        child
    }

    fn parse_atom_expression(&mut self) -> MTree {
        let atom = self.current();
        self.advance();
        MTree::new(atom)
    }



    fn parse_infix_expression(&mut self, mut left: MTree, rbl: u8) -> MTree {
        loop {
            let current = self.current();

            // call
            if current.token_type == TokenType::PAREN_L {
                left = self.parse_call_expression(left);
                continue;
            }

            // member access
            if current.token_type == TokenType::PERIOD {
                self.advance();
                let id = self.current();
                self.expect(TokenType::id());

                left = MTree {
                    token: Token::using_location(TokenType::PERIOD, current),
                    children: vec![left, MTree::new(id)],
                };
                continue;
            }

            // postfix ops
            if current.token_type.is_postfix_operator() {
                self.advance();
                left = MTree { token: current, children: vec![left] };
                continue;
            }

            // infix ops
            if rbl > current.token_type.binding_power().left {
                return left;
            }

            self.advance();


            let right = self.parse_expression_token(current.token_type.binding_power().right);
            left = MTree { token: current, children: vec![left, right] };
        }
    }

    fn parse_call_expression(&mut self, callee: MTree) -> MTree {
        let mut node = MTree::new(Token::using_location(TokenType::CALL, callee.token.clone()));

        node.children.push(callee);

        self.expect(TokenType::PAREN_L);

        node._push(self.parse_argument_list());

        self.expect(TokenType::PAREN_R);
        node
    }

}