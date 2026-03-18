use crate::core::lexer::token::Token;
use crate::core::lexer::token_type::TokenType;
use crate::core::parser::parser::Parser;
use crate::core::parser::mtree::MTree;

impl Parser {
    pub fn parse_statement(&mut self) -> MTree {
        self.log.info("parse_statement()");
        self.log.indent_inc();

        let child: MTree;
        let token_type = self.current().token_type;

        match token_type {
            
            TokenType::PRINT => child = self.parse_print(),
            TokenType::SEMICOLON => child = self.parse_blank(),
            TokenType::BRACE_L => child = self.parse_block(),
            TokenType::RETURN => child = self.parse_return(),
            TokenType::VAR => {
                child = self.parse_var();
                self.expect(TokenType::SEMICOLON);
            },
            TokenType::IF => child = self.parse_if(),
            _ => {
                if token_type.is_type(false) {
                    child = self.parse_variable_declaration();
                    self.expect(TokenType::SEMICOLON)
                } else {
                    child = self.parse_expression();
                    self.expect(TokenType::SEMICOLON);
                }
            }
        }
        self.log.indent_dec();

        child
    }

    pub fn parse_var(&mut self) -> MTree {
        self.log.info("parse_var()");
        self.log.indent_inc();

        let mut child = MTree::new(Token::using_location(TokenType::VAR_DECL, self.current()));

        self.expect(TokenType::VAR);

        let id = self.current();
        self.expect(TokenType::id());
        child._push(MTree::new(id));

        if self.accept(TokenType::ASSIGN) {
            child._push(self.parse_expression());
        }
        
        // Semicolons handled outside of parse_var

        self.log.indent_dec();

        child
    }

    pub fn parse_variable_declaration(&mut self) -> MTree {
        self.log.info("parse_variable_declaration()");
        self.log.indent_inc();

        let mut child = MTree::new(Token::using_location(TokenType::VAR_DECL, self.current()));

        let token = self.current();
        self.expect_type(false);
        child._push(MTree::new(token));

        let id = self.current();
        self.expect(TokenType::id());
        child._push(MTree::new(id));

        if self.accept(TokenType::ASSIGN) {
            child._push(self.parse_expression());
        }

        self.log.indent_dec();

        child
    }
    
    pub fn parse_print(&mut self) -> MTree {
        self.log.info("parse_print()");
        self.log.indent_inc();

        
        let mut child = MTree::new(self.current());
        self.expect(TokenType::PRINT);

        child._push(self.parse_expression());
        self.expect(TokenType::SEMICOLON);

        self.log.indent_dec();
        child
    }

    pub fn parse_return(&mut self) -> MTree {
        self.log.info("parse_return()");
        self.log.indent_inc();

        let mut child = MTree::new(self.current());

        self.expect(TokenType::RETURN);
        if !self.accept(TokenType::SEMICOLON) {
            child._push(self.parse_expression());
            self.expect(TokenType::SEMICOLON);
        }

        self.log.indent_dec();

        child
    }

    pub fn parse_if(&mut self) -> MTree {
        self.log.info("parse_if()");
        self.log.indent_inc();

        let mut child = MTree::new(self.current());

        self.expect(TokenType::IF);
        self.expect(TokenType::PAREN_L);
        child._push(self.parse_expression());
        self.expect(TokenType::PAREN_R);
        child._push(self.parse_optional_block());

        if self.accept(TokenType::ELSE) {
            if self.is(TokenType::IF) {
                child._push(self.parse_if());
            } else {
                child._push(self.parse_optional_block());
            }
        }

        self.log.indent_dec();

        child
    }

    pub fn parse_optional_block(&mut self) -> MTree {
        self.log.info("parse_optional_block()");

        if self.is(TokenType::BRACE_L) {
            return self.parse_block();
        } else {
            return self.parse_statement();
        }

    }


    pub fn parse_blank(&mut self) -> MTree {
        self.log.info("parse_blank()");
        let child = MTree::new(self.current());
        while self.is(TokenType::SEMICOLON) {
            self.expect(TokenType::SEMICOLON);
        }
        child
    }
}