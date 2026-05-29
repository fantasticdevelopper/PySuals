use super::token::TokenType;
use crate::ast::*;

pub struct GrammarChecker {
    errors: Vec<String>,
}

impl GrammarChecker {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
        }
    }
    
    pub fn check(&mut self, program: &Program) -> Result<(), Vec<String>> {
        for component in &program.components {
            self.check_component(component);
        }
        
        for function in &program.functions {
            self.check_function(function);
        }
        
        if !self.errors.is_empty() {
            Err(self.errors.clone())
        } else {
            Ok(())
        }
    }
    
    fn check_component(&mut self, component: &Component) {
        if component.name.is_empty() {
            self.errors.push("Component name cannot be empty".to_string());
        }
        
        let first_char = component.name.chars().next().unwrap();
        if !first_char.is_uppercase() {
            self.errors.push(format!(
                "Component name '{}' must start with uppercase letter",
                component.name
            ));
        }
        
        for param in &component.params {
            self.check_param(param);
        }
        
        for stmt in &component.body {
            self.check_stmt(stmt);
        }
    }
    
    fn check_function(&mut self, function: &Function) {
        if function.name.is_empty() {
            self.errors.push("Function name cannot be empty".to_string());
        }
        
        for param in &function.params {
            self.check_param(param);
        }
        
        let has_return = self.check_returns(&function.body);
        if !has_return && function.return_type.is_some() {
            self.errors.push(format!(
                "Function '{}' declared with return type but has no return statement",
                function.name
            ));
        }
    }
    
    fn check_param(&mut self, param: &Param) {
        if param.name.is_empty() {
            self.errors.push("Parameter name cannot be empty".to_string());
        }
        
        if let Some(type_hint) = &param.type_hint {
            if !self.is_valid_type(type_hint) {
                self.errors.push(format!("Invalid type hint: {}", type_hint));
            }
        }
    }
    
    fn check_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Expr(expr) => self.check_expr(expr),
            Stmt::Return(expr) => self.check_expr(expr),
            Stmt::If(if_stmt) => self.check_if_stmt(if_stmt),
            Stmt::For(for_stmt) => self.check_for_stmt(for_stmt),
            Stmt::While(while_stmt) => self.check_while_stmt(while_stmt),
            Stmt::Assign(assign) => self.check_assign_stmt(assign),
            Stmt::Var(var) => self.check_var_stmt(var),
            Stmt::Block(stmts) => {
                for stmt in stmts {
                    self.check_stmt(stmt);
                }
            }
        }
    }
    
    fn check_if_stmt(&mut self, if_stmt: &IfStmt) {
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
    
    fn check_for_stmt(&mut self, for_stmt: &ForStmt) {
        self.check_expr(&for_stmt.iter);
        
        for stmt in &for_stmt.body {
            self.check_stmt(stmt);
        }
    }
    
    fn check_while_stmt(&mut self, while_stmt: &WhileStmt) {
        self.check_expr(&while_stmt.cond);
        
        for stmt in &while_stmt.body {
            self.check_stmt(stmt);
        }
    }
    
    fn check_assign_stmt(&mut self, assign: &AssignStmt) {
        self.check_expr(&assign.value);
    }
    
    fn check_var_stmt(&mut self, var: &VarStmt) {
        self.check_expr(&var.value);
    }
    
    fn check_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Literal(_) => {}
            Expr::Ident(ident) => {
                if ident.name.is_empty() {
                    self.errors.push("Identifier name cannot be empty".to_string());
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
            Expr::Template(template) => {
                for expr in &template.expressions {
                    self.check_expr(expr);
                }
            }
            Expr::Await(await_expr) => {
                self.check_expr(&await_expr.expr);
            }
            Expr::Spread(spread) => {
                self.check_expr(&spread.expr);
            }
        }
    }
    
    fn check_returns(&mut self, stmts: &[Stmt]) -> bool {
        for stmt in stmts {
            match stmt {
                Stmt::Return(_) => return true,
                Stmt::Block(block) => {
                    if self.check_returns(block) {
                        return true;
                    }
                }
                Stmt::If(if_stmt) => {
                    let then_returns = self.check_returns(&if_stmt.body);
                    let else_returns = if let Some(else_body) = &if_stmt.else_body {
                        self.check_returns(else_body)
                    } else {
                        false
                    };
                    
                    if then_returns && else_returns {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }
    
    fn is_valid_type(&self, type_name: &str) -> bool {
        matches!(
            type_name,
            "str" | "int" | "float" | "bool" | "list" | "dict" | "None" | "Any"
        )
    }
}

pub struct IndentationChecker {
    indent_level: usize,
    errors: Vec<String>,
}

impl IndentationChecker {
    pub fn new() -> Self {
        Self {
            indent_level: 0,
            errors: Vec::new(),
        }
    }
    
    pub fn check(&mut self, source: &str) -> Result<(), Vec<String>> {
        let mut lines: Vec<&str> = source.lines().collect();
        
        for (i, line) in lines.iter().enumerate() {
            if line.is_empty() {
                continue;
            }
            
            let indent = line.chars().take_while(|c| *c == ' ').count();
            
            if indent % 4 != 0 {
                self.errors.push(format!(
                    "Line {}: Indentation must be multiple of 4 spaces, found {} spaces",
                    i + 1, indent
                ));
            }
            
            let expected = self.indent_level * 4;
            if indent > expected {
                if indent == expected + 4 {
                    self.indent_level += 1;
                } else {
                    self.errors.push(format!(
                        "Line {}: Unexpected indentation level {} (expected {})",
                        i + 1, indent, expected + 4
                    ));
                }
            } else if indent < expected {
                if indent % 4 == 0 {
                    self.indent_level = indent / 4;
                } else {
                    self.errors.push(format!(
                        "Line {}: Invalid dedent",
                        i + 1
                    ));
                }
            }
        }
        
        if !self.errors.is_empty() {
            Err(self.errors.clone())
        } else {
            Ok(())
        }
    }
}

pub fn validate_grammar(program: &Program) -> Result<(), Vec<String>> {
    let mut checker = GrammarChecker::new();
    checker.check(program)
}

pub fn check_indentation(source: &str) -> Result<(), Vec<String>> {
    let mut checker = IndentationChecker::new();
    checker.check(source)
                        }
