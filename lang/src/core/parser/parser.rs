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

    pub fn expect_type(&mut self, allow_null: bool) {
        let current = self.current().token_type;
        if current.is_type(false) {
            self.log.info(&format!("expect({current:?})"));
            self.advance();
        } else {
            if current == TokenType::NULL && allow_null {
                self.log.info("expect(NULL)");
                self.advance();
            } else {
                panic!("Expected variable type, current token is '{current:?}'!");
            }
        }
    }

    pub fn expect_function_type(&mut self) {
        let current = self.current().token_type;
        if current.is_function_type() {
            self.log.info(&format!("expect({current:?})"));
            self.advance();
        } else {
            panic!("Expected function type, current token is '{current:?}'!");
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
            tree._push(self.parse_function());
        }

        tree
    }

    pub fn parse_function(&mut self) -> MTree {
        self.log.info("parse_function()");
        self.log.indent_inc();

        let mut child = MTree::new(Token::from(TokenType::FUNC_DECL));

        let func_type = self.current();
        self.expect_function_type();
        child._push(MTree::new(func_type));

        let return_type = self.current();
        self.expect_type(true);
        child._push(MTree::new(return_type));

        let id = self.current();
        self.expect(TokenType::id());
        child._push(MTree::new(id));

        child._push(self.parse_parameter_list());
        child._push(self.parse_block());

        self.log.indent_dec();
        child
    }

    pub fn parse_parameter_list(&mut self) -> MTree {
        self.log.info("parse_parameter_list()");
        self.log.indent_inc();

        let mut child = MTree::new(Token::from(TokenType::PARAM_LIST));

        self.expect(TokenType::PAREN_L);

        if !self.is(TokenType::PAREN_R) {
            loop {
                child._push(self.parse_parameter());

                // break if no comma follows
                if !self.accept(TokenType::COMMA) {
                    break;
                }
            }
        }

        self.expect(TokenType::PAREN_R);

        self.log.indent_dec();
        child
    }

    pub fn parse_parameter(&mut self) -> MTree {
        self.log.info("parse_parameter()");
        self.log.indent_inc();

        let mut child = MTree::new(Token::from(TokenType::PARAM));

        let type_token = self.current();
        self.expect_type(false);
        child._push(MTree::new(type_token));

        let id = self.current();
        self.expect(TokenType::id());
        child._push(MTree::new(id));

        self.log.indent_dec();

        child
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