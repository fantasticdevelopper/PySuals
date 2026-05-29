use anyhow::Result;
use crate::ast::*;

mod scope;
mod typeck;
mod borrow;

use scope::ScopeAnalyzer;
use typeck::TypeChecker;
use borrow::BorrowChecker;

pub struct Analyzer {
    scope_analyzer: ScopeAnalyzer,
    type_checker: TypeChecker,
    borrow_checker: BorrowChecker,
    errors: Vec<String>,
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            scope_analyzer: ScopeAnalyzer::new(),
            type_checker: TypeChecker::new(),
            borrow_checker: BorrowChecker::new(),
            errors: Vec::new(),
        }
    }
    
    pub fn analyze(&mut self, program: Program) -> Result<Program> {
        self.scope_analyzer.analyze(&program);
        self.type_checker.check(&program);
        self.borrow_checker.check(&program);
        
        if self.scope_analyzer.has_errors() {
            self.errors.extend(self.scope_analyzer.get_errors());
        }
        
        if self.type_checker.has_errors() {
            self.errors.extend(self.type_checker.get_errors());
        }
        
        if self.borrow_checker.has_errors() {
            self.errors.extend(self.borrow_checker.get_errors());
        }
        
        if !self.errors.is_empty() {
            return Err(anyhow::anyhow!("Analysis failed: {:?}", self.errors));
        }
        
        Ok(program)
    }
    
    pub fn validate(&self, program: &Program) -> bool {
        self.scope_analyzer.validate(program)
            && self.type_checker.validate(program)
            && self.borrow_checker.validate(program)
    }
    
    pub fn get_symbols(&self) -> Vec<Symbol> {
        self.scope_analyzer.get_symbols()
    }
    
    pub fn get_type_info(&self, expr: &Expr) -> Option<TypeInfo> {
        self.type_checker.get_type(expr)
    }
}

pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub scope: String,
    pub type_hint: Option<String>,
}

pub enum SymbolKind {
    Component,
    Function,
    Signal,
    Effect,
    Variable,
    Parameter,
}

pub struct TypeInfo {
    pub type_name: String,
    pub nullable: bool,
    pub optional: bool,
}

pub fn analyze(program: Program) -> Result<Program> {
    let mut analyzer = Analyzer::new();
    analyzer.analyze(program)
}

pub fn validate(program: &Program) -> bool {
    let analyzer = Analyzer::new();
    analyzer.validate(program)
}
