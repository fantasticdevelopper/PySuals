use super::{
    Program, Component, Function, Signal, Effect, Computed,
    Import, Export, Stmt, Expr, Param, IfStmt, ForStmt,
    WhileStmt, AssignStmt, VarStmt, CallExpr, BinaryExpr,
    UnaryExpr, MemberExpr, ObjectExpr, ArrayExpr, LambdaExpr,
    Literal, CssBlock,
};

pub trait Visitor: Sized {
    fn visit_program(&mut self, program: &Program) {
        self.visit_program_children(program);
    }
    
    fn visit_program_children(&mut self, program: &Program) {
        for component in &program.components {
            self.visit_component(component);
        }
        for function in &program.functions {
            self.visit_function(function);
        }
        for import in &program.imports {
            self.visit_import(import);
        }
        for export in &program.exports {
            self.visit_export(export);
        }
        for css in &program.css {
            self.visit_css(css);
        }
    }
    
    fn visit_component(&mut self, component: &Component) {
        for param in &component.params {
            self.visit_param(param);
        }
        for stmt in &component.body {
            self.visit_stmt(stmt);
        }
        for signal in &component.signals {
            self.visit_signal(signal);
        }
        for computed in &component.computed {
            self.visit_computed(computed);
        }
        for effect in &component.effects {
            self.visit_effect(effect);
        }
    }
    
    fn visit_function(&mut self, function: &Function) {
        for param in &function.params {
            self.visit_param(param);
        }
        for stmt in &function.body {
            self.visit_stmt(stmt);
        }
    }
    
    fn visit_signal(&mut self, signal: &Signal) {
        self.visit_expr(&signal.initial);
    }
    
    fn visit_effect(&mut self, effect: &Effect) {
        for stmt in &effect.body {
            self.visit_stmt(stmt);
        }
    }
    
    fn visit_computed(&mut self, computed: &Computed) {
        self.visit_expr(&computed.body);
    }
    
    fn visit_import(&mut self, _import: &Import) {}
    
    fn visit_export(&mut self, _export: &Export) {}
    
    fn visit_css(&mut self, _css: &CssBlock) {}
    
    fn visit_param(&mut self, param: &Param) {
        if let Some(default) = &param.default {
            self.visit_expr(default);
        }
    }
    
    fn visit_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Expr(expr) => self.visit_expr(expr),
            Stmt::Return(expr) => self.visit_expr(expr),
            Stmt::If(if_stmt) => self.visit_if_stmt(if_stmt),
            Stmt::For(for_stmt) => self.visit_for_stmt(for_stmt),
            Stmt::While(while_stmt) => self.visit_while_stmt(while_stmt),
            Stmt::Assign(assign) => self.visit_assign_stmt(assign),
            Stmt::Var(var) => self.visit_var_stmt(var),
            Stmt::Block(stmts) => {
                for stmt in stmts {
                    self.visit_stmt(stmt);
                }
            }
        }
    }
    
    fn visit_if_stmt(&mut self, if_stmt: &IfStmt) {
        self.visit_expr(&if_stmt.cond);
        for stmt in &if_stmt.body {
            self.visit_stmt(stmt);
        }
        for (cond, body) in &if_stmt.else_if {
            self.visit_expr(cond);
            for stmt in body {
                self.visit_stmt(stmt);
            }
        }
        if let Some(else_body) = &if_stmt.else_body {
            for stmt in else_body {
                self.visit_stmt(stmt);
            }
        }
    }
    
    fn visit_for_stmt(&mut self, for_stmt: &ForStmt) {
        self.visit_expr(&for_stmt.iter);
        for stmt in &for_stmt.body {
            self.visit_stmt(stmt);
        }
    }
    
    fn visit_while_stmt(&mut self, while_stmt: &WhileStmt) {
        self.visit_expr(&while_stmt.cond);
        for stmt in &while_stmt.body {
            self.visit_stmt(stmt);
        }
    }
    
    fn visit_assign_stmt(&mut self, assign: &AssignStmt) {
        self.visit_expr(&assign.value);
    }
    
    fn visit_var_stmt(&mut self, var: &VarStmt) {
        self.visit_expr(&var.value);
    }
    
    fn visit_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Literal(_) => {}
            Expr::Ident(_) => {}
            Expr::Call(call) => self.visit_call_expr(call),
            Expr::Binary(binary) => self.visit_binary_expr(binary),
            Expr::Unary(unary) => self.visit_unary_expr(unary),
            Expr::Member(member) => self.visit_member_expr(member),
            Expr::Object(object) => self.visit_object_expr(object),
            Expr::Array(array) => self.visit_array_expr(array),
            Expr::Lambda(lambda) => self.visit_lambda_expr(lambda),
            Expr::Ternary(cond, then_expr, else_expr) => {
                self.visit_expr(cond);
                self.visit_expr(then_expr);
                self.visit_expr(else_expr);
            }
        }
    }
    
    fn visit_call_expr(&mut self, call: &CallExpr) {
        self.visit_expr(&call.callee);
        for arg in &call.args {
            self.visit_expr(arg);
        }
    }
    
    fn visit_binary_expr(&mut self, binary: &BinaryExpr) {
        self.visit_expr(&binary.left);
        self.visit_expr(&binary.right);
    }
    
    fn visit_unary_expr(&mut self, unary: &UnaryExpr) {
        self.visit_expr(&unary.expr);
    }
    
    fn visit_member_expr(&mut self, member: &MemberExpr) {
        self.visit_expr(&member.obj);
    }
    
    fn visit_object_expr(&mut self, object: &ObjectExpr) {
        for prop in &object.props {
            self.visit_expr(&prop.value);
        }
    }
    
    fn visit_array_expr(&mut self, array: &ArrayExpr) {
        for elem in &array.elements {
            self.visit_expr(elem);
        }
    }
    
    fn visit_lambda_expr(&mut self, lambda: &LambdaExpr) {
        self.visit_expr(&lambda.body);
    }
}

pub struct CountingVisitor {
    pub component_count: usize,
    pub signal_count: usize,
    pub effect_count: usize,
    pub function_count: usize,
}

impl CountingVisitor {
    pub fn new() -> Self {
        Self {
            component_count: 0,
            signal_count: 0,
            effect_count: 0,
            function_count: 0,
        }
    }
}

impl Visitor for CountingVisitor {
    fn visit_component(&mut self, _component: &Component) {
        self.component_count += 1;
    }
    
    fn visit_signal(&mut self, _signal: &Signal) {
        self.signal_count += 1;
    }
    
    fn visit_effect(&mut self, _effect: &Effect) {
        self.effect_count += 1;
    }
    
    fn visit_function(&mut self, _function: &Function) {
        self.function_count += 1;
    }
}

pub struct CollectingVisitor {
    pub signals: Vec<String>,
    pub components: Vec<String>,
    pub imports: Vec<String>,
}

impl CollectingVisitor {
    pub fn new() -> Self {
        Self {
            signals: Vec::new(),
            components: Vec::new(),
            imports: Vec::new(),
        }
    }
}

impl Visitor for CollectingVisitor {
    fn visit_signal(&mut self, signal: &Signal) {
        self.signals.push(signal.name.clone());
    }
    
    fn visit_component(&mut self, component: &Component) {
        self.components.push(component.name.clone());
    }
    
    fn visit_import(&mut self, import: &Import) {
        self.imports.push(import.path.clone());
    }
}

pub fn visit_program<V: Visitor>(visitor: &mut V, program: &Program) {
    visitor.visit_program(program);
}
