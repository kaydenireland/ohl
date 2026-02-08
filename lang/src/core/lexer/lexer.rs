use crate::core::lexer::token::Token;
use crate::core::lexer::token_type::TokenType;
use crate::core::util::error::Error;
use crate::core::util::location::Location;

enum LexerState {
    START,
    END,
    
    WORDS,
    NUMBERS,
    NUMPOINT,
    DECIMALS,
    
    SLASH,
    COMMENT,
    START_BLOCK_COMMENT,
    END_BLOCK_COMMENT,

    STRING,

    EXCLAIM,
    EQUAL,
    GREATER,
    LESS
}

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
        self.string_line = 0;
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
        println!("{}", self.current.to_string());
    }
    
    pub fn advance(&mut self) -> Token {
        loop {
            // Reached End of File While Lexing Token
            if self.position >= self.input.len() {
                
                if !self.buffer.is_empty() {
                    self.state = LexerState::END;
                    let token_type: TokenType = self.match_buffer();
                    self.current = self.create_token_with_location(token_type, self.line, self.col - self.buffer.len());
                    self.buffer = String::new();
                    break;
                }

                match self.state {
                    LexerState::SLASH => self.current = self.create_token(TokenType::SLASH),
                    LexerState::STRING => {
                        Error::new(self.line, self.col, "Unterminated string".to_string()).report();
                        self.buffer = String::new();
                        self.current = self.create_token(TokenType::EOI);
                        break;
                    }
                    
                    _ => {}
                }              
                
                self.state = LexerState::END;
                self.current = self.create_token(TokenType::EOI);
                break;
            }
            
            let char = self.input.chars().nth(self.position).unwrap();
            self.position += 1;
            self.col += 1;
            
            // State Machine
            match self.state {
                LexerState::START => match char {
                    
                    // Whitespace
                    ' ' | '\t' | '\r' => continue,
                    '\n' => {
                        self.line += 1;
                        self.col = 0;
                        continue;
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
                    '"' => {
                        self.state = LexerState::STRING;
                        self.string_line = self.line.clone();
                        self.string_col = self.col.clone();
                    },
                    
                    // Containers
                    '(' => {
                        self.current = self.create_token(TokenType::PAREN_L);
                        break;
                    },
                    ')' => {
                        self.current = self.create_token(TokenType::PAREN_R);
                        break;
                    },
                    '{' => {
                        self.current = self.create_token(TokenType::BRACE_L);
                        break;
                    },
                    '}' => {
                        self.current = self.create_token(TokenType::BRACE_R);
                        break;
                    },
                    
                    // Separators
                    ';' => {
                        self.current = self.create_token(TokenType::SEMICOLON);
                        break;
                    },
                    '.' => {
                        self.current = self.create_token(TokenType::DOT);
                        break;
                    },
                    ',' => {
                        self.current = self.create_token(TokenType::COMMA);
                        break;
                    },
                    
                    // Arithmetic Operators
                    '+' => {
                        self.current = self.create_token(TokenType::PLUS);
                        break;
                    },
                    '-' => {
                        self.current = self.create_token(TokenType::DASH);
                        break;
                    },
                    '*' => {
                        self.current = self.create_token(TokenType::STAR);
                        break;
                    },
                    '/' => self.state = LexerState::SLASH,
                    
                    // Assignment Operators
                    '=' => self.state = LexerState::EQUAL,
                    '<' => self.state = LexerState::LESS,
                    '>' => self.state = LexerState::GREATER,
                    '!' => self.state = LexerState::EXCLAIM,
                    
                    _ => {
                        Error::new(
                            self.line,
                            self.col,
                            format!("Unrecognized character '{}'", char).to_string()
                        ).report();
                    }
                    
                },
                LexerState::WORDS => match char {
                    'A'..='Z' | 'a'..='z' | '_' | '0'..='9' => self.buffer.push(char),
                    
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
                    '.' => self.state = LexerState::NUMPOINT,
                    
                    _ => {
                        self.state = LexerState::START;
                        let value: f32 = self.buffer.parse().unwrap();
                        self.current = self.create_token_with_location(
                            TokenType::LIT_FLOAT { value }, 
                            self.line, 
                            self.col - self.buffer.len()
                        );
                        self.buffer = String::new();
                        
                        self.position -= 1;
                        self.col -= 1;
                        break;
                    }
                },
                LexerState::NUMPOINT => match char {
                    '0'..='9' => {
                        self.state = LexerState::DECIMALS;
                        self.buffer.push('.');
                        self.buffer.push(char);
                    },
                    
                    _ => {
                        self.state = LexerState::START;
                        let value: f32 = self.buffer.parse().unwrap();
                        self.current = self.create_token_with_location(
                            TokenType::LIT_FLOAT { value }, 
                            self.line, 
                            self.col - self.buffer.len() - 1
                        );
                        self.buffer = String::new();
                        
                        self.position -= 2;
                        self.col -= 2;
                        break;
                    }
                },
                LexerState::DECIMALS => match char {
                    '0'..='9' => self.buffer.push(char),
                    
                    _ => {
                        self.state = LexerState::START;
                        let value: f32 = self.buffer.parse().unwrap();
                        self.current = self.create_token_with_location(
                            TokenType::LIT_FLOAT { value }, 
                            self.line, 
                            self.col - self.buffer.len()
                        );                        self.buffer = String::new();
                        
                        self.position -= 1;
                        self.col -= 1;
                        break;
                    }
                },
                LexerState::SLASH => match char {
                    '/' => self.state = LexerState::COMMENT,
                    '*' => self.state = LexerState::START_BLOCK_COMMENT,
                    
                    _ => {
                        self.state = LexerState::START;
                        self.current = self.create_token(TokenType::SLASH);
                        
                        self.position -= 1;
                        self.col -= 1;
                        break;
                    }
                },
                LexerState::COMMENT => {
                    if char == '\n' {
                        self.line += 1;
                        self.col = 0;
                        self.state = LexerState::START;
                    }
                },
                LexerState::START_BLOCK_COMMENT => {
                    if char == '\n' {
                        self.line += 1;
                        self.col = 0;
                    } else if char == '*' {
                        self.state = LexerState::END_BLOCK_COMMENT;
                    }
                },
                LexerState::END_BLOCK_COMMENT => match char {
                    '/' => self.state = LexerState::START,
                    '\n' => {
                        self.line += 1;
                        self.col = 0;
                        self.state = LexerState::START_BLOCK_COMMENT;
                    }
                    
                    _ => self.state = LexerState::START_BLOCK_COMMENT
                },
                LexerState::STRING => {
                    if char == '"' {
                        self.state = LexerState::START;
                        let value: String = self.buffer.clone();
                        self.current = self.create_token_with_location(
                            TokenType::LIT_STRING { value },
                            self.string_line,
                            self.string_col
                        );
                        self.buffer = String::new();

                        self.string_line = 0;
                        self.string_col = 0;
                        break;
                    } else {
                        self.buffer.push(char);
                    }
                },
                LexerState::EXCLAIM => match char {
                    '=' => {
                        self.state = LexerState::START;
                        self.current = self.create_token_with_location(
                            TokenType::NOT_EQUAL, 
                            self.line, 
                            self.col - 1);
                        break;
                    },

                    _ => {
                        self.state = LexerState::START;
                        self.current = self.create_token_with_location(
                            TokenType::NOT, 
                            self.line, 
                            self.col - 1
                        );
                        
                        self.position -= 1;
                        self.col -= 1;
                        break;
                    }
                },
                LexerState::EQUAL => match char {
                    '=' => {
                        self.state = LexerState::START;
                        self.current = self.create_token_with_location(
                            TokenType::EQUAL, 
                            self.line, 
                            self.col - 1);
                        break;
                    },

                    _ => {
                        self.state = LexerState::START;
                        self.current = self.create_token_with_location(
                            TokenType::ASSIGN, 
                            self.line, 
                            self.col - 1
                        );
                        
                        self.position -= 1;
                        self.col -= 1;
                        break;
                    }
                },
                LexerState::GREATER => match char {
                    '=' => {
                        self.state = LexerState::START;
                        self.current = self.create_token_with_location(
                            TokenType::GREATER_EQUAL, 
                            self.line, 
                            self.col - 1);
                        break;
                    },

                    _ => {
                        self.state = LexerState::START;
                        self.current = self.create_token_with_location(
                            TokenType::GREATER, 
                            self.line, 
                            self.col - 1
                        );
                        
                        self.position -= 1;
                        self.col -= 1;
                        break;
                    }
                },
                LexerState::LESS => match char {
                    '=' => {
                        self.state = LexerState::START;
                        self.current = self.create_token_with_location(
                            TokenType::LESS_EQUAL, 
                            self.line, 
                            self.col - 1);
                        break;
                    },

                    _ => {
                        self.state = LexerState::START;
                        self.current = self.create_token_with_location(
                            TokenType::LESS, 
                            self.line, 
                            self.col - 1
                        );
                        
                        self.position -= 1;
                        self.col -= 1;
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
            "not" => TokenType::NOT,
            "and" => TokenType::AND,
            "or" => TokenType::OR,
            "xor" => TokenType::XOR,
            
            "var" => TokenType::VAR,
            "null" => TokenType::NULL,
            "true" => TokenType::TRUE,
            "false" => TokenType::FALSE,

            "print" => TokenType::PRINT,
            
            _ => {
                TokenType::ID { name: string.to_string()}
            }
        }
    }
    
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
