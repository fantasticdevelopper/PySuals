use anyhow::{Result, anyhow};
use crate::ast::*;
use std::iter::Peekable;

mod token;
mod grammar;
mod error;

use token::{Token, TokenType, TokenStream};
use error::ParseError;

pub struct Parser {
    tokens: Peekable<TokenStream>,
    current_line: usize,
    current_col: usize,
}

impl Parser {
    pub fn new(source: &str) -> Self {
        let tokens = TokenStream::new(source);
        Self {
            tokens: tokens.peekable(),
            current_line: 1,
            current_col: 1,
        }
    }
    
    pub fn parse(&mut self) -> Result<Program> {
        let mut program = Program::default();
        
        while let Some(token) = self.tokens.peek() {
            match token.token_type {
                TokenType::Component => {
                    let component = self.parse_component()?;
                    program.components.push(component);
                }
                TokenType::Def => {
                    let function = self.parse_function()?;
                    program.functions.push(function);
                }
                TokenType::Import => {
                    let import = self.parse_import()?;
                    program.imports.push(import);
                }
                TokenType::Export => {
                    let export = self.parse_export()?;
                    program.exports.push(export);
                }
                TokenType::Css => {
                    let css = self.parse_css()?;
                    program.css.push(css);
                }
                _ => {
                    self.tokens.next();
                }
            }
        }
        
        Ok(program)
    }
    
    fn parse_component(&mut self) -> Result<Component> {
        self.expect(TokenType::Component)?;
        
        let name = self.expect_ident()?;
        self.expect(TokenType::LParen)?;
        
        let mut params = Vec::new();
        if self.peek_token_type() != TokenType::RParen {
            params = self.parse_params()?;
        }
        
        self.expect(TokenType::RParen)?;
        self.expect(TokenType::Colon)?;
        
        let body = self.parse_block()?;
        
        Ok(Component {
            name,
            params,
            body,
            signals: Vec::new(),
            computed: Vec::new(),
            effects: Vec::new(),
            css_scope: None,
        })
    }
    
    fn parse_function(&mut self) -> Result<Function> {
        self.expect(TokenType::Def)?;
        
        let name = self.expect_ident()?;
        self.expect(TokenType::LParen)?;
        
        let mut params = Vec::new();
        if self.peek_token_type() != TokenType::RParen {
            params = self.parse_params()?;
        }
        
        self.expect(TokenType::RParen)?;
        self.expect(TokenType::Colon)?;
        
        let body = self.parse_block()?;
        
        Ok(Function {
            name,
            params,
            body,
            return_type: None,
            is_async: false,
            is_generator: false,
        })
    }
    
    fn parse_import(&mut self) -> Result<Import> {
        self.expect(TokenType::Import)?;
        
        let mut is_default = false;
        let mut names = Vec::new();
        let mut alias = None;
        
        if self.peek_token_type() == TokenType::Ident {
            let name = self.expect_ident()?;
            names.push(name);
            is_default = true;
        } else if self.peek_token_type() == TokenType::LBrace {
            self.expect(TokenType::LBrace)?;
            
            while self.peek_token_type() != TokenType::RBrace {
                let name = self.expect_ident()?;
                names.push(name);
                
                if self.peek_token_type() == TokenType::Comma {
                    self.expect(TokenType::Comma)?;
                }
            }
            
            self.expect(TokenType::RBrace)?;
        }
        
        self.expect(TokenType::From)?;
        
        let path = self.expect_string()?;
        
        Ok(Import {
            path,
            names,
            is_default,
            alias,
        })
    }
    
    fn parse_export(&mut self) -> Result<Export> {
        self.expect(TokenType::Export)?;
        
        let mut is_default = false;
        
        if self.peek_token_type() == TokenType::Default {
            self.expect(TokenType::Default)?;
            is_default = true;
        }
        
        let name = self.expect_ident()?;
        
        Ok(Export {
            name,
            is_default,
        })
    }
    
