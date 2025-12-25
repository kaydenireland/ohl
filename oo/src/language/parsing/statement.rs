use crate::language::parsing::parser::Parser;
use crate::language::parsing::mtree::MTree;
use crate::language::tokenizing::token::Token;

// Statement Parsing
impl Parser {

    // TODO: Match, Imports

    pub fn parse_statement(&mut self) -> MTree {
        self.log.info("parse_statement()");
        self.log.indent_inc();

        let child: MTree;

        match self.current() {
            Token::FOR => child = self.parse_for(),
            Token::WHILE => child = self.parse_while(),
            Token::LOOP => child = self.parse_loop(),

            Token::BREAK => child = self.parse_break(),
            Token::CONTINUE => child = self.parse_continue(),
            Token::REPEAT => child = self.parse_repeat(),

            Token::MATCH => child = self.parse_match(),
            Token::DEFER => child = self.parse_defer(),
            Token::IF => child = self.parse_if(),
            
            Token::RETURN => child = self.parse_return(),
            Token::BRACE_L => child = self.parse_block_nest(),
            Token::LET => {
                child = self.parse_let();
                self.expect(Token::SEMICOLON);
            },
            // TODO parse empty statements (just semicolons)
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
            child._push(MTree::new(Token::MUTABLE));
            child._push(self.parse_expression());
        } else if self.accept(Token::COLON) {
            self.expect(Token::ASSIGN);
            child._push(MTree::new(Token::IMMUTABLE));
            child._push(self.parse_expression());
        } else {
            child._push(MTree::new(Token::MUTABLE));
        }

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

        if self.is(Token::LET) {
            child._push(self.parse_let()); 
            self.expect(Token::SEMICOLON);
        } else {
            child._push(self.parse_expression());
            self.expect(Token::SEMICOLON);
        }

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
        // TODO: Optional Condition
        self.expect(Token::PAREN_L);
        child._push(self.parse_expression());
        self.expect(Token::PAREN_R);
        child._push(self.parse_block_nest());

        self.log.indent_dec();

        child
    }

    pub fn parse_break(&mut self) -> MTree {
        self.log.info("parse_break()");
        self.log.indent_inc();

        let child = MTree::new(Token::BREAK);
        self.expect(Token::BREAK);
        self.expect(Token::SEMICOLON);

        self.log.indent_dec();
        child
    }

    pub fn parse_continue(&mut self) -> MTree {
        self.log.info("parse_continue()");
        self.log.indent_inc();

        let child = MTree::new(Token::CONTINUE);
        self.expect(Token::CONTINUE);
        self.expect(Token::SEMICOLON);

        self.log.indent_dec();
        child
    }

    pub fn parse_repeat(&mut self) -> MTree {
        self.log.info("parse_repeat()");
        self.log.indent_inc();

        let child = MTree::new(Token::REPEAT);
        self.expect(Token::REPEAT);
        self.expect(Token::SEMICOLON);

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

    pub fn parse_defer(&mut self) -> MTree {
        self.log.info("parse_defer");
        self.log.indent_inc();

        let mut child = MTree::new(Token::DEFER);
        self.expect(Token::DEFER);

        if self.is(Token::BRACE_L) {
            child._push(self.parse_block_nest());
            // TODO rename parse_block_nest to parse_block
        } else {
            child._push(self.parse_expression());
            self.expect(Token::SEMICOLON);
        }

        self.log.indent_dec();
        child
    }

    pub fn parse_return(&mut self) -> MTree {
        self.log.info("parse_return()");
        self.log.indent_inc();

        // TODO: Replace Token::RTRN_STMT to Token::RETURN
        let mut child = MTree::new(Token::RTRN_STMT);

        self.expect(Token::RETURN);
        if !self.accept(Token::SEMICOLON) {
            child._push(self.parse_expression());
            self.expect(Token::SEMICOLON);
        }

        self.log.indent_dec();

        child
    }
}
