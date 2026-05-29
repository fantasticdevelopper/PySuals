use std::collections::HashMap;
use crate::ast::*;

pub struct Inliner {
    inline_threshold: usize,
    inlined_functions: HashMap<String, Function>,
}

impl Inliner {
    pub fn new() -> Self {
        Self {
            inline_threshold: 10,
            inlined_functions: HashMap::new(),
        }
    }
    
    pub fn inline(&mut self, mut program: Program) -> Program {
        self.collect_small_functions(&program);
        
        for component in &mut program.components {
            component.body = self.inline_stmts(&component.body);
        }
        
        for function in &mut program.functions {
            function.body = self.inline_stmts(&function.body);
        }
        
        program.functions = self.filter_inlined_functions(program.functions);
        
        program
    }
    
    fn collect_small_functions(&mut self, program: &Program) {
        for function in &program.functions {
            if self.is_small_function(function) {
                self.inlined_functions.insert(function.name.clone(), function.clone());
            }
        }
    }
    
    fn is_small_function(&self, function: &Function) -> bool {
        let mut size = 0;
        
        for stmt in &function.body {
            size += self.estimate_stmt_size(stmt);
        }
        
        size < self.inline_threshold
    }
    
    fn estimate_stmt_size(&self, stmt: &Stmt) -> usize {
        match stmt {
            Stmt::Expr(expr) => self.estimate_expr_size(expr),
            Stmt::Return(expr) => 1 + self.estimate_expr_size(expr),
            Stmt::Assign(assign) => 1 + self.estimate_expr_size(&assign.value),
            Stmt::Var(var) => 1 + self.estimate_expr_size(&var.value),
            Stmt::Block(block) => {
                block.iter().map(|s| self.estimate_stmt_size(s)).sum()
            }
            _ => 1,
        }
    }
    
    fn estimate_expr_size(&self, expr: &Expr) -> usize {
        match expr {
            Expr::Literal(_) => 1,
            Expr::Ident(_) => 1,
            Expr::Call(call) => {
                1 + self.estimate_expr_size(&call.callee) 
                + call.args.iter().map(|a| self.estimate_expr_size(a)).sum::<usize>()
            }
            Expr::Binary(binary) => {
                1 + self.estimate_expr_size(&binary.left) + self.estimate_expr_size(&binary.right)
            }
            Expr::Unary(unary) => 1 + self.estimate_expr_size(&unary.expr),
            Expr::Member(member) => 1 + self.estimate_expr_size(&member.obj),
            Expr::Object(object) => {
                1 + object.props.iter().map(|p| self.estimate_expr_size(&p.value)).sum::<usize>()
            }
            Expr::Array(array) => {
                1 + array.elements.iter().map(|e| self.estimate_expr_size(e)).sum::<usize>()
            }
            Expr::Lambda(lambda) => 1 + self.estimate_expr_size(&lambda.body),
            Expr::Ternary(cond, then_expr, else_expr) => {
                1 + self.estimate_expr_size(cond) 
                + self.estimate_expr_size(then_expr) 
                + self.estimate_expr_size(else_expr)
            }
            _ => 1,
        }
    }
    
    fn inline_stmts(&mut self, stmts: &[Stmt]) -> Vec<Stmt> {
        let mut result = Vec::new();
        
        for stmt in stmts {
            result.push(self.inline_stmt(stmt));
        }
        
        result
    }
    
    fn inline_stmt(&mut self, stmt: &Stmt) -> Stmt {
        match stmt {
            Stmt::Expr(expr) => Stmt::Expr(self.inline_expr(expr)),
            Stmt::Return(expr) => Stmt::Return(self.inline_expr(expr)),
            Stmt::Assign(assign) => Stmt::Assign(AssignStmt {
                target: assign.target.clone(),
                value: self.inline_expr(&assign.value),
                op: assign.op.clone(),
            }),
            Stmt::Var(var) => Stmt::Var(VarStmt {
                name: var.name.clone(),
                value: self.inline_expr(&var.value),
                is_const: var.is_const,
            }),
            Stmt::Block(block) => Stmt::Block(self.inline_stmts(block)),
            Stmt::If(if_stmt) => Stmt::If(IfStmt {
                cond: self.inline_expr(&if_stmt.cond),
                body: self.inline_stmts(&if_stmt.body),
                else_if: if_stmt.else_if.iter().map(|(c, b)| {
                    (self.inline_expr(c), self.inline_stmts(b))
                }).collect(),
                else_body: if_stmt.else_body.as_ref().map(|b| self.inline_stmts(b)),
            }),
            _ => stmt.clone(),
        }
    }
    
    fn inline_expr(&mut self, expr: &Expr) -> Expr {
        match expr {
            Expr::Call(call) => {
                if let Expr::Ident(ident) = &*call.callee {
                    if let Some(function) = self.inlined_functions.get(&ident.name) {
                        return self.inline_function_call(function, &call.args);
                    }
                }
                
                Expr::Call(CallExpr {
                    callee: Box::new(self.inline_expr(&call.callee)),
                    args: call.args.iter().map(|a| self.inline_expr(a)).collect(),
                })
            }
            Expr::Binary(binary) => Expr::Binary(BinaryExpr {
                left: Box::new(self.inline_expr(&binary.left)),
                right: Box::new(self.inline_expr(&binary.right)),
                op: binary.op.clone(),
            }),
            Expr::Unary(unary) => Expr::Unary(UnaryExpr {
                expr: Box::new(self.inline_expr(&unary.expr)),
                op: unary.op.clone(),
            }),
            Expr::Member(member) => Expr::Member(MemberExpr {
                obj: Box::new(self.inline_expr(&member.obj)),
                prop: member.prop.clone(),
                computed: member.computed,
            }),
            _ => expr.clone(),
        }
    }
    
    fn inline_function_call(&mut self, function: &Function, args: &[Expr]) -> Expr {
        let mut body_expr = None;
        
        for stmt in &function.body {
            if let Stmt::Return(expr) = stmt {
                body_expr = Some(self.inline_expr(expr));
                break;
            }
        }
        
        body_expr.unwrap_or(Expr::Literal(LiteralExpr::null()))
    }
    
    fn filter_inlined_functions(&self, functions: Vec<Function>) -> Vec<Function> {
        functions
            .into_iter()
            .filter(|f| !self.inlined_functions.contains_key(&f.name))
            .collect()
    }
              }
