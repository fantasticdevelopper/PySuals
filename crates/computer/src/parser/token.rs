use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Component,
    Signal,
    Effect,
    Computed,
    Def,
    Return,
    If,
    Else,
    For,
    In,
    While,
    Import,
    From,
    Export,
    Default,
    Css,
    
    Ident,
    String,
    Number,
    
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Colon,
    Semicolon,
    Comma,
    Dot,
    
    Eq,
    EqEq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    And,
    Or,
    Not,
    
    Arrow,
    Space,
    Indent,
    Dedent,
    Newline,
    
    True,
    False,
    Null,
    
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, value: String, line: usize, column: usize) -> Self {
        Self {
            token_type,
            value,
            line,
            column,
        }
    }
}

pub struct TokenStream {
    tokens: Vec<Token>,
    position: usize,
}

impl TokenStream {
    pub fn new(source: &str) -> Self {
        let tokens = Lexer::new(source).tokenize();
        Self {
            tokens,
            position: 0,
        }
    }
    
    pub fn next(&mut self) -> Option<Token> {
        if self.position < self.tokens.len() {
            let token = self.tokens[self.position].clone();
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }
    
    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }
}

impl Iterator for TokenStream {
    type Item = Token;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

struct Lexer {
    chars: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            chars: source.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }
    
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        
        while let Some(token) = self.next_token() {
            tokens.push(token);
            if let TokenType::Eof = token.token_type {
                break;
            }
        }
        
