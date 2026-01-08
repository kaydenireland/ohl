use crate::language::tokenizing::token::Token;
#[allow(non_camel_case_types)]

enum LexerState {
    START,
    END,

    CHARS,
    READ_CHAR,
    READ_STRING,
    NUMBERS,
    AFTER_DOT,
    DECIMALS,

    PERIOD,
    RANGE,

    EQUAL,
    GREATER,
    LESS,

    PLUS,
    DASH,
    ASTERISK,
    SLASH,
    PERCENT,
    CARET,
    ROOT,

    PIPE,
    AMPERSAND,

    EXCLAIM,
    QUESTION,

    COMMENTS,
    BLOCK_COMMENT,
    BLOCK_COMMENT2,
}

pub struct Lexer {
    input_string: String,
    position: usize,
    state: LexerState,
    current_token: Token,
    buffer_string: String,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input_string: input,
            position: 0,
            state: LexerState::START,
            current_token: Token::EOI,
            buffer_string: String::new(),
        }
    }

    pub fn set_input(&mut self, input: String) {
        self.input_string = input;
        self.position = 0;
        self.state = LexerState::START;
        self.current_token = Token::EOI;
        self.buffer_string = String::new();
    }

    pub fn current(&self) -> Token {
        self.current_token.clone()
    }

    pub fn print_tokens(&mut self) {
        println!("");
        loop {
            self.advance();
            if let Token::EOI = self.current() {
                break;
            }
            println!("{:?}, ", self.current());
        }
        println!("{:?}", self.current());
    }

    pub fn advance(&mut self) -> Token {
        loop {
            if self.position == self.input_string.len() {
                match self.state {
                    LexerState::AFTER_DOT => {
                        let value: f32 = self.buffer_string.parse().unwrap();
                        self.state = LexerState::START;
                        self.current_token = Token::LIT_FLOAT { value };
                        self.buffer_string = String::new();
                        self.position -= 1;
                        break;
                    }
                    LexerState::EQUAL => self.current_token = Token::ASSIGN,
                    LexerState::GREATER => self.current_token = Token::GT,
                    LexerState::LESS => self.current_token = Token::LT,
                    LexerState::PLUS => self.current_token = Token::ADD,
                    LexerState::DASH => self.current_token = Token::SUB,
                    LexerState::ASTERISK => self.current_token = Token::MULT,
                    LexerState::SLASH => self.current_token = Token::DIV,
                    LexerState::PERCENT => self.current_token = Token::REM,
                    LexerState::CARET => self.current_token = Token::POWER,
                    LexerState::ROOT => self.current_token = Token::ROOT,
                    LexerState::EXCLAIM => self.current_token = Token::NOT,
                    LexerState::QUESTION => self.current_token = Token::QUESTION,
                    LexerState::PERIOD => self.current_token = Token::POINT,
                    _ => self.current_token = Token::EOI,
                }

                if !self.buffer_string.is_empty() {
                    self.state = LexerState::START;
                    self.current_token = self.match_buffer_string();
                    self.buffer_string = String::new();
                    break;
                }
                self.state = LexerState::END;
                break;
            }

            let current_char = self.input_string.chars().nth(self.position).unwrap();
            self.position += 1;

            match self.state {
                LexerState::START => match current_char {
                    ' ' | '\t' | '\r' | '\n' => continue,
                    'A'..='Z' | 'a'..='z' | '_' => {
                        self.state = LexerState::CHARS;
                        self.buffer_string.push(current_char);
                    }
                    '0'..='9' => {
                        self.state = LexerState::NUMBERS;
                        self.buffer_string.push(current_char);
                    }
                    ':' => {
                        self.current_token = Token::COLON;
                        break;
                    }
                    ';' => {
                        self.current_token = Token::SEMICOLON;
                        break;
                    }
                    '{' => {
                        self.current_token = Token::BRACE_L;
                        break;
                    }
                    '}' => {
                        self.current_token = Token::BRACE_R;
                        break;
                    }
                    '[' => {
                        self.current_token = Token::BRACKET_L;
                        break;
                    }
                    ']' => {
                        self.current_token = Token::BRACKET_R;
                        break;
                    }
                    '(' => {
                        self.current_token = Token::PAREN_L;
                        break;
                    }
                    ')' => {
                        self.current_token = Token::PAREN_R;
                        break;
                    }
                    '\'' => self.state = LexerState::READ_CHAR,
                    '"' => self.state = LexerState::READ_STRING,
                    '.' => self.state = LexerState::PERIOD,
                    ',' => {
                        self.current_token = Token::COMMA;
                        break;
                    }
                    '=' => self.state = LexerState::EQUAL,
                    '>' => self.state = LexerState::GREATER,
                    '<' => self.state = LexerState::LESS,
                    '+' => self.state = LexerState::PLUS,
                    '-' => self.state = LexerState::DASH,
                    '*' => self.state = LexerState::ASTERISK,
                    '/' => self.state = LexerState::SLASH,
                    '%' => self.state = LexerState::PERCENT,
                    '^' => self.state = LexerState::CARET,
                    '!' => self.state = LexerState::EXCLAIM,
                    '?' => self.state = LexerState::QUESTION,
                    '|' => self.state = LexerState::PIPE,
                    '&' => self.state = LexerState::AMPERSAND,

                    _ => {}
                },
                LexerState::CHARS => match current_char {
                    'A'..'Z' | '_' | 'a'..'z' | '0'..'9' => {
                        // TODO: Explore dashes without messing up expressions
                        self.buffer_string.push(current_char);
                    }

                    _ => {
                        self.state = LexerState::START;
                        self.current_token = self.match_buffer_string();
                        self.buffer_string = String::new();

                        self.position -= 1;
                        break;
                    }
                },
                LexerState::NUMBERS => match current_char {
                    '0'..='9' => {
                        self.buffer_string.push(current_char);
                    }

                    '.' => {
                        self.state = LexerState::AFTER_DOT;
                    }

                    _ => {
                        self.state = LexerState::START;
                        let value: i32 = self.buffer_string.parse().unwrap();
                        self.current_token = Token::LIT_INT { value };
                        self.buffer_string = String::new();

                        self.position -= 1;
                        break;
                    }
                },
                LexerState::AFTER_DOT => match current_char {
                    '0'..='9' => {
                        self.state = LexerState::DECIMALS;
                        self.buffer_string.push('.');
                        self.buffer_string.push(current_char);
                    }

                    '.' => {
                        self.state = LexerState::RANGE;

                        let value: i32 = self.buffer_string.parse().unwrap();
                        self.current_token = Token::LIT_INT { value };
                        self.buffer_string = String::new();
                        break;
                    }

                    _ => {
                        self.state = LexerState::START;
                        let value: f32 = self.buffer_string.parse().unwrap();
                        self.current_token = Token::LIT_FLOAT { value };
                        self.buffer_string = String::new();

                        self.position -= 1;
                        break;
                    }
                },
                LexerState::DECIMALS => match current_char {
                    '0'..='9' => {
                        self.buffer_string.push(current_char);
                    }

                    _ => {
                        self.state = LexerState::START;
                        let value: f32 = self.buffer_string.parse().unwrap();
                        self.current_token = Token::LIT_FLOAT { value };
                        self.buffer_string = String::new();

                        self.position -= 1;
                        break;
                    }
                },
                LexerState::READ_CHAR => match current_char {
                    '\'' => {
                        self.state = LexerState::START;
                        if self.buffer_string.len() == 1 {
                            let value = self.buffer_string.chars().nth(0).unwrap();
                            self.current_token = Token::LIT_CHAR { value };
                            self.buffer_string = String::new();
                            break;
                        }
                        self.buffer_string = String::new();
                    }
                    _ => {
                        self.buffer_string.push(current_char);
                    }
                },
                // TODO: Error Handling for unclosed chars/strings, invalid chars
                LexerState::READ_STRING => match current_char {
                    '"' => {
                        self.state = LexerState::START;
                        let value = self.buffer_string.clone();
                        self.current_token = Token::LIT_STRING { value };
                        self.buffer_string = String::new();
                        break;
                    }
                    _ => {
                        self.buffer_string.push(current_char);
                    }
                },
                LexerState::PERIOD => match current_char {
                    '0'..='9' => {
                        self.buffer_string.push('.');
                        self.buffer_string.push(current_char);
                        self.state = LexerState::DECIMALS
                    }

                    '.' => self.state = LexerState::RANGE,

                    _ => {
                        self.current_token = Token::POINT;
                        self.position -= 1;
                        self.state = LexerState::START;
                        break;
                    }
                },
                LexerState::RANGE => match current_char {
                    '=' => {
                        self.current_token = Token::RANGE_INCL;
                        self.state = LexerState::START;
                        break;
                    }

                    _ => {
                        self.current_token = Token::RANGE_EXCL;
                        self.position -= 1;
                        self.state = LexerState::START;
                        break;
                    }
                }
                LexerState::EQUAL => match current_char {
                    '>' => {
                        self.state = LexerState::START;
                        self.current_token = Token::BIG_ARROW;
                        break;
                    }
                    '=' => {
                        self.state = LexerState::START;
                        self.current_token = Token::EQUAL;
                        break;
                    }
                    _ => {
                        self.current_token = Token::ASSIGN;
                        self.position -= 1;
                        self.state = LexerState::START;
                        break;
                    }
                },
                LexerState::GREATER => match current_char {
                    '=' => {
                        self.state = LexerState::START;
                        self.current_token = Token::NLT;
                        break;
                    }
                    _ => {
                        self.current_token = Token::GT;
                        self.position -= 1;
                        self.state = LexerState::START;
                        break;
                    }
                },
                LexerState::LESS => match current_char {
                    '=' => {
                        self.state = LexerState::START;
                        self.current_token = Token::NGT;
                        break;
                    }
                    _ => {
                        self.current_token = Token::LT;
                        self.position -= 1;
                        self.state = LexerState::START;
                        break;
                    }
                },
                LexerState::PLUS => match current_char {
                    '+' => {
                        self.state = LexerState::START;
                        self.current_token = Token::INCREMENT;
                        break;
                    }
                    '=' => {
                        self.state = LexerState::START;
                        self.current_token = Token::ADD_ASSIGN;
                        break;
                    }
                    _ => {
                        self.state = LexerState::START;
                        self.current_token = Token::ADD;
                        self.position -= 1;
                        break;
                    }
                },
                LexerState::DASH => match current_char {
                    '-' => {
                        self.state = LexerState::START;
                        self.current_token = Token::DECREMENT;
                        break;
                    }
                    '=' => {
                        self.state = LexerState::START;
                        self.current_token = Token::SUB_ASSIGN;
                        break;
                    }
                    '>' => {
                        self.state = LexerState::START;
                        self.current_token = Token::ARROW;
                        break;
                    }
                    _ => {
                        self.state = LexerState::START;
                        self.current_token = Token::SUB;
                        self.position -= 1;
                        break;
                    }
                },
                LexerState::ASTERISK => match current_char {
                    '*' => {
                        self.state = LexerState::START;
                        self.current_token = Token::SQUARE;
                        break;
                    }
                    '=' => {
                        self.state = LexerState::START;
                        self.current_token = Token::MULT_ASSIGN;
                        break;
                    }
                    _ => {
                        self.state = LexerState::START;
                        self.current_token = Token::MULT;
                        self.position -= 1;
                        break;
                    }
                },
                LexerState::SLASH => match current_char {
                    '/' => self.state = LexerState::COMMENTS,
                    '*' => self.state = LexerState::BLOCK_COMMENT,
                    '=' => {
                        self.state = LexerState::START;
                        self.current_token = Token::DIV_ASSIGN;
                        break;
                    }

                    _ => {
                        self.state = LexerState::START;
                        self.current_token = Token::DIV;
                        self.position -= 1;
                        break;
                    }
                },
                LexerState::PERCENT => match current_char {
                    '=' => {
                        self.state = LexerState::START;
                        self.current_token = Token::REM_ASSIGN;
                        break;
                    }
                    _ => {
                        self.state = LexerState::START;
                        self.current_token = Token::REM;
                        self.position -= 1;
                        break;
                    }
                },
                LexerState::CARET => match current_char {
                    '/' => self.state = LexerState::ROOT,
                    '=' => {
                        self.state = LexerState::START;
                        self.current_token = Token::POWER_ASSIGN;
                        break;
                    }
                    '^' => {
                        self.state = LexerState::START;
                        self.current_token = Token::XOR;
                        break;
                    }
                    _ => {
                        self.state = LexerState::START;
                        self.current_token = Token::POWER;
                        self.position -= 1;
                        break;
                    }
                },
                LexerState::ROOT => match current_char {
                    '=' => {
                        self.state = LexerState::START;
                        self.current_token = Token::ROOT_ASSIGN;
                        break;
                    }
                    _ => {
                        self.state = LexerState::START;
                        self.current_token = Token::ROOT;
                        self.position -= 1;
                        break;
                    }
                },
                LexerState::EXCLAIM => match current_char {
                    '=' => {
                        self.state = LexerState::START;
                        self.current_token = Token::NEQ;
                        break;
                    }
                    _ => {
                        self.state = LexerState::START;
                        self.current_token = Token::NOT;
                        self.position -= 1;
                        break;
                    }
                },
                LexerState::QUESTION => match current_char {
                    '?' => {
                        self.state = LexerState::START;
                        self.current_token = Token::NULL_COAL;
                        break;
                    }
                    _ => {
                        self.state = LexerState::START;
                        self.current_token = Token::QUESTION;
                        self.position -= 1;
                        break;
                    }
                },
                LexerState::COMMENTS => {
                    if current_char == '\n' {
                        self.state = LexerState::START;
                    }
                },
                LexerState::BLOCK_COMMENT => {
                    if current_char == '*' {
                        self.state = LexerState::BLOCK_COMMENT2
                    }
                },
                LexerState::BLOCK_COMMENT2 => {
                    match current_char {
                        '/' => self.state = LexerState::START,
                        _ => self.state = LexerState::BLOCK_COMMENT,
                    }
                },
                LexerState::PIPE => match current_char {
                    '|' => {
                        self.state = LexerState::START;
                        self.current_token = Token::OR;
                        break;
                    }
                    _ => panic!("Unexpected character '|' in input!"),
                },
                LexerState::AMPERSAND => match current_char {
                    '&' => {
                        self.state = LexerState::START;
                        self.current_token = Token::AND;
                        break;
                    }
                    _ => panic!("Unexpected character '&' in input!"),
                },

                _ => {}
            }
        }

        self.current()
    }

    fn match_buffer_string(&mut self) -> Token {
        let string = self.buffer_string.as_str();
        match string {
            "import" => Token::IMPORT,
            "from" => Token::FROM,
            "as" => Token::AS,
            "public" => Token::PUBLIC,
            "protected" => Token::PROTECTED,
            "private" => Token::PRIVATE,
            "if" => Token::IF,
            "else" => Token::ELSE,
            "return" => Token::RETURN,
            "for" => Token::FOR,
            "each" => Token::EACH,
            "in" => Token::IN,
            "do" => Token::DO,
            "while" => Token::WHILE,
            "loop" => Token::LOOP,
            "continue" => Token::CONTINUE,
            "repeat" => Token::REPEAT,
            "break" => Token::BREAK,
            "match" => Token::MATCH,
            "default" => Token::DEFAULT,
            "defer" => Token::DEFER,
            "not" => Token::NOT,
            "and" => Token::AND,
            "or" => Token::OR,
            "xor" => Token::XOR,
            "class" => Token::CLASS,
            "implement" => Token::IMPL,
            "enum" => Token::ENUM,
            "extends" => Token::EXTENDS,
            "int" => Token::INT,
            "float" => Token::FLOAT,
            "char" => Token::CHAR,
            "string" => Token::STRING,
            "boolean" => Token::BOOLEAN,
            "function" => Token::FUNC,
            "let" => Token::LET,
            "true" | "false" => {
                let value: bool = string == "true";
                Token::LIT_BOOL { value }
            }
            "null" => Token::NULL,
            _ => {
                if string.contains('.') {
                    let value = string.parse::<f32>().unwrap();
                    if value.fract() != 0.0 {
                        return Token::LIT_FLOAT { value };
                    } else {
                        return Token::LIT_INT {
                            value: value as i32,
                        };
                    }
                }
                if let Ok(value) = string.parse::<i32>() {
                    return Token::LIT_INT { value };
                }

                return Token::ID {
                    name: string.to_string(),
                };
            }
        }
    }
}