    fn parse_css(&mut self) -> Result<CssBlock> {
        self.expect(TokenType::Css)?;
        
        let selector = self.expect_string()?;
        self.expect(TokenType::LBrace)?;
        
        let mut rules = Vec::new();
        while self.peek_token_type() != TokenType::RBrace {
            let property = self.expect_ident()?;
            self.expect(TokenType::Colon)?;
            let value = self.expect_string()?;
            self.expect(TokenType::Semicolon)?;
            
            rules.push(CssRule {
                property,
                value,
            });
        }
        
        self.expect(TokenType::RBrace)?;
        
        Ok(CssBlock {
            selector,
            rules,
            scoped: true,
        })
    }
    
    fn parse_params(&mut self) -> Result<Vec<Param>> {
        let mut params = Vec::new();
        
        loop {
            let name = self.expect_ident()?;
            
            let type_hint = if self.peek_token_type() == TokenType::Colon {
                self.expect(TokenType::Colon)?;
                let type_name = self.expect_ident()?;
                Some(type_name)
            } else {
                None
            };
            
            let default = if self.peek_token_type() == TokenType::Eq {
                self.expect(TokenType::Eq)?;
                let expr = self.parse_expr()?;
                Some(expr)
            } else {
                None
            };
            
            params.push(Param {
                name,
                type_hint,
                default,
            });
            
            if self.peek_token_type() != TokenType::Comma {
                break;
            }
            self.expect(TokenType::Comma)?;
        }
        
        Ok(params)
    }
    
    fn parse_block(&mut self) -> Result<Vec<Stmt>> {
        let mut stmts = Vec::new();
        
        self.expect_indent()?;
        
        while self.current_indent() > 0 && !self.is_eof() {
            let stmt = self.parse_stmt()?;
            stmts.push(stmt);
        }
        
        self.expect_dedent()?;
        
        Ok(stmts)
    }
    
    fn parse_stmt(&mut self) -> Result<Stmt> {
        match self.peek_token_type() {
            TokenType::If => self.parse_if_stmt(),
            TokenType::For => self.parse_for_stmt(),
            TokenType::While => self.parse_while_stmt(),
            TokenType::Return => self.parse_return_stmt(),
            TokenType::Ident => self.parse_assign_or_expr(),
            _ => self.parse_expr_stmt(),
        }
    }
    
    fn parse_if_stmt(&mut self) -> Result<Stmt> {
        self.expect(TokenType::If)?;
        self.expect(TokenType::Space)?;
        let cond = self.parse_expr()?;
        self.expect(TokenType::Colon)?;
        let body = self.parse_block()?;
        
        Ok(Stmt::If(IfStmt {
            cond,
            body,
            else_if: Vec::new(),
            else_body: None,
        }))
    }
    
    fn parse_for_stmt(&mut self) -> Result<Stmt> {
        self.expect(TokenType::For)?;
        self.expect(TokenType::Space)?;
        
        let item = self.expect_ident()?;
        self.expect(TokenType::In)?;
        let iter = self.parse_expr()?;
        self.expect(TokenType::Colon)?;
        let body = self.parse_block()?;
        
        Ok(Stmt::For(ForStmt {
            item,
            iter,
            body,
            key: None,
        }))
    }
    
    fn parse_while_stmt(&mut self) -> Result<Stmt> {
        self.expect(TokenType::While)?;
        self.expect(TokenType::Space)?;
        let cond = self.parse_expr()?;
        self.expect(TokenType::Colon)?;
        let body = self.parse_block()?;
        
        Ok(Stmt::While(WhileStmt {
            cond,
            body,
        }))
    }
    
    fn parse_return_stmt(&mut self) -> Result<Stmt> {
        self.expect(TokenType::Return)?;
        let value = self.parse_expr()?;
        Ok(Stmt::Return(value))
    }
    
    fn parse_assign_or_expr(&mut self) -> Result<Stmt> {
        let name = self.expect_ident()?;
        
        if self.peek_token_type() == TokenType::Eq {
            self.expect(TokenType::Eq)?;
            let value = self.parse_expr()?;
            Ok(Stmt::Assign(AssignStmt {
                target: name,
                value,
                op: AssignOp::Eq,
            }))
        } else {
            let expr = self.parse_expr_from_ident(name)?;
            Ok(Stmt::Expr(expr))
        }
    }
    
    fn parse_expr_stmt(&mut self) -> Result<Stmt> {
        let expr = self.parse_expr()?;
        Ok(Stmt::Expr(expr))
    }
    
    fn parse_expr(&mut self) -> Result<Expr> {
        self.parse_logical_or()
    }
    