        tokens
    }
    
    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        
        if self.is_eof() {
            return Some(Token::new(TokenType::Eof, String::new(), self.line, self.column));
        }
        
        let ch = self.current_char();
        
        match ch {
            '(' => {
                self.advance();
                Some(Token::new(TokenType::LParen, "(".to_string(), self.line, self.column - 1))
            }
            ')' => {
                self.advance();
                Some(Token::new(TokenType::RParen, ")".to_string(), self.line, self.column - 1))
            }
            '{' => {
                self.advance();
                Some(Token::new(TokenType::LBrace, "{".to_string(), self.line, self.column - 1))
            }
            '}' => {
                self.advance();
                Some(Token::new(TokenType::RBrace, "}".to_string(), self.line, self.column - 1))
            }
            '[' => {
                self.advance();
                Some(Token::new(TokenType::LBracket, "[".to_string(), self.line, self.column - 1))
            }
            ']' => {
                self.advance();
                Some(Token::new(TokenType::RBracket, "]".to_string(), self.line, self.column - 1))
            }
            ':' => {
                self.advance();
                Some(Token::new(TokenType::Colon, ":".to_string(), self.line, self.column - 1))
            }
            ';' => {
                self.advance();
                Some(Token::new(TokenType::Semicolon, ";".to_string(), self.line, self.column - 1))
            }
            ',' => {
                self.advance();
                Some(Token::new(TokenType::Comma, ",".to_string(), self.line, self.column - 1))
            }
            '.' => {
                self.advance();
                Some(Token::new(TokenType::Dot, ".".to_string(), self.line, self.column - 1))
            }
            '=' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    Some(Token::new(TokenType::EqEq, "==".to_string(), self.line, self.column - 2))
                } else if self.current_char() == '>' {
                    self.advance();
                    Some(Token::new(TokenType::Arrow, "=>".to_string(), self.line, self.column - 2))
                } else {
                    Some(Token::new(TokenType::Eq, "=".to_string(), self.line, self.column - 1))
                }
            }
            '!' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    Some(Token::new(TokenType::NotEq, "!=".to_string(), self.line, self.column - 2))
                } else {
                    Some(Token::new(TokenType::Not, "!".to_string(), self.line, self.column - 1))
                }
            }
            '<' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    Some(Token::new(TokenType::LtEq, "<=".to_string(), self.line, self.column - 2))
                } else {
                    Some(Token::new(TokenType::Lt, "<".to_string(), self.line, self.column - 1))
                }
            }
            '>' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    Some(Token::new(TokenType::GtEq, ">=".to_string(), self.line, self.column - 2))
                } else {
                    Some(Token::new(TokenType::Gt, ">".to_string(), self.line, self.column - 1))
                }
            }
            '+' => {
                self.advance();
                Some(Token::new(TokenType::Plus, "+".to_string(), self.line, self.column - 1))
            }
            '-' => {
                self.advance();
                Some(Token::new(TokenType::Minus, "-".to_string(), self.line, self.column - 1))
            }
            '*' => {
                self.advance();
                Some(Token::new(TokenType::Star, "*".to_string(), self.line, self.column - 1))
            }
            '/' => {
                self.advance();
                Some(Token::new(TokenType::Slash, "/".to_string(), self.line, self.column - 1))
            }
            '%' => {
                self.advance();
                Some(Token::new(TokenType::Percent, "%".to_string(), self.line, self.column - 1))
            }
            '&' => {
                self.advance();
                if self.current_char() == '&' {
                    self.advance();
                    Some(Token::new(TokenType::And, "&&".to_string(), self.line, self.column - 2))
                } else {
                    self.error("Invalid token");
                    None
                }
            }
            '|' => {
                self.advance();
                if self.current_char() == '|' {
                    self.advance();
                    Some(Token::new(TokenType::Or, "||".to_string(), self.line, self.column - 2))
                } else {
                    self.error("Invalid token");
                    None
                }
            }
            '"' | '\'' => {
                self.read_string()
            }
            _ if ch.is_alphabetic() || ch == '_' => {
                self.read_identifier()
            }
            _ if ch.is_numeric() => {
                self.read_number()
            }
            _ => {
                self.error(&format!("Unexpected character: {}", ch));
                None
            }
        }
    }
    
    fn read_identifier(&mut self) -> Option<Token> {
        let start_col = self.column;
        let mut value = String::new();
        
        while !self.is_eof() && (self.current_char().is_alphanumeric() || self.current_char() == '_') {
            value.push(self.current_char());
            self.advance();
        }
        
        let token_type = match value.as_str() {
            "component" => TokenType::Component,
            "signal" => TokenType::Signal,
            "effect" => TokenType::Effect,
            "computed" => TokenType::Computed,
            "def" => TokenType::Def,
            "return" => TokenType::Return,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "for" => TokenType::For,
            "in" => TokenType::In,
            "while" => TokenType::While,
            "import" => TokenType::Import,
            "from" => TokenType::From,
            "export" => TokenType::Export,
            "default" => TokenType::Default,
            "css" => TokenType::Css,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "null" => TokenType::Null,
            _ => TokenType::Ident,
        };
        
        Some(Token::new(token_type, value, self.line, start_col))
    }
    
    fn read_number(&mut self) -> Option<Token> {
        let start_col = self.column;
        let mut value = String::new();
        
        while !self.is_eof() && (self.current_char().is_numeric() || self.current_char() == '.') {
            value.push(self.current_char());
            self.advance();
        }
        
        Some(Token::new(TokenType::Number, value, self.line, start_col))
    }
    
    fn read_string(&mut self) -> Option<Token> {
        let quote = self.current_char();
        let start_col = self.column;
        self.advance();
        
        let mut value = String::new();
        
        while !self.is_eof() && self.current_char() != quote {
            if self.current_char() == '\\' {
                self.advance();
                match self.current_char() {
                    'n' => value.push('\n'),
                    't' => value.push('\t'),
                    'r' => value.push('\r'),
                    '"' => value.push('"'),
                    '\'' => value.push('\''),
                    '\\' => value.push('\\'),
                    _ => value.push(self.current_char()),
                }
            } else {
                value.push(self.current_char());
            }
            self.advance();
        }
        
        self.advance();
        
        Some(Token::new(TokenType::String, value, self.line, start_col))
    }
    
    fn skip_whitespace(&mut self) {
        while !self.is_eof() {
            match self.current_char() {
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                '\n' => {
                    self.advance();
                    self.line += 1;
                    self.column = 1;
                }
                '#' => {
                    self.skip_comment();
                }
                _ => break,
            }
        }
    }
    
    fn skip_comment(&mut self) {
        while !self.is_eof() && self.current_char() != '\n' {
            self.advance();
        }
    }
    
    fn current_char(&self) -> char {
        if self.position < self.chars.len() {
            self.chars[self.position]
        } else {
            '\0'
        }
    }
    
    fn advance(&mut self) {
        if !self.is_eof() {
            self.position += 1;
            self.column += 1;
        }
    }
    
    fn is_eof(&self) -> bool {
        self.position >= self.chars.len()
    }
    
    fn error(&mut self, message: &str) {
        eprintln!("Lexer error at {}:{}: {}", self.line, self.column, message);
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
              }
