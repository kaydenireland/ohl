use crate::backend::expression::{Expression, Literal, Precedence};
use crate::backend::lexer::Lexer;
use crate::backend::logger::Logger;
use crate::backend::mtree::MTree;
use crate::backend::token::Token;

// TODO: Match, Imports
pub struct Parser {
    lexer: Lexer,
    log: Logger,
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

        // TODO: break, continue, kword statements
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
        child._push(self.parse_expression());
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
        // TODO parse match arms
        self.expect(Token::BRACE_R);

        self.log.indent_dec();
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

impl Parser {
    // Expression Parsing

    pub fn parse_expression(&mut self) -> MTree {
        self.parse_expression_precedence(Precedence::NONE)
    }

    pub fn parse_expression_precedence(&mut self, minimum_precedence: Precedence) -> MTree {
        self.log.info("parse_expression()");
        self.log.indent_inc();

        let mut lhs = self.parse_prefix();

        while self.current().precedence() > minimum_precedence {
            let operator = self.current();
            let precedence = operator.precedence();

            if matches!(operator, Token::INCREMENT | Token::DECREMENT) {
                self.advance();
                let mut new_lhs = MTree::new(Token::EXPR);
                new_lhs._push(lhs);
                new_lhs._push(MTree::new(operator));
                lhs = new_lhs;
                continue;
            }

            self.advance();

            let next_minimum_precedence = if matches!(
                operator,
                Token::ASSIGN
                    | Token::ADD_ASSIGN
                    | Token::SUB_ASSIGN
                    | Token::MULT_ASSIGN
                    | Token::DIV_ASSIGN
                    | Token::REM_ASSIGN
                    | Token::POWER_ASSIGN
                    | Token::ROOT_ASSIGN
            ) {
                Precedence::from_u8(precedence as u8 - 1)
            } else {
                precedence
            };

            let rhs = self.parse_expression_precedence(next_minimum_precedence);
            let mut new_lhs = MTree::new(Token::EXPR);
            new_lhs._push(lhs);
            new_lhs._push(MTree::new(operator));
            new_lhs._push(rhs);

            lhs = new_lhs;
        }

        self.log.indent_dec();

        lhs
    }

    pub fn parse_prefix(&mut self) -> MTree {
        self.log.info("parse_prefix()");
        self.log.indent_inc();

        let mut child: MTree;

        match self.current() {
            Token::LIT_INT { .. }
            | Token::LIT_FLOAT { .. }
            | Token::LIT_CHAR { .. }
            | Token::LIT_STRING { .. }
            | Token::LIT_BOOL { .. }
            | Token::NULL => {
                let literal = self.current();
                child = MTree::new(literal.clone()); // TODO: clone?
                self.advance();
            }
            Token::ID { .. } => {
                let id = self.current();
                child = MTree::new(id.clone());
                self.advance();
            }
            Token::PAREN_L => {
                self.expect(Token::PAREN_L);
                child = self.parse_expression();
                self.expect(Token::PAREN_R);
            }
            Token::NOT | Token::SUB | Token::INCREMENT | Token::DECREMENT => {
                let operator: Token = self.current();
                self.advance();
                let rhs = self.parse_expression_precedence(Precedence::UNARY);
                child = MTree::new(Token::EXPR);
                child._push(MTree::new(operator));
                child._push(rhs);
            }
            _ => panic!(
                "Unexpected token '{:?}' in prefix expression!",
                self.current()
            ),
        }

        self.log.indent_dec();

        child
    }
}