    fn parse_logical_or(&mut self) -> Result<Expr> {
        let mut expr = self.parse_logical_and()?;
        
        while let TokenType::Or = self.peek_token_type() {
            self.expect(TokenType::Or)?;
            let right = self.parse_logical_and()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                right: Box::new(right),
                op: BinaryOp::Or,
            });
        }
        
        Ok(expr)
    }
    
    fn parse_logical_and(&mut self) -> Result<Expr> {
        let mut expr = self.parse_equality()?;
        
        while let TokenType::And = self.peek_token_type() {
            self.expect(TokenType::And)?;
            let right = self.parse_equality()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                right: Box::new(right),
                op: BinaryOp::And,
            });
        }
        
        Ok(expr)
    }
    
    fn parse_equality(&mut self) -> Result<Expr> {
        let mut expr = self.parse_comparison()?;
        
        while let op @ (TokenType::EqEq | TokenType::NotEq) = self.peek_token_type() {
            self.next_token();
            let right = self.parse_comparison()?;
            let bin_op = match op {
                TokenType::EqEq => BinaryOp::Eq,
                TokenType::NotEq => BinaryOp::NotEq,
                _ => unreachable!(),
            };
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                right: Box::new(right),
                op: bin_op,
            });
        }
        
        Ok(expr)
    }
    
    fn parse_comparison(&mut self) -> Result<Expr> {
        let mut expr = self.parse_addition()?;
        
        while let op @ (TokenType::Lt | TokenType::Gt | TokenType::LtEq | TokenType::GtEq) = self.peek_token_type() {
            self.next_token();
            let right = self.parse_addition()?;
            let bin_op = match op {
                TokenType::Lt => BinaryOp::Lt,
                TokenType::Gt => BinaryOp::Gt,
                TokenType::LtEq => BinaryOp::LtEq,
                TokenType::GtEq => BinaryOp::GtEq,
                _ => unreachable!(),
            };
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                right: Box::new(right),
                op: bin_op,
            });
        }
        
        Ok(expr)
    }
    
    fn parse_addition(&mut self) -> Result<Expr> {
        let mut expr = self.parse_multiplication()?;
        
        while let op @ (TokenType::Plus | TokenType::Minus) = self.peek_token_type() {
            self.next_token();
            let right = self.parse_multiplication()?;
            let bin_op = match op {
                TokenType::Plus => BinaryOp::Add,
                TokenType::Minus => BinaryOp::Sub,
                _ => unreachable!(),
            };
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                right: Box::new(right),
                op: bin_op,
            });
        }
        
        Ok(expr)
    }
    
    fn parse_multiplication(&mut self) -> Result<Expr> {
        let mut expr = self.parse_unary()?;
        
        while let op @ (TokenType::Star | TokenType::Slash | TokenType::Percent) = self.peek_token_type() {
            self.next_token();
            let right = self.parse_unary()?;
            let bin_op = match op {
                TokenType::Star => BinaryOp::Mul,
                TokenType::Slash => BinaryOp::Div,
                TokenType::Percent => BinaryOp::Mod,
                _ => unreachable!(),
            };
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                right: Box::new(right),
                op: bin_op,
            });
        }
        
        Ok(expr)
    }
    
    fn parse_unary(&mut self) -> Result<Expr> {
        match self.peek_token_type() {
            TokenType::Not => {
                self.next_token();
                let expr = self.parse_unary()?;
                Ok(Expr::Unary(UnaryExpr {
                    op: UnaryOp::Not,
                    expr: Box::new(expr),
                }))
            }
            TokenType::Minus => {
                self.next_token();
                let expr = self.parse_unary()?;
                Ok(Expr::Unary(UnaryExpr {
                    op: UnaryOp::Neg,
                    expr: Box::new(expr),
                }))
            }
            _ => self.parse_primary(),
        }
    }
    
    fn parse_primary(&mut self) -> Result<Expr> {
        match self.peek_token_type() {
            TokenType::Number => {
                let value = self.expect_number()?;
                Ok(Expr::Literal(LiteralExpr {
                    value: LiteralValue::Number(value),
                    raw: value.to_string(),
                }))
            }
            TokenType::String => {
                let value = self.expect_string()?;
                Ok(Expr::Literal(LiteralExpr {
                    value: LiteralValue::String(value.clone()),
                    raw: value,
                }))
            }
            TokenType::True => {
                self.next_token();
                Ok(Expr::Literal(LiteralExpr {
                    value: LiteralValue::Boolean(true),
                    raw: "true".to_string(),
                }))
            }
            TokenType::False => {
                self.next_token();
                Ok(Expr::Literal(LiteralExpr {
                    value: LiteralValue::Boolean(false),
                    raw: "false".to_string(),
                }))
            }
            TokenType::Null => {
                self.next_token();
                Ok(Expr::Literal(LiteralExpr {
                    value: LiteralValue::Null,
                    raw: "null".to_string(),
                }))
            }
            TokenType::Ident => {
                let name = self.expect_ident()?;
                self.parse_expr_from_ident(name)
            }
            TokenType::LParen => {
                self.expect(TokenType::LParen)?;
                let expr = self.parse_expr()?;
                self.expect(TokenType::RParen)?;
                Ok(expr)
            }
            _ => Err(anyhow!("Unexpected token")),
        }
    }
    
    fn parse_expr_from_ident(&mut self, name: String) -> Result<Expr> {
        if self.peek_token_type() == TokenType::LParen {
            self.expect(TokenType::LParen)?;
            let mut args = Vec::new();
            
            if self.peek_token_type() != TokenType::RParen {
                loop {
                    let arg = self.parse_expr()?;
                    args.push(arg);
                    if self.peek_token_type() != TokenType::Comma {
                        break;
                    }
                    self.expect(TokenType::Comma)?;
                }
            }
            
            self.expect(TokenType::RParen)?;
            
            Ok(Expr::Call(CallExpr {
                callee: Box::new(Expr::Ident(IdentExpr { name, global: false })),
                args,
            }))
        } else {
            Ok(Expr::Ident(IdentExpr { name, global: false }))
        }
    }
    
    fn expect(&mut self, expected: TokenType) -> Result<()> {
        let token = self.tokens.next();
        match token {
            Some(t) if t.token_type == expected => Ok(()),
            Some(t) => Err(anyhow!("Expected {:?}, got {:?}", expected, t.token_type)),
            None => Err(anyhow!("Unexpected EOF")),
        }
    }
    
    fn expect_ident(&mut self) -> Result<String> {
        let token = self.tokens.next();
        match token {
            Some(t) if t.token_type == TokenType::Ident => Ok(t.value),
            Some(t) => Err(anyhow!("Expected ident, got {:?}", t.token_type)),
            None => Err(anyhow!("Unexpected EOF")),
        }
    }
    
    fn expect_string(&mut self) -> Result<String> {
        let token = self.tokens.next();
        match token {
            Some(t) if t.token_type == TokenType::String => Ok(t.value),
            Some(t) => Err(anyhow!("Expected string, got {:?}", t.token_type)),
            None => Err(anyhow!("Unexpected EOF")),
        }
    }
    
    fn expect_number(&mut self) -> Result<f64> {
        let token = self.tokens.next();
        match token {
            Some(t) if t.token_type == TokenType::Number => {
                t.value.parse::<f64>().map_err(|_| anyhow!("Invalid number"))
            }
            Some(t) => Err(anyhow!("Expected number, got {:?}", t.token_type)),
            None => Err(anyhow!("Unexpected EOF")),
        }
    }
    
    fn expect_indent(&mut self) -> Result<()> {
        match self.peek_token_type() {
            TokenType::Indent => {
                self.next_token();
                Ok(())
            }
            _ => Ok(()),
        }
    }
    
    fn expect_dedent(&mut self) -> Result<()> {
        match self.peek_token_type() {
            TokenType::Dedent => {
                self.next_token();
                Ok(())
            }
            _ => Ok(()),
        }
    }
    
    fn peek_token_type(&mut self) -> TokenType {
        self.tokens.peek().map_or(TokenType::Eof, |t| t.token_type)
    }
    
    fn current_indent(&self) -> usize {
        0
    }
    
    fn is_eof(&self) -> bool {
        self.tokens.peek().is_none()
    }
    
    fn next_token(&mut self) {
        self.tokens.next();
    }
}

pub fn parse(source: &str) -> Result<Program> {
    let mut parser = Parser::new(source);
    parser.parse()
      }
