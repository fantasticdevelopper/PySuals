use std::collections::{HashMap, HashSet};
use crate::ast::*;

#[derive(Debug, Clone)]
pub struct Scope {
    pub name: String,
    pub parent: Option<usize>,
    pub symbols: HashMap<String, Symbol>,
    pub children: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub defined_at: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Component,
    Function,
    Signal,
    Effect,
    Computed,
    Variable,
    Parameter,
    Import,
}

pub struct ScopeAnalyzer {
    scopes: Vec<Scope>,
    current_scope: usize,
    errors: Vec<String>,
    symbols: Vec<Symbol>,
}

impl ScopeAnalyzer {
    pub fn new() -> Self {
        let global_scope = Scope {
            name: "global".to_string(),
            parent: None,
            symbols: HashMap::new(),
            children: Vec::new(),
        };
        
        Self {
            scopes: vec![global_scope],
            current_scope: 0,
            errors: Vec::new(),
            symbols: Vec::new(),
        }
    }
    
    pub fn analyze(&mut self, program: &Program) {
        self.analyze_program(program);
    }
    
    fn analyze_program(&mut self, program: &Program) {
        for component in &program.components {
            self.enter_scope(&component.name);
            self.analyze_component(component);
            self.exit_scope();
        }
        
        for function in &program.functions {
            self.enter_scope(&function.name);
            self.analyze_function(function);
            self.exit_scope();
        }
        
        for import in &program.imports {
            self.analyze_import(import);
        }
    }
    
    fn analyze_component(&mut self, component: &Component) {
        self.define_symbol(&component.name, SymbolKind::Component);
        
        for param in &component.params {
            self.define_symbol(&param.name, SymbolKind::Parameter);
        }
        
        for signal in &component.signals {
            self.define_symbol(&signal.name, SymbolKind::Signal);
        }
        
        for computed in &component.computed {
            self.define_symbol(&computed.name, SymbolKind::Computed);
        }
        
        for stmt in &component.body {
            self.analyze_stmt(stmt);
        }
    }
    
    fn analyze_function(&mut self, function: &Function) {
        self.define_symbol(&function.name, SymbolKind::Function);
        
        for param in &function.params {
            self.define_symbol(&param.name, SymbolKind::Parameter);
        }
        
        for stmt in &function.body {
            self.analyze_stmt(stmt);
        }
    }
    
    fn analyze_import(&mut self, import: &Import) {
        for name in &import.names {
            self.define_symbol(name, SymbolKind::Import);
        }
    }
    
    fn analyze_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Expr(expr) => self.analyze_expr(expr),
            Stmt::Return(expr) => self.analyze_expr(expr),
            Stmt::If(if_stmt) => self.analyze_if_stmt(if_stmt),
            Stmt::For(for_stmt) => self.analyze_for_stmt(for_stmt),
            Stmt::While(while_stmt) => self.analyze_while_stmt(while_stmt),
            Stmt::Assign(assign) => self.analyze_assign_stmt(assign),
            Stmt::Var(var) => self.analyze_var_stmt(var),
            Stmt::Block(stmts) => {
                for s in stmts {
                    self.analyze_stmt(s);
                }
            }
        }
    }
    
    fn analyze_if_stmt(&mut self, if_stmt: &IfStmt) {
        self.analyze_expr(&if_stmt.cond);
        
        for stmt in &if_stmt.body {
            self.analyze_stmt(stmt);
        }
        
        for (cond, body) in &if_stmt.else_if {
            self.analyze_expr(cond);
            for stmt in body {
                self.analyze_stmt(stmt);
            }
        }
        
        if let Some(else_body) = &if_stmt.else_body {
            for stmt in else_body {
                self.analyze_stmt(stmt);
            }
        }
    }
    
    fn analyze_for_stmt(&mut self, for_stmt: &ForStmt) {
        self.analyze_expr(&for_stmt.iter);
        
        self.enter_scope("for");
        self.define_symbol(&for_stmt.item, SymbolKind::Variable);
        
        for stmt in &for_stmt.body {
            self.analyze_stmt(stmt);
        }
        
        self.exit_scope();
    }
    
    fn analyze_while_stmt(&mut self, while_stmt: &WhileStmt) {
        self.analyze_expr(&while_stmt.cond);
        
        for stmt in &while_stmt.body {
            self.analyze_stmt(stmt);
        }
    }
    
    fn analyze_assign_stmt(&mut self, assign: &AssignStmt) {
        if !self.symbol_exists(&assign.target) {
            self.define_symbol(&assign.target, SymbolKind::Variable);
        }
        self.analyze_expr(&assign.value);
    }
    
    fn analyze_var_stmt(&mut self, var: &VarStmt) {
        self.define_symbol(&var.name, SymbolKind::Variable);
        self.analyze_expr(&var.value);
    }
    
    fn analyze_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Literal(_) => {}
            Expr::Ident(ident) => {
                if !self.symbol_exists(&ident.name) {
                    self.errors.push(format!("Undefined variable: {}", ident.name));
                }
            }
            Expr::Call(call) => {
                self.analyze_expr(&call.callee);
                for arg in &call.args {
                    self.analyze_expr(arg);
                }
            }
            Expr::Binary(binary) => {
                self.analyze_expr(&binary.left);
                self.analyze_expr(&binary.right);
            }
            Expr::Unary(unary) => {
                self.analyze_expr(&unary.expr);
            }
            Expr::Member(member) => {
                self.analyze_expr(&member.obj);
            }
            Expr::Object(object) => {
                for prop in &object.props {
                    self.analyze_expr(&prop.value);
                }
            }
            Expr::Array(array) => {
                for elem in &array.elements {
                    self.analyze_expr(elem);
                }
            }
            Expr::Lambda(lambda) => {
                self.analyze_expr(&lambda.body);
            }
            Expr::Ternary(cond, then_expr, else_expr) => {
                self.analyze_expr(cond);
                self.analyze_expr(then_expr);
                self.analyze_expr(else_expr);
            }
            Expr::Template(template) => {
                for expr in &template.expressions {
                    self.analyze_expr(expr);
                }
            }
            Expr::Await(await_expr) => {
                self.analyze_expr(&await_expr.expr);
            }
            Expr::Spread(spread) => {
                self.analyze_expr(&spread.expr);
            }
        }
    }
    
    fn enter_scope(&mut self, name: &str) {
        let scope_id = self.scopes.len();
        let scope = Scope {
            name: name.to_string(),
            parent: Some(self.current_scope),
            symbols: HashMap::new(),
            children: Vec::new(),
        };
        
        if let Some(parent) = self.scopes.get_mut(self.current_scope) {
            parent.children.push(scope_id);
        }
        
        self.scopes.push(scope);
        self.current_scope = scope_id;
    }
    
    fn exit_scope(&mut self) {
        if let Some(parent) = self.scopes[self.current_scope].parent {
            self.current_scope = parent;
        }
    }
    
    fn define_symbol(&mut self, name: &str, kind: SymbolKind) {
        let symbol = Symbol {
            name: name.to_string(),
            kind,
            defined_at: self.current_scope,
        };
        
        self.symbols.push(symbol.clone());
        
        if let Some(scope) = self.scopes.get_mut(self.current_scope) {
            scope.symbols.insert(name.to_string(), symbol);
        }
    }
    
    fn symbol_exists(&self, name: &str) -> bool {
        let mut current = Some(self.current_scope);
        
        while let Some(scope_id) = current {
            if let Some(scope) = self.scopes.get(scope_id) {
                if scope.symbols.contains_key(name) {
                    return true;
                }
                current = scope.parent;
            } else {
                break;
            }
        }
        
        false
    }
    
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    
    pub fn get_errors(&self) -> Vec<String> {
        self.errors.clone()
    }
    
    pub fn validate(&self, _program: &Program) -> bool {
        !self.has_errors()
    }
    
    pub fn get_symbols(&self) -> Vec<Symbol> {
        self.symbols.clone()
    }
}
