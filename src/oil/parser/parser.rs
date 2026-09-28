use log::info;
use crate::{indent_dec, indent_inc, indent_reset, log_debug};
use crate::oil::intermediate::function::IntermediateFunction;
use crate::oil::intermediate::instruction::IntermediateInstruction;
use crate::oil::intermediate::operator::Value;
use crate::oil::intermediate::program::IntermediateProgram;
use crate::oil::lexer::lexer::Lexer;
use crate::oil::lexer::token::{Token, TokenType};

pub struct Parser {
    lexer: Lexer
}

impl Parser {
    pub fn new(lexer: Lexer, _debug: bool) -> Parser {
        log_debug!(_debug);
        Parser { lexer }
    }

    pub fn analyze(&mut self) -> Result<IntermediateProgram, String> {
        self.advance();
        let program = self.parse();
        self.expect(TokenType::EOI);
        indent_reset!();
        program
    }

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
            info!("expect({current:?})");
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

    pub fn parse_identifier(&mut self) -> Result<String, String> {
        match &self.current().token_type {
            TokenType::ID(value) => {
                let name = value.clone();
                self.advance();
                Ok(name)
            },
            _ => Err(format!("Unexpected token type.  Must be an ID: '{:?}'!", self.current().token_type))
        }
    }

    pub fn parse_value(&mut self) -> Result<Value, String> {
        match &self.current().token_type {
            TokenType::INT(value) => {
                let value = *value;
                self.advance();
                Ok(Value::INT(value))
            }

            TokenType::ID(name) => {
                let name = name.clone();
                self.advance();
                Ok(Value::VAR(name))
            }

            _ => Err(format!(
                "Expected value, found {:?}",
                self.current().token_type
            )),
        }
    }
}

impl Parser {
    fn parse(&mut self) -> Result<IntermediateProgram, String> {
        info!("parse()");
        indent_inc!();

        let mut functions: Vec<IntermediateFunction> = Vec::new();

        while !self.is(TokenType::EOI) {
            functions.push(self.parse_function()?);
        }

        info!("");
        indent_dec!();
        Ok(IntermediateProgram { functions, temp_counter: 0, label_counter: 0 })
    }

    fn parse_function(&mut self) -> Result<IntermediateFunction, String> {
        info!("parse_function()");
        indent_inc!();

        self.expect(TokenType::FN);
        let name = self.parse_identifier()?;
        self.expect(TokenType::NEWLINE);
        let mut instructions: Vec<IntermediateInstruction> = Vec::new();

        while !self.is(TokenType::FN) && !self.is(TokenType::EOI) {
            instructions.push(self.parse_instruction()?);
        }

        indent_dec!();
        Ok(IntermediateFunction { name, instructions })
    }

    fn parse_instruction(&mut self) -> Result<IntermediateInstruction, String> {
        info!("parse_instruction()");
        indent_inc!();

        let instruction: IntermediateInstruction = match self.current().token_type {
            TokenType::RET => self.parse_return()?,
            TokenType::COPY => self.parse_copy()?,
            TokenType::JMP => self.parse_jump()?,
            TokenType::JMPI => self.parse_jump_conditional(true)?,
            TokenType::JMPIN => self.parse_jump_conditional(false)?,
            TokenType::LABL => self.parse_label()?,

            TokenType::ID(_) => self.parse_assignment()?,

            _ => return Err(format!("Invalid Token in Instruction: {:?}", self.current().token_type))
        };
        self.expect(TokenType::NEWLINE);

        indent_dec!();
        Ok(instruction)
    }
}