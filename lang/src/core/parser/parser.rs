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

    pub fn get_scope_modifier(&mut self) -> Token {
        if self.current().token_type.is_scope_modifier() {
            let scope = self.current();
            self.expect_scope_modifier();
            scope
        } else {
            Token::using_location(TokenType::PRIVATE, self.current())
        }
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

    pub fn expect_type(&mut self, allow_void: bool, implicit: bool) {
        let current = self.current().token_type;
        if current.is_type(implicit) {
            self.log.info(&format!("expect({current:?})"));
            self.advance();
        } else {
            if current == TokenType::VOID && allow_void {
                self.log.info("expect(VOID)");
                self.advance();
            } else {
                panic!("Expected variable type, current token is '{current:?}'!");
            }
        }
    }

    pub fn expect_scope_modifier(&mut self) {
        let current = self.current().token_type;
        if current.is_scope_modifier() {
            self.log.info(&format!("expect({current:?})"));
            self.advance();
        } else {
            panic!("Expected scope modifier, current token is '{current:?}'!");
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

    // <program> ::= {class_declaration};
    pub fn parse(&mut self) -> MTree {
        let mut tree = MTree::new(Token::from(TokenType::START));
        self.log.info("parse()");
        self.log.indent_inc();
        while !self.accept(TokenType::EOI) {
            tree._push(self.parse_class());
        }

        self.log.info("");

        tree
    }

    // <class_declaration> ::= [<scope_modifier>] "class" <id> "{" [<class_body>] "}";
    pub fn parse_class(&mut self) -> MTree {
        self.log.info("parse_class()");
        self.log.indent_inc();

        let mut child = MTree::new(Token::using_location(TokenType::CLASS, self.current()));

        let scope: Token = self.get_scope_modifier();
        child._push(MTree::new(scope));

        self.expect(TokenType::CLASS);

        let id = self.current();
        self.expect(TokenType::id());
        child._push(MTree::new(id));

        child._push(self.parse_class_body());

        self.log.indent_dec();
        child
    }

    // <class_body> ::= {<function_declaration> | <variable_declaration>};
    pub fn parse_class_body(&mut self) -> MTree {
        self.log.info("parse_class_body()");
        self.log.indent_inc();

        let mut child = MTree::new(Token::using_location(TokenType::CLASS_BODY, self.current()));

        self.expect(TokenType::BRACE_L);

        loop {
            let scope: Token = self.get_scope_modifier();
            let current_type = self.current().token_type;

            if current_type == TokenType::FUNCTION {
                child._push(self.parse_function(scope));
            } else if current_type == TokenType::VARIABLE {
                child._push(self.parse_class_variable_declaration(scope));
                self.expect(TokenType::SEMICOLON);
            } else {
                break;
            }

        }

        self.expect(TokenType::BRACE_R);

        self.log.indent_dec();
        child
    }

    // <variable_declaration> ::= <scope_modifier> "var" <id> [":" <type>] ["=" <expression>] ";" | "const" <id> [":" <type>] ["=" <expression>] ";";
    pub fn parse_class_variable_declaration(&mut self, scope: Token) -> MTree {
        self.log.info("parse_variable_declaration()");
        self.log.indent_inc();

        let mut child = MTree::new(Token::using_location(TokenType::CLASS_VARIABLE, self.current()));

        child._push(MTree::new(scope));
        self.expect(TokenType::VARIABLE);
        let id = self.current();
        self.expect(TokenType::id());
        child._push(MTree::new(id));

        if self.accept(TokenType::COLON) {
            let token = self.current();
            self.expect_type(false, true);
            child._push(MTree::new(token));
        } else {
            child._push(MTree::new(Token::using_location(TokenType::INFER, self.current())));
        }

        if self.accept(TokenType::ASSIGN) {
            child._push(self.parse_expression());
        }

        self.log.indent_dec();

        child
    }

    // [modifier] fun <id> (<params>) <block>
    pub fn parse_function(&mut self, scope: Token) -> MTree {
        self.log.info("parse_function()");
        self.log.indent_inc();

        let mut child = MTree::new(Token::using_location(TokenType::FUNCTION, self.current()));

        child._push(MTree::new(scope));

        self.expect(TokenType::FUNCTION);

        let id = self.current();
        self.expect(TokenType::id());
        child._push(MTree::new(id));
        child._push(self.parse_parameter_list());

        if self.current().token_type == TokenType::ARROW {
            self.expect(TokenType::ARROW);
            let return_type = self.current();
            self.expect_type(true, false);
            child._push(MTree::new(return_type));
        } else {
            child._push(MTree::new(Token::using_location(TokenType::VOID, self.current())));
        }

        child._push(self.parse_block());

        self.log.indent_dec();
        child
    }

    // <params> ::= <param> {"," <param>};
    pub fn parse_parameter_list(&mut self) -> MTree {
        self.log.info("parse_parameter_list()");
        self.log.indent_inc();

        let mut child = MTree::new(Token::using_location(TokenType::PARAM_LIST, self.current()));

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

    // <param> ::= <id> {"," <id>} ":" <type>;
    pub fn parse_parameter(&mut self) -> MTree {
        self.log.info("parse_parameter()");
        self.log.indent_inc();

        let mut child = MTree::new(Token::using_location(TokenType::PARAM, self.current()));

        let id = self.current();
        self.expect(TokenType::id());
        child._push(MTree::new(id));

        // TODO: multiple params under one type // (x, y: int)

        self.expect(TokenType::COLON);

        let type_token = self.current();
        self.expect_type(false, false);
        child._push(MTree::new(type_token));

        self.log.indent_dec();

        child
    }

    pub fn parse_argument_list(&mut self) -> MTree {
        self.log.info("parse_argument_list()");
        self.log.indent_inc();

        let mut child = MTree::new(Token::using_location(TokenType::ARG_LIST, self.current()));

        if !self.is(TokenType::PAREN_R) {
            child.children.push(self.parse_expression());
            while self.accept(TokenType::COMMA) {
                child.children.push(self.parse_expression());
            }
        }

        self.log.indent_dec();

        child
    }

    pub fn parse_block(&mut self) -> MTree {
        self.log.info("parse_block");
        self.log.indent_inc();

        let mut child = MTree::new(Token::using_location(TokenType::BLOCK, self.current()));

        self.expect(TokenType::BRACE_L);
        while !self.is(TokenType::BRACE_R) {
            child._push(self.parse_statement());
        }
        self.expect(TokenType::BRACE_R);

        self.log.indent_dec();

        child
    }
}