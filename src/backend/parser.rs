use crate::backend::lexer::Lexer;
use crate::backend::logger::Logger;
use crate::backend::mtree::MTree;
use crate::backend::token::Token;

// TODO: Match, Imports
pub struct Parser {
    lexer: Lexer,
    pub log: Logger
}

impl Parser {
    pub fn new(lexer: Lexer, debug: bool) -> Parser {
        let log = Logger::new(debug);
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

impl Parser {
    // Statement Parsing

    pub fn parse_statement(&mut self) -> MTree {
        self.log.info("parse_statement()");
        self.log.indent_inc();

        let child: MTree;

        match self.current() {
            Token::FOR => child = self.parse_for(),
            Token::WHILE => child = self.parse_while(),
            Token::LOOP => child = self.parse_loop(),
            Token::MATCH => child = self.parse_match(),
            Token::IF => child = self.parse_if(),
            Token::RETURN => child = self.parse_return(),
            Token::BRACE_L => child = self.parse_block_nest(),
            Token::LET => child = self.parse_let(),
            _ => {
                child = self.parse_expression();
                self.expect(Token::SEMICOLON);
            }
        }
        self.log.indent_dec();

        child
    }

    pub fn parse_let(&mut self) -> MTree {
        self.log.info("parse_let()");
        self.log.indent_inc();

        let mut child = MTree::new(Token::VAR_DECL);

        self.expect(Token::LET);

        let id = self.current();
        self.expect(Token::id());
        child._push(MTree::new(id));

        self.expect(Token::COLON);
        let type_token = self.current();
        self.expect_type(false);
        child._push(MTree::new(type_token));

        if self.accept(Token::ASSIGN) {
            child._push(self.parse_expression());
        }

        self.expect(Token::SEMICOLON);

        self.log.indent_dec();

        child
    }

    pub fn parse_for(&mut self) -> MTree {
        self.expect(Token::FOR);
        if self.is(Token::EACH) {
            return self.parse_for_each();
        }

        self.log.info("parse_for()");
        self.log.indent_inc();

        let mut child = MTree::new(Token::FOR);
        self.expect(Token::PAREN_L);
        child._push(self.parse_let());
        child._push(self.parse_expression());
        self.expect(Token::SEMICOLON);
        child._push(self.parse_expression());
        self.expect(Token::PAREN_R);
        child._push(self.parse_block_nest());

        self.log.indent_dec();

        child
    }

    pub fn parse_for_each(&mut self) -> MTree {
        self.log.info("parse_for_each()");
        self.log.indent_inc();

        let mut child = MTree::new(Token::FOR);

        self.expect(Token::EACH);
        let id = self.current();
        self.expect(Token::id());
        child._push(MTree::new(id));
        self.expect(Token::IN);
        self.expect(Token::PAREN_L);
        child._push(self.parse_expression());
        self.expect(Token::PAREN_R);
        child._push(self.parse_block_nest());

        self.log.indent_dec();

        child
    }

    pub fn parse_while(&mut self) -> MTree {
        self.log.info("parse_while()");
        self.log.indent_inc();

        let mut child = MTree::new(Token::WHILE);

        self.expect(Token::WHILE);
        self.expect(Token::PAREN_L);
        child._push(self.parse_expression());
        self.expect(Token::PAREN_R);
        child._push(self.parse_block_nest());

        self.log.indent_dec();

        child
    }

    pub fn parse_loop(&mut self) -> MTree {
        self.log.info("parse_loop()");
        self.log.indent_inc();

        let mut child = MTree::new(Token::LOOP);

        self.expect(Token::LOOP);
        child._push(self.parse_block_nest());

        self.log.indent_dec();

        child
    }

    pub fn parse_if(&mut self) -> MTree {
        self.log.info("parse_if()");
        self.log.indent_inc();

        let mut child = MTree::new(Token::IF_STMT);

        self.expect(Token::IF);
        self.expect(Token::PAREN_L);
        child._push(self.parse_expression());
        self.expect(Token::PAREN_R);
        child._push(self.parse_block_nest());

        if self.accept(Token::ELSE) {
            if self.is(Token::IF) {
                child._push(self.parse_if());
            } else {
                child._push(self.parse_block_nest());
            }
        }

        self.log.indent_dec();

        child
    }

    pub fn parse_match(&mut self) -> MTree {
        self.log.info("parse_match()");
        self.log.indent_inc();

        let mut child = MTree::new(Token::MATCH);

        self.expect(Token::MATCH);
        self.expect(Token::PAREN_L);
        child._push(self.parse_expression());
        self.expect(Token::PAREN_R);
        self.expect(Token::BRACE_L);
        while !self.is(Token::BRACE_R) {
            child._push(self.parse_match_arm());
        }
        self.expect(Token::BRACE_R);

        self.log.indent_dec();
        child
    }

    pub fn parse_match_arm(&mut self) -> MTree {
        self.log.info("parse_match_arm()");
        self.log.indent_inc();
        
        let mut child = MTree::new(Token::MATCH_ARM);

        child._push(self.parse_expression());

        if self.is(Token::DEFAULT) {
            let default_token = self.current();
            self.advance();
            child._push(MTree::new(default_token));
        } else {
            child._push(self.parse_expression());
        }

        self.expect(Token::BIG_ARROW);

        if self.is(Token::BRACE_L) {
            child._push(self.parse_block_nest());
        }else {
            child._push(self.parse_statement());
        }

        child
    }

    // match arm

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
