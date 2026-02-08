use crate::core::lexer::token::Token;
use crate::core::lexer::token_type::TokenType;
use crate::core::parser::parser::Parser;
use crate::core::parser::mtree::MTree;

impl Parser {
    pub fn parse_statement(&mut self) -> MTree {
        self.log.info("parse_statement()");
        self.log.indent_inc();

        let child: MTree;

        match self.current().token_type {
            
            TokenType::PRINT => child = self.parse_print(),
            TokenType::SEMICOLON => child = self.parse_blank(),
            TokenType::BRACE_L => child = self.parse_block(),
            TokenType::VAR => {
                child = self.parse_var();
                self.expect(TokenType::SEMICOLON);
            },
            _ => {
                child = self.parse_expression();
                self.expect(TokenType::SEMICOLON);
            }
        }
        self.log.indent_dec();

        child
    }

    pub fn parse_var(&mut self) -> MTree {
        self.log.info("parse_var()");
        self.log.indent_inc();

        let mut child = MTree::new(Token::from(TokenType::VAR));

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
    
    pub fn parse_print(&mut self) -> MTree {
        self.log.info("parse_print()");
        self.log.indent_inc();

        self.expect(TokenType::PRINT);

        let mut child = MTree::new(Token::from(TokenType::PRINT));
        child._push(self.parse_expression());
        self.expect(TokenType::SEMICOLON);

        self.log.indent_dec();
        child
    }


    pub fn parse_blank(&mut self) -> MTree {
        self.log.info("parse_blank()");
        while self.is(TokenType::SEMICOLON) {
            self.expect(TokenType::SEMICOLON);
        }
        MTree::new(Token::from(TokenType::SEMICOLON))
    }
}