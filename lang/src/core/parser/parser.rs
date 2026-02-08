use crate::core::lexer::lexer::Lexer;
use crate::core::lexer::token_type::TokenType;
use crate::core::util::logger::Logger;
use crate::core::parser::mtree::MTree;
use crate::core::lexer::token::Token;


pub struct Parser {
    lexer: Lexer,
    pub log: Logger
}

impl Parser {
    pub fn new(lexer: Lexer, _debug: bool) -> Parser {
        let log = Logger::new(_debug);
        Parser { lexer, log }
    }

    pub fn analyze(&mut self) -> MTree {
        self.advance();
        let tree = self.parse();
        self.expect(TokenType::EOI);
        tree
    }
}

impl Parser {
    // utility functions for lexer
    pub fn current(&self) -> Token {
        self.lexer.current()
    }

    pub fn advance(&mut self) {
        self.lexer.advance();
    }

    pub fn is(&self, token: TokenType) -> bool {
        self.lexer.current().token_type == token
    }

    pub fn expect(&mut self, token: TokenType) {
        let current = self.current();
        if std::mem::discriminant(&current.token_type) == std::mem::discriminant(&token) {
            self.log.info(&format!("expect({current:?})"));
            self.advance();
        } else {
            panic!("Expected '{token:?}', currently '{:?}'!", current.token_type);
        }
    }

    pub fn accept(&mut self, token: TokenType) -> bool {
        if self.current().token_type == token {
            self.advance();
            true
        } else {
            false
        }
    }
}

impl Parser {
    // Parsing Functions

    pub fn parse(&mut self) -> MTree {
        let mut tree = MTree::new(Token::from(TokenType::START));
        self.log.info("parse()");
        self.log.indent_inc();
        while !self.accept(TokenType::EOI) {
            tree._push(self.parse_statement());
        }

        tree
    }

    pub fn parse_block(&mut self) -> MTree {
        self.log.info("parse_block");
        self.log.indent_inc();

        let mut child = MTree::new(Token::from(TokenType::BLOCK));

        self.expect(TokenType::BRACE_L);
        while !self.is(TokenType::BRACE_R) {
            child._push(self.parse_statement());
        }
        self.expect(TokenType::BRACE_R);

        self.log.indent_dec();

        child
    }
}