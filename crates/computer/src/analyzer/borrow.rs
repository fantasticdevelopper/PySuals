use std::collections::{HashMap, HashSet};
use crate::ast::*;

#[derive(Debug, Clone, PartialEq)]
pub enum BorrowKind {
    Shared,
    Mutable,
    Owned,
}

#[derive(Debug, Clone)]
pub struct Borrow {
    pub name: String,
    pub kind: BorrowKind,
    pub scope: usize,
}

pub struct BorrowChecker {
    borrows: HashMap<String, Borrow>,
    signals: HashSet<String>,
    errors: Vec<String>,
    current_scope: usize,
}

impl BorrowChecker {
    pub fn new() -> Self {
        Self {
            borrows: HashMap::new(),
            signals: HashSet::new(),
            errors: Vec::new(),
            current_scope: 0,
        }
    }
    
    pub fn check(&mut self, program: &Program) {
        for component in &program.components {
            self.check_component(component);
        }
    }
    
    fn check_component(&mut self, component: &Component) {
        for signal in &component.signals {
            self.signals.insert(signal.name.clone());
        }
        
        for stmt in &component.body {
            self.check_stmt(stmt);
        }
    }
    
    fn check_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Expr(expr) => self.check_expr(expr),
            Stmt::Assign(assign) => self.check_assign(assign),
            Stmt::Var(var) => self.check_var(var),
            Stmt::If(if_stmt) => self.check_if(if_stmt),
            Stmt::For(for_stmt) => self.check_for(for_stmt),
            Stmt::While(while_stmt) => self.check_while(while_stmt),
            Stmt::Block(block) => {
                for s in block {
                    self.check_stmt(s);
                }
            }
            _ => {}
        }
    }
    
    fn check_assign(&mut self, assign: &AssignStmt) {
        if self.signals.contains(&assign.target) {
            if let Some(borrow) = self.borrows.get(&assign.target) {
                if borrow.kind == BorrowKind::Shared {
                    self.errors.push(format!(
                        "Cannot mutate signal '{}' while borrowed as shared",
                        assign.target
                    ));
                }
            }
            
            self.borrows.insert(assign.target.clone(), Borrow {
                name: assign.target.clone(),
                kind: BorrowKind::Mutable,
                scope: self.current_scope,
            });
        }
        
        self.check_expr(&assign.value);
    }
    
    fn check_var(&mut self, var: &VarStmt) {
        self.check_expr(&var.value);
    }
    
    fn check_if(&mut self, if_stmt: &IfStmt) {
        self.check_expr(&if_stmt.cond);
        
        for stmt in &if_stmt.body {
            self.check_stmt(stmt);
        }
        
        for (cond, body) in &if_stmt.else_if {
            self.check_expr(cond);
            for stmt in body {
                self.check_stmt(stmt);
            }
        }
        
        if let Some(else_body) = &if_stmt.else_body {
            for stmt in else_body {
                self.check_stmt(stmt);
            }
        }
    }
    
    fn check_for(&mut self, for_stmt: &ForStmt) {
        self.check_expr(&for_stmt.iter);
        
        self.enter_scope();
        
        for stmt in &for_stmt.body {
            self.check_stmt(stmt);
        }
        
        self.exit_scope();
    }
    
    fn check_while(&mut self, while_stmt: &WhileStmt) {
        self.check_expr(&while_stmt.cond);
        
        for stmt in &while_stmt.body {
            self.check_stmt(stmt);
        }
    }
    
    fn check_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Ident(ident) => {
                if self.signals.contains(&ident.name) {
                    if let Some(borrow) = self.borrows.get(&ident.name) {
                        if borrow.kind == BorrowKind::Mutable {
                            self.errors.push(format!(
                                "Cannot read signal '{}' while mutably borrowed",
                                ident.name
                            ));
                        }
                    } else {
                        self.borrows.insert(ident.name.clone(), Borrow {
                            name: ident.name.clone(),
                            kind: BorrowKind::Shared,
                            scope: self.current_scope,
                        });
                    }
                }
            }
            Expr::Call(call) => {
                self.check_expr(&call.callee);
                for arg in &call.args {
                    self.check_expr(arg);
                }
            }
            Expr::Binary(binary) => {
                self.check_expr(&binary.left);
                self.check_expr(&binary.right);
            }
            Expr::Unary(unary) => {
                self.check_expr(&unary.expr);
            }
            Expr::Member(member) => {
                self.check_expr(&member.obj);
            }
            Expr::Object(object) => {
                for prop in &object.props {
                    self.check_expr(&prop.value);
                }
            }
            Expr::Array(array) => {
                for elem in &array.elements {
                    self.check_expr(elem);
                }
            }
            Expr::Lambda(lambda) => {
                self.check_expr(&lambda.body);
            }
            Expr::Ternary(cond, then_expr, else_expr) => {
                self.check_expr(cond);
                self.check_expr(then_expr);
                self.check_expr(else_expr);
            }
            _ => {}
        }
    }
    
    fn enter_scope(&mut self) {
        self.current_scope += 1;
    }
    
    fn exit_scope(&mut self) {
        let scope_to_remove = self.current_scope;
        self.borrows.retain(|_, b| b.scope != scope_to_remove);
        
        if self.current_scope > 0 {
            self.current_scope -= 1;
        }
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
}

pub fn check_borrows(program: &Program) -> Result<(), Vec<String>> {
    let mut checker = BorrowChecker::new();
    checker.check(program);
    
    if checker.has_errors() {
        Err(checker.get_errors())
    } else {
        Ok(())
    }
}
