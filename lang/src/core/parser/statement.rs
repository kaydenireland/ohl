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
            TokenType::IF => child = self.parse_if(),
            TokenType::WHILE => child = self.parse_while(),
            TokenType::DO => child = self.parse_do_while(),
            TokenType::LOOP => child = self.parse_loop(),
            TokenType::BREAK | TokenType::CONTINUE | TokenType::REPEAT => {
                self.expect(token_type.clone());
                child = MTree::new(Token::using_location(token_type.clone(), self.current()));
                self.expect(TokenType::SEMICOLON);
            }
            _ => {
                if token_type.is_type(true) {
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

    pub fn parse_variable_declaration(&mut self) -> MTree {
        self.log.info("parse_variable_declaration()");
        self.log.indent_inc();

        let mut child = MTree::new(Token::using_location(TokenType::VAR_DECL, self.current()));

        let token = self.current();
        self.expect_type(false, true);
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

        self.expect(TokenType::PAREN_L);

        child._push(self.parse_expression());

        self.expect(TokenType::PAREN_R);
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
        // Then
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

    pub fn parse_while(&mut self) -> MTree {
        self.log.info("parse_while()");
        self.log.indent_inc();

        let mut child = MTree::new(self.current());

        self.expect(TokenType::WHILE);

        self.expect(TokenType::PAREN_L);
        child._push(self.parse_expression());
        self.expect(TokenType::PAREN_R);

        child._push(self.parse_optional_block());

        self.log.indent_dec();

        child
    }

    pub fn parse_do_while(&mut self) -> MTree {
        self.log.info("parse_do_while()");
        self.log.indent_inc();

        let mut child = MTree::new(self.current());

        self.expect(TokenType::DO);
        child._push(self.parse_optional_block());

        self.expect(TokenType::WHILE);

        self.expect(TokenType::PAREN_L);
        child._push(self.parse_expression());
        self.expect(TokenType::PAREN_R);
        self.expect(TokenType::SEMICOLON);

        self.log.indent_dec();

        child
    }

    pub fn parse_loop(&mut self) -> MTree {
        self.log.info("parse_loop()");
        self.log.indent_inc();

        let mut child = MTree::new(Token::using_location(TokenType::WHILE, self.current()));

        self.expect(TokenType::LOOP);
        child._push(MTree::new(Token::using_location(TokenType::TRUE, self.current())));

        child._push(self.parse_block());

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