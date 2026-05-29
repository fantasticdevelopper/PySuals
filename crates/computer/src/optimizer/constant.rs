use crate::ast::*;

pub struct ConstantFolder {
    changed: bool,
}

impl ConstantFolder {
    pub fn new() -> Self {
        Self {
            changed: false,
        }
    }
    
    pub fn fold(&mut self, mut program: Program) -> Program {
        for component in &mut program.components {
            component.body = self.fold_stmts(&component.body);
        }
        
        for function in &mut program.functions {
            function.body = self.fold_stmts(&function.body);
        }
        
        program
    }
    
    fn fold_stmts(&mut self, stmts: &[Stmt]) -> Vec<Stmt> {
        let mut result = Vec::new();
        
        for stmt in stmts {
            result.push(self.fold_stmt(stmt));
        }
        
        result
    }
    
    fn fold_stmt(&mut self, stmt: &Stmt) -> Stmt {
        match stmt {
            Stmt::Expr(expr) => Stmt::Expr(self.fold_expr(expr)),
            Stmt::Return(expr) => Stmt::Return(self.fold_expr(expr)),
            Stmt::Assign(assign) => Stmt::Assign(AssignStmt {
                target: assign.target.clone(),
                value: self.fold_expr(&assign.value),
                op: assign.op.clone(),
            }),
            Stmt::Var(var) => Stmt::Var(VarStmt {
                name: var.name.clone(),
                value: self.fold_expr(&var.value),
                is_const: var.is_const,
            }),
            Stmt::If(if_stmt) => {
                let cond = self.fold_expr(&if_stmt.cond);
                
                if self.is_constant_false(&cond) {
                    if let Some(else_body) = &if_stmt.else_body {
                        return Stmt::Block(else_body.clone());
                    } else {
                        return Stmt::Block(Vec::new());
                    }
                }
                
                if self.is_constant_true(&cond) {
                    return Stmt::Block(if_stmt.body.clone());
                }
                
                Stmt::If(IfStmt {
                    cond,
                    body: self.fold_stmts(&if_stmt.body),
                    else_if: if_stmt.else_if.iter().map(|(c, b)| {
                        (self.fold_expr(c), self.fold_stmts(b))
                    }).collect(),
                    else_body: if_stmt.else_body.as_ref().map(|b| self.fold_stmts(b)),
                })
            }
            Stmt::Block(block) => Stmt::Block(self.fold_stmts(block)),
            _ => stmt.clone(),
        }
    }
    
    fn fold_expr(&mut self, expr: &Expr) -> Expr {
        match expr {
            Expr::Binary(binary) => {
                let left = self.fold_expr(&binary.left);
                let right = self.fold_expr(&binary.right);
                
                if let (Expr::Literal(lit1), Expr::Literal(lit2)) = (&left, &right) {
                    return self.eval_binary(&lit1.value, &lit2.value, &binary.op);
                }
                
                Expr::Binary(BinaryExpr {
                    left: Box::new(left),
                    right: Box::new(right),
                    op: binary.op.clone(),
                })
            }
            Expr::Unary(unary) => {
                let expr = self.fold_expr(&unary.expr);
                
                if let Expr::Literal(lit) = &expr {
                    return self.eval_unary(&lit.value, &unary.op);
                }
                
                Expr::Unary(UnaryExpr {
                    expr: Box::new(expr),
                    op: unary.op.clone(),
                })
            }
            Expr::Ternary(cond, then_expr, else_expr) => {
                let cond = self.fold_expr(cond);
                
                if self.is_constant_true(&cond) {
                    return self.fold_expr(then_expr);
                }
                
                if self.is_constant_false(&cond) {
                    return self.fold_expr(else_expr);
                }
                
                Expr::Ternary(
                    Box::new(cond),
                    Box::new(self.fold_expr(then_expr)),
                    Box::new(self.fold_expr(else_expr)),
                )
            }
            Expr::Call(call) => {
                let callee = self.fold_expr(&call.callee);
                let args: Vec<Expr> = call.args.iter().map(|a| self.fold_expr(a)).collect();
                
                Expr::Call(CallExpr {
                    callee: Box::new(callee),
                    args,
                })
            }
            _ => expr.clone(),
        }
    }
    
    fn eval_binary(&self, left: &LiteralValue, right: &LiteralValue, op: &BinaryOp) -> Expr {
        use LiteralValue::*;
        
        match (left, right, op) {
            (Number(a), Number(b), BinaryOp::Add) => {
                Expr::Literal(LiteralExpr::number(*a + *b))
            }
            (Number(a), Number(b), BinaryOp::Sub) => {
                Expr::Literal(LiteralExpr::number(*a - *b))
            }
            (Number(a), Number(b), BinaryOp::Mul) => {
                Expr::Literal(LiteralExpr::number(*a * *b))
            }
            (Number(a), Number(b), BinaryOp::Div) => {
                Expr::Literal(LiteralExpr::number(*a / *b))
            }
            (String(a), String(b), BinaryOp::Add) => {
                Expr::Literal(LiteralExpr::string(format!("{}{}", a, b)))
            }
            (Boolean(a), Boolean(b), BinaryOp::Eq) => {
                Expr::Literal(LiteralExpr::boolean(a == b))
            }
            (Boolean(a), Boolean(b), BinaryOp::And) => {
                Expr::Literal(LiteralExpr::boolean(*a && *b))
            }
            (Boolean(a), Boolean(b), BinaryOp::Or) => {
                Expr::Literal(LiteralExpr::boolean(*a || *b))
            }
            _ => Expr::Literal(LiteralExpr::null()),
        }
    }
    
    fn eval_unary(&self, value: &LiteralValue, op: &UnaryOp) -> Expr {
        use LiteralValue::*;
        
        match (value, op) {
            (Number(n), UnaryOp::Neg) => Expr::Literal(LiteralExpr::number(-*n)),
            (Boolean(b), UnaryOp::Not) => Expr::Literal(LiteralExpr::boolean(!*b)),
            _ => Expr::Literal(LiteralExpr::null()),
        }
    }
    
    fn is_constant_true(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Literal(lit) => matches!(&lit.value, LiteralValue::Boolean(true)),
            _ => false,
        }
    }
    
    fn is_constant_false(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Literal(lit) => matches!(&lit.value, LiteralValue::Boolean(false)),
            _ => false,
        }
    }
    
    pub fn has_changed(&self) -> bool {
        self.changed
    }
}

impl LiteralExpr {
    pub fn number(n: f64) -> Self {
        Self {
            value: LiteralValue::Number(n),
            raw: n.to_string(),
        }
    }
    
    pub fn string(s: String) -> Self {
        Self {
            value: LiteralValue::String(s.clone()),
            raw: format!("\"{}\"", s),
        }
    }
    
    pub fn boolean(b: bool) -> Self {
        Self {
            value: LiteralValue::Boolean(b),
            raw: b.to_string(),
        }
    }
    
    pub fn null() -> Self {
        Self {
            value: LiteralValue::Null,
            raw: "null".to_string(),
        }
    }
             }
