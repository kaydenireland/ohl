use crate::oil::lexer::token::{Token, TokenType};
use crate::util::error::error::OhlError;
use crate::util::error::location::Location;

#[derive(Clone)]
enum LexerState {
    START,
    END,

    WORDS,
    NUMBERS,

    COMMENT
}


#[derive(Clone)]
pub struct Lexer {
    input: String,
    position: usize,
    state: LexerState,
    current: Token,
    buffer: String,
    line: usize,
    col: usize,

    string_line: usize,
    string_col: usize
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            input,
            position: 0,
            state: LexerState::START,
            current: Token::from(TokenType::EOI),
            buffer: String::new(),
            line: 1,
            col: 0,

            string_line: 0,
            string_col: 0
        }
    }

    pub fn set_input(&mut self, input: String) {
        self.input = input;
        self.position = 0;
        self.state = LexerState::START;
        self.current = Token::from(TokenType::EOI);
        self.buffer = String::new();
        self.line = 1;
        self.col = 1;

        self.string_line = 0;
        self.string_col = 0;
    }

    pub fn reset(&mut self) {
        self.position = 0;
        self.state = LexerState::START;
        self.current = Token::from(TokenType::EOI);
        self.buffer = String::new();
        self.line = 1;
        self.col = 1;

        self.string_line = 0;
        self.string_col = 0;
    }

    pub fn current(&self) -> Token {
        self.current.clone()
    }

    pub fn print_tokens(&mut self) {
        println!();
        loop {
            self.advance();
            if self.current.token_type == TokenType::EOI {
                break;
            }
            println!("{}", self.current.to_string());
        }
        println!("{}\n", self.current.to_string());
    }

    pub fn advance(&mut self) -> Token {
        loop {
            if self.position >= self.input.len() {
                if !self.buffer.is_empty() {
                    self.state = LexerState::END;
                    let token_type: TokenType = self.match_buffer();
                    self.current = self.create_token_with_location(token_type, self.line, self.col - self.buffer.len());
                    self.buffer = String::new();
                    break;
                }

                self.state = LexerState::END;
                self.current = self.create_token(TokenType::EOI);
                break;
            }

            let char = self.input.chars().nth(self.position).unwrap();
            self.position += 1;
            self.col += 1;

            match self.state {
                LexerState::START => match char {

                    // Whitespace
                    ' ' | '\t' | '\r' => continue,
                    '\n' => {
                        self.line += 1;
                        self.col = 1;
                        self.current = self.create_token(TokenType::NEWLINE);
                        break;
                    },

                    // Alphanumeric
                    'A'..='Z' | 'a'..='z' | '_' => {
                        self.state = LexerState::WORDS;
                        self.buffer.push(char);
                    },
                    '0'..='9' => {
                        self.state = LexerState::NUMBERS;
                        self.buffer.push(char);
                    },
                    '#' => {
                        self.state = LexerState::COMMENT;
                    },

                    _ => {
                        OhlError::new(
                            self.line, self.col,
                            format!("Unrecognized character '{}'", char).to_string()
                        ).report();
                    }
                },
                LexerState::WORDS => match char {
                    'A'..='Z' | 'a'..='z' | '.' | '_' | '0'..='9' => self.buffer.push(char),

                    _ => {
                        self.state = LexerState::START;
                        let token_type: TokenType = self.match_buffer();
                        self.current = self.create_token_with_location(token_type, self.line, self.col - self.buffer.len());
                        self.buffer = String::new();

                        self.position -= 1;
                        self.col -= 1;
                        break;
                    }
                },
                LexerState::NUMBERS => match char {
                    '0'..='9' => self.buffer.push(char),

                    _ => {
                        self.state = LexerState::START;
                        let value: i32 = self.buffer.parse().unwrap();
                        self.current = self.create_token_with_location(
                            TokenType::INT(value),
                            self.line,
                            self.col - self.buffer.len()
                        );
                        self.buffer = String::new();

                        self.position -= 1;
                        self.col -= 1;
                        break;
                    }
                },
                LexerState::COMMENT => {
                    if char == '\n' {
                        self.line += 1;
                        self.col = 1;
                        self.state = LexerState::START;
                        self.current = self.create_token(TokenType::NEWLINE);
                        break;
                    }
                },

                _ => {}
            }
        }

        self.current.clone()
    }

    fn match_buffer(&mut self) -> TokenType {
        let string = self.buffer.as_str();
        match string {
            "fn" => TokenType::FN,
            "ret" => TokenType::RET,
            "copy" => TokenType::COPY,
            "jmp" => TokenType::JMP,
            "jmpi" => TokenType::JMPI,
            "jmpin" => TokenType::JUMPIN,
            "labl" => TokenType::LABL,

            "neg" => TokenType::NEG,
            "not" => TokenType::NOT,

            "add" => TokenType::ADD,
            "sub" => TokenType::SUB,
            "mlt" => TokenType::MLT,
            "div" => TokenType::DIV,
            "mod" => TokenType::MOD,

            "band" => TokenType::AND,
            "bor" => TokenType::OR,
            "bxor" => TokenType::XOR,
            "slt" => TokenType::SLT,
            "srt" => TokenType::SRT,

            "eq" => TokenType::EQ,
            "neq" => TokenType::NEQ,
            "lt" => TokenType::LT,
            "le" => TokenType::LE,
            "gt" => TokenType::GT,
            "ge" => TokenType::GE,

            _ => TokenType::ID(string.to_string())
        }
    }


}

impl Lexer {
    fn create_token(&mut self, token_type: TokenType) -> Token {
        Token {
            token_type,
            location: Location::new(self.line, self.col),
        }
    }

    fn create_token_with_location(&mut self, token_type: TokenType, line: usize, col: usize) -> Token {
        Token {
            token_type,
            location: Location::new(line, col),
        }
    }
}