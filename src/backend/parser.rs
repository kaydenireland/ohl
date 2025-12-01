use crate::backend::mtree::MTree;
use crate::backend::token::Token;
use crate::backend::lexer::Lexer;
use crate::backend::logger::Logger;

pub struct Parser {
    lexer: Lexer,
    log: Logger
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let log = Logger::new();
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
    pub fn current(&mut self) -> Token {
        self.lexer.current()
    }

    pub fn advance(&mut self) {
        self.lexer.advance();
    }

    pub fn peek(&mut self, token: Token) -> bool {
        self.lexer.current() == token
    } 

    pub fn expect(&mut self, token: Token) {
        if self.current() == token {
            self.advance();
            self.log.info("expect({token:?})");
        } else {
            panic!("Expected '{token:?}', currently '{:?}'!", self.current());
        }
    }

    pub fn expect_type(&mut self, allow_null: bool) {
        if self.current().is_type(){
            self.advance();
            self.log.info("expect({self.current():?})");
        } else {
            if self.current() == Token::NULL && allow_null{
                self.log.info("expect(NULL)")
            } else {
                let token = self.current();
                panic!("Expected variable type, current token is '{token:?}'!");
            }
        }
    }

    pub fn expect_function_type(&mut self) {
        if self.current().is_function_type() {
            self.advance();
            self.log.info("expect({self.current():?})");
        } else {
            let token = self.current();
            panic!("Expected function type, current token is '{token:?}'!");
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
        while !self.accept(Token::EOI) {
            tree._push(self.parse_function());
        }

        tree
    }

    pub fn parse_function(&mut self) -> MTree {
        self.log.info("parse_function()");
        self.log.indent_inc();

        let mut child = MTree::new(Token::FUNC_DECL);

        let token = self.current();
        self.expect_function_type();
        child._push(MTree::new(token));

        let token = self.current();
        self.expect_type(true);
        child._push(MTree::new(token));

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
        if self.accept(Token::PAREN_R) {
            return child;
        }

        child._push(self.parse_parameter());
        while self.accept(Token::COMMA) {
            child._push(self.parse_parameter());
        }
        self.expect(Token::PAREN_R);

        self.log.indent_dec();

        child
    }

    pub fn parse_parameter(&mut self) -> MTree {
        self.log.info("parse_parameter()");
        self.log.indent_inc();

        let mut child = MTree::new(Token::PARAM);

        let type_token = self.current();
        self.expect_type(false);
        child._push(MTree::new(type_token));

        let id = self.current();
        self.expect(Token::id());
        child._push(MTree::new(id));

        self.log.indent_dec();

        child
    }

    pub fn parse_block_nest(&mut self) -> MTree {
        self.log.info("parse_block_nest()");
        self.log.indent_inc();

        let mut child = MTree::new(Token::BLOCK);

        self.expect(Token::BRACKET_L);
        while !self.peek(Token::BRACKET_R) {
            child._push(self.parse_statement());
        }
        self.expect(Token::BRACKET_R);
        
        self.log.indent_dec();

        child
    }
}

impl Parser {
    // Statement/Expression Parsing

    pub fn parse_statement(&mut self) -> MTree {
        self.log.info("parse_statement()");
        self.log.indent_inc();

        let child: MTree;

        match self.current() {
            Token::FOR => {},
            Token::WHILE => {},
            Token::LOOP => {},
            Token::IF => {},
            Token::RETURN => child = self.parse_return(),
            Token::BRACE_L => {},
            _ => panic!("Unexpected token '{:?}' in statement!", self.current())
        }
        self.log.indent_dec();

        child
    }

    pub fn parse_expression(&mut self) -> MTree {
        self.log.info("parse_expression()");
        self.log.indent_inc();

        let mut child = MTree::new(Token::EXPR);

        // TODO expression logic
        child
    }

    pub fn parse_return(&mut self) -> MTree {
        self.log.info("parse_return()");
        self.log.indent_inc();

        let mut child = MTree::new(Token::RTRN_STMT);

        self.expect(Token::RETURN);
        child._push(self.parse_expression());
        self.expect(Token::SEMICOLON);

        self.log.indent_dec();

        child
    }
}