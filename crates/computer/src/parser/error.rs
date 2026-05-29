use std::fmt;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub error_type: ParseErrorType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseErrorType {
    UnexpectedToken,
    ExpectedToken,
    InvalidSyntax,
    IndentationError,
    UnterminatedString,
    InvalidNumber,
    UndefinedVariable,
    TypeMismatch,
    CyclicDependency,
}

impl ParseError {
    pub fn new(message: String, line: usize, column: usize, error_type: ParseErrorType) -> Self {
        Self {
            message,
            line,
            column,
            error_type,
        }
    }
    
    pub fn unexpected_token(line: usize, column: usize, found: &str, expected: &str) -> Self {
        Self::new(
            format!("Unexpected token '{}', expected {}", found, expected),
            line,
            column,
            ParseErrorType::UnexpectedToken,
        )
    }
    
    pub fn expected_token(line: usize, column: usize, expected: &str) -> Self {
        Self::new(
            format!("Expected {}", expected),
            line,
            column,
            ParseErrorType::ExpectedToken,
        )
    }
    
    pub fn invalid_syntax(line: usize, column: usize, message: &str) -> Self {
        Self::new(
            format!("Invalid syntax: {}", message),
            line,
            column,
            ParseErrorType::InvalidSyntax,
        )
    }
    
    pub fn indentation_error(line: usize, column: usize) -> Self {
        Self::new(
            "Indentation error".to_string(),
            line,
            column,
            ParseErrorType::IndentationError,
        )
    }
    
    pub fn unterminated_string(line: usize, column: usize) -> Self {
        Self::new(
            "Unterminated string literal".to_string(),
            line,
            column,
            ParseErrorType::UnterminatedString,
        )
    }
    
    pub fn invalid_number(line: usize, column: usize, value: &str) -> Self {
        Self::new(
            format!("Invalid number literal: {}", value),
            line,
            column,
            ParseErrorType::InvalidNumber,
        )
    }
    
    pub fn undefined_variable(line: usize, column: usize, name: &str) -> Self {
        Self::new(
            format!("Undefined variable: {}", name),
            line,
            column,
            ParseErrorType::UndefinedVariable,
        )
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} at {}:{}: {}",
            self.error_type, self.line, self.column, self.message
        )
    }
}

impl Error for ParseError {}

impl fmt::Display for ParseErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseErrorType::UnexpectedToken => write!(f, "UnexpectedToken"),
            ParseErrorType::ExpectedToken => write!(f, "ExpectedToken"),
            ParseErrorType::InvalidSyntax => write!(f, "InvalidSyntax"),
            ParseErrorType::IndentationError => write!(f, "IndentationError"),
            ParseErrorType::UnterminatedString => write!(f, "UnterminatedString"),
            ParseErrorType::InvalidNumber => write!(f, "InvalidNumber"),
            ParseErrorType::UndefinedVariable => write!(f, "UndefinedVariable"),
            ParseErrorType::TypeMismatch => write!(f, "TypeMismatch"),
            ParseErrorType::CyclicDependency => write!(f, "CyclicDependency"),
        }
    }
}

pub struct ErrorCollector {
    errors: Vec<ParseError>,
}

impl ErrorCollector {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
        }
    }
    
    pub fn add(&mut self, error: ParseError) {
        self.errors.push(error);
    }
    
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    
    pub fn get_errors(&self) -> &[ParseError] {
        &self.errors
    }
    
    pub fn clear(&mut self) {
        self.errors.clear();
    }
    
    pub fn into_result(self) -> Result<(), Vec<ParseError>> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors)
        }
    }
    
    pub fn print_errors(&self, source: &str) {
        for error in &self.errors {
            eprintln!("\n{}", error);
            
            let lines: Vec<&str> = source.lines().collect();
            if error.line <= lines.len() {
                let line = lines[error.line - 1];
                eprintln!("    {}", line);
                eprintln!("    {}{}", " ".repeat(error.column - 1), "^");
            }
        }
    }
}

pub struct RecoveryParser {
    errors: ErrorCollector,
    in_error_mode: bool,
}

impl RecoveryParser {
    pub fn new() -> Self {
        Self {
            errors: ErrorCollector::new(),
            in_error_mode: false,
        }
    }
    
    pub fn enter_error_mode(&mut self) {
        self.in_error_mode = true;
    }
    
    pub fn exit_error_mode(&mut self) {
        self.in_error_mode = false;
    }
    
    pub fn is_in_error_mode(&self) -> bool {
        self.in_error_mode
    }
    
    pub fn recover(&mut self, tokens: &[super::token::TokenType], position: usize) -> usize {
        let mut new_pos = position;
        
        while new_pos < tokens.len() {
            if self.is_synchronization_point(&tokens[new_pos]) {
                break;
            }
            new_pos += 1;
        }
        
        self.in_error_mode = false;
        new_pos
    }
    
    fn is_synchronization_point(&self, token_type: &super::token::TokenType) -> bool {
        matches!(
            token_type,
            super::token::TokenType::Component
                | super::token::TokenType::Def
                | super::token::TokenType::Import
                | super::token::TokenType::Export
                | super::token::TokenType::RBrace
                | super::token::TokenType::Eof
        )
    }
    
    pub fn add_error(&mut self, error: ParseError) {
        self.errors.add(error);
    }
    
    pub fn get_errors(&self) -> &[ParseError] {
        self.errors.get_errors()
    }
    
    pub fn has_errors(&self) -> bool {
        self.errors.has_errors()
    }
    
    pub fn print_errors(&self, source: &str) {
        self.errors.print_errors(source);
    }
}

impl Default for RecoveryParser {
    fn default() -> Self {
        Self::new()
    }
}
