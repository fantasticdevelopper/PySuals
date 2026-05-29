use std::collections::{HashMap, HashSet};
use crate::ast::*;

pub struct DeadCodeEliminator {
    used_functions: HashSet<String>,
    used_components: HashSet<String>,
    used_signals: HashSet<String>,
}

impl DeadCodeEliminator {
    pub fn new() -> Self {
        Self {
            used_functions: HashSet::new(),
            used_components: HashSet::new(),
            used_signals: HashSet::new(),
        }
    }
    
    pub fn eliminate(&mut self, program: Program) -> Program {
        self.mark_used(&program);
        
        Program {
            components: self.filter_components(program.components),
            functions: self.filter_functions(program.functions),
            imports: program.imports,
            exports: program.exports,
            css: program.css,
        }
    }
    
    fn mark_used(&mut self, program: &Program) {
        for export in &program.exports {
            self.mark_export(export);
        }
        
        for component in &program.components {
            self.mark_component_usage(component);
        }
        
        for function in &program.functions {
            self.mark_function_usage(function);
        }
    }
    
    fn mark_export(&mut self, export: &Export) {
        if export.is_default {
            self.used_components.insert(export.name.clone());
        }
    }
    
    fn mark_component_usage(&mut self, component: &Component) {
        if self.used_components.contains(&component.name) {
            for stmt in &component.body {
                self.mark_stmt(stmt);
            }
        }
    }
    
    fn mark_function_usage(&mut self, function: &Function) {
        if self.used_functions.contains(&function.name) {
            for stmt in &function.body {
                self.mark_stmt(stmt);
            }
        }
    }
    
    fn mark_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Expr(expr) => self.mark_expr(expr),
            Stmt::Return(expr) => self.mark_expr(expr),
            Stmt::Assign(assign) => self.mark_expr(&assign.value),
            Stmt::Var(var) => self.mark_expr(&var.value),
            Stmt::If(if_stmt) => {
                self.mark_expr(&if_stmt.cond);
                for s in &if_stmt.body {
                    self.mark_stmt(s);
                }
                for (c, b) in &if_stmt.else_if {
                    self.mark_expr(c);
                    for s in b {
                        self.mark_stmt(s);
                    }
                }
                if let Some(b) = &if_stmt.else_body {
                    for s in b {
                        self.mark_stmt(s);
                    }
                }
            }
            Stmt::For(for_stmt) => {
                self.mark_expr(&for_stmt.iter);
                for s in &for_stmt.body {
                    self.mark_stmt(s);
                }
            }
            Stmt::While(while_stmt) => {
                self.mark_expr(&while_stmt.cond);
                for s in &while_stmt.body {
                    self.mark_stmt(s);
                }
            }
            Stmt::Block(block) => {
                for s in block {
                    self.mark_stmt(s);
                }
            }
            _ => {}
        }
    }
    
    fn mark_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Call(call) => {
                self.mark_expr(&call.callee);
                for arg in &call.args {
                    self.mark_expr(arg);
                }
            }
            Expr::Ident(ident) => {
                if ident.name == "signal" {
                    self.used_signals.insert(ident.name.clone());
                }
            }
            Expr::Binary(binary) => {
                self.mark_expr(&binary.left);
                self.mark_expr(&binary.right);
            }
            Expr::Unary(unary) => {
                self.mark_expr(&unary.expr);
            }
            Expr::Member(member) => {
                self.mark_expr(&member.obj);
            }
            Expr::Object(object) => {
                for prop in &object.props {
                    self.mark_expr(&prop.value);
                }
            }
            Expr::Array(array) => {
                for elem in &array.elements {
                    self.mark_expr(elem);
                }
            }
            Expr::Lambda(lambda) => {
                self.mark_expr(&lambda.body);
            }
            Expr::Ternary(cond, then_expr, else_expr) => {
                self.mark_expr(cond);
                self.mark_expr(then_expr);
                self.mark_expr(else_expr);
            }
            _ => {}
        }
    }
    
    fn filter_components(&self, components: Vec<Component>) -> Vec<Component> {
        components
            .into_iter()
            .filter(|c| self.used_components.contains(&c.name))
            .collect()
    }
    
    fn filter_functions(&self, functions: Vec<Function>) -> Vec<Function> {
        functions
            .into_iter()
            .filter(|f| self.used_functions.contains(&f.name))
            .collect()
    }
    
    pub fn remove_unused_signal(&mut self, name: &str) {
        if !self.used_signals.contains(name) {
            // Signal can be removed
        }
    }
    
    pub fn get_unused_count(&self, program: &Program) -> usize {
        let mut count = 0;
        
        for component in &program.components {
            if !self.used_components.contains(&component.name) {
                count += 1;
            }
        }
        
        for function in &program.functions {
            if !self.used_functions.contains(&function.name) {
                count += 1;
            }
        }
        
        count
    }
}
