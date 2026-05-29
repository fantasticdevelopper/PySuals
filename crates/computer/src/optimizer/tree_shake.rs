use std::collections::{HashMap, HashSet, VecDeque};
use crate::ast::*;

pub struct TreeShaker {
    reachable: HashSet<String>,
    dependency_graph: HashMap<String, Vec<String>>,
}

impl TreeShaker {
    pub fn new() -> Self {
        Self {
            reachable: HashSet::new(),
            dependency_graph: HashMap::new(),
        }
    }
    
    pub fn shake(&mut self, mut program: Program) -> Program {
        self.build_dependency_graph(&program);
        self.find_entry_points(&program);
        self.mark_reachable();
        
        program.components = self.filter_reachable_components(program.components);
        program.functions = self.filter_reachable_functions(program.functions);
        
        program
    }
    
    fn build_dependency_graph(&mut self, program: &Program) {
        for component in &program.components {
            let deps = self.find_component_deps(component);
            self.dependency_graph.insert(component.name.clone(), deps);
        }
        
        for function in &program.functions {
            let deps = self.find_function_deps(function);
            self.dependency_graph.insert(function.name.clone(), deps);
        }
    }
    
    fn find_component_deps(&self, component: &Component) -> Vec<String> {
        let mut deps = Vec::new();
        
        for stmt in &component.body {
            self.collect_deps_from_stmt(stmt, &mut deps);
        }
        
        deps
    }
    
    fn find_function_deps(&self, function: &Function) -> Vec<String> {
        let mut deps = Vec::new();
        
        for stmt in &function.body {
            self.collect_deps_from_stmt(stmt, &mut deps);
        }
        
        deps
    }
    
    fn collect_deps_from_stmt(&self, stmt: &Stmt, deps: &mut Vec<String>) {
        match stmt {
            Stmt::Expr(expr) => self.collect_deps_from_expr(expr, deps),
            Stmt::Return(expr) => self.collect_deps_from_expr(expr, deps),
            Stmt::Assign(assign) => self.collect_deps_from_expr(&assign.value, deps),
            Stmt::Var(var) => self.collect_deps_from_expr(&var.value, deps),
            Stmt::If(if_stmt) => {
                self.collect_deps_from_expr(&if_stmt.cond, deps);
                for s in &if_stmt.body {
                    self.collect_deps_from_stmt(s, deps);
                }
                for (_, b) in &if_stmt.else_if {
                    for s in b {
                        self.collect_deps_from_stmt(s, deps);
                    }
                }
                if let Some(b) = &if_stmt.else_body {
                    for s in b {
                        self.collect_deps_from_stmt(s, deps);
                    }
                }
            }
            Stmt::For(for_stmt) => {
                self.collect_deps_from_expr(&for_stmt.iter, deps);
                for s in &for_stmt.body {
                    self.collect_deps_from_stmt(s, deps);
                }
            }
            Stmt::While(while_stmt) => {
                self.collect_deps_from_expr(&while_stmt.cond, deps);
                for s in &while_stmt.body {
                    self.collect_deps_from_stmt(s, deps);
                }
            }
            Stmt::Block(block) => {
                for s in block {
                    self.collect_deps_from_stmt(s, deps);
                }
            }
            _ => {}
        }
    }
    
    fn collect_deps_from_expr(&self, expr: &Expr, deps: &mut Vec<String>) {
        match expr {
            Expr::Call(call) => {
                if let Expr::Ident(ident) = &*call.callee {
                    deps.push(ident.name.clone());
                }
                self.collect_deps_from_expr(&call.callee, deps);
                for arg in &call.args {
                    self.collect_deps_from_expr(arg, deps);
                }
            }
            Expr::Ident(ident) => {
                if ident.name.chars().next().map_or(false, |c| c.is_uppercase()) {
                    deps.push(ident.name.clone());
                }
            }
            Expr::Binary(binary) => {
                self.collect_deps_from_expr(&binary.left, deps);
                self.collect_deps_from_expr(&binary.right, deps);
            }
            Expr::Unary(unary) => {
                self.collect_deps_from_expr(&unary.expr, deps);
            }
            Expr::Member(member) => {
                self.collect_deps_from_expr(&member.obj, deps);
            }
            Expr::Object(object) => {
                for prop in &object.props {
                    self.collect_deps_from_expr(&prop.value, deps);
                }
            }
            Expr::Array(array) => {
                for elem in &array.elements {
                    self.collect_deps_from_expr(elem, deps);
                }
            }
            Expr::Lambda(lambda) => {
                self.collect_deps_from_expr(&lambda.body, deps);
            }
            Expr::Ternary(cond, then_expr, else_expr) => {
                self.collect_deps_from_expr(cond, deps);
                self.collect_deps_from_expr(then_expr, deps);
                self.collect_deps_from_expr(else_expr, deps);
            }
            _ => {}
        }
    }
    
    fn find_entry_points(&mut self, program: &Program) {
        for export in &program.exports {
            self.reachable.insert(export.name.clone());
        }
        
        if self.reachable.is_empty() && !program.components.is_empty() {
            if let Some(first) = program.components.first() {
                self.reachable.insert(first.name.clone());
            }
        }
    }
    
    fn mark_reachable(&mut self) {
        let mut queue: VecDeque<String> = self.reachable.iter().cloned().collect();
        
        while let Some(current) = queue.pop_front() {
            if let Some(deps) = self.dependency_graph.get(&current) {
                for dep in deps {
                    if !self.reachable.contains(dep) {
                        self.reachable.insert(dep.clone());
                        queue.push_back(dep.clone());
                    }
                }
            }
        }
    }
    
    fn filter_reachable_components(&self, components: Vec<Component>) -> Vec<Component> {
        components
            .into_iter()
            .filter(|c| self.reachable.contains(&c.name))
            .collect()
    }
    
    fn filter_reachable_functions(&self, functions: Vec<Function>) -> Vec<Function> {
        functions
            .into_iter()
            .filter(|f| self.reachable.contains(&f.name))
            .collect()
    }
    
    pub fn get_removed_count(&self, program: &Program) -> usize {
        let mut count = 0;
        
        for component in &program.components {
            if !self.reachable.contains(&component.name) {
                count += 1;
            }
        }
        
        for function in &program.functions {
            if !self.reachable.contains(&function.name) {
                count += 1;
            }
        }
        
        count
    }
    
    pub fn get_removed_names(&self) -> Vec<String> {
        let mut removed = Vec::new();
        
        for name in self.dependency_graph.keys() {
            if !self.reachable.contains(name) {
                removed.push(name.clone());
            }
        }
        
        removed
    }
}
