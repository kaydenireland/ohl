use crate::language::tokenizing::lexer::Lexer;
use crate::language::logger::Logger;
use crate::language::parsing::mtree::MTree;
use crate::language::tokenizing::token::Token;


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
        self.expect(Token::EOI);
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

    pub fn is(&self, token: Token) -> bool {
        self.lexer.current() == token
    }

    pub fn expect(&mut self, token: Token) {
        let current = self.current();
        if current == token {
            self.log.info(&format!("expect({current:?})"));
            self.advance();
        } else {
            panic!("Expected '{token:?}', currently '{:?}'!", current);
        }
    }

    pub fn expect_type(&mut self, allow_null: bool) {
        let current = self.current();
        if current.is_type() {
            self.log.info(&format!("expect({current:?})"));
            self.advance();
        } else {
            if current == Token::NULL && allow_null {
                self.log.info("expect(NULL)");
                self.advance();
            } else {
                panic!("Expected variable type, current token is '{current:?}'!");
            }
        }
    }

    pub fn expect_function_type(&mut self) {
        let current = self.current();
        if current.is_function_type() {
            self.log.info(&format!("expect({current:?})"));
            self.advance();
        } else {
            panic!("Expected function type, current token is '{current:?}'!");
        }
    }

    pub fn accept(&mut self, token: Token) -> bool {
        if self.current() == token {
            self.advance();
            true
        } else {
            false
        }
    }
}

impl Parser {
    // Parse functions

    pub fn parse(&mut self) -> MTree {
        let mut tree = MTree::new(Token::START);
        self.log.info("parse()");
        self.log.indent_inc();
        while !self.accept(Token::EOI) {
            tree._push(self.parse_function());
        }

        tree
    }

    pub fn parse_function(&mut self) -> MTree {
        self.log.info("parse_function()");
        self.log.indent_inc();

        let mut child = MTree::new(Token::FUNC_DECL);

        let func_type = self.current();
        self.expect_function_type();
        child._push(MTree::new(func_type));

        let return_type = self.current();
        self.expect_type(true);
        child._push(MTree::new(return_type));

        let id = self.current();
        self.expect(Token::id());
        child._push(MTree::new(id));

        child._push(self.parse_parameter_list());
        child._push(self.parse_block_nest());

        self.log.indent_dec();
        child
    }

    pub fn parse_parameter_list(&mut self) -> MTree {
        self.log.info("parse_parameter_list()");
        self.log.indent_inc();

        let mut child = MTree::new(Token::PARAM_LIST);

        self.expect(Token::PAREN_L);

        if !self.is(Token::PAREN_R) {
            loop {
                child._push(self.parse_parameter());

                // break if no comma follows
                if !self.accept(Token::COMMA) {
                    break;
                }
            }
        }

        self.expect(Token::PAREN_R);

        self.log.indent_dec();
        child
    }

    pub fn parse_parameter(&mut self) -> MTree {
        self.log.info("parse_parameter()");
        self.log.indent_inc();

        let mut child = MTree::new(Token::PARAM);

        let id = self.current();
        self.expect(Token::id());
        child._push(MTree::new(id));

        self.expect(Token::COLON);

        let type_token = self.current();
        self.expect_type(false);
        child._push(MTree::new(type_token));

        self.log.indent_dec();

        child
    }

    pub fn parse_block_nest(&mut self) -> MTree {
        self.log.info("parse_block_nest()");
        self.log.indent_inc();

        let mut child = MTree::new(Token::BLOCK);

        self.expect(Token::BRACE_L);
        while !self.is(Token::BRACE_R) {
            child._push(self.parse_statement());
        }
        self.expect(Token::BRACE_R);

        self.log.indent_dec();

        child
    }
}

