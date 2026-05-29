use crate::ast::*;
use super::CompilerConfig;

pub struct JSGenerator {
    output: String,
    indent: usize,
    config: CompilerConfig,
}

impl JSGenerator {
    pub fn new(config: &CompilerConfig) -> Self {
        Self {
            output: String::new(),
            indent: 0,
            config: config.clone(),
        }
    }
    
    pub fn generate(&mut self, program: &Program) -> String {
        self.output.clear();
        
        self.emit_line("import { signal, effect, h, mount } from '@pysuals/runtime';");
        self.emit_line("");
        
        for component in &program.components {
            self.generate_component(component);
        }
        
        for function in &program.functions {
            self.generate_function(function);
        }
        
        self.emit_line("");
        self.emit_line("// Mount application");
        self.emit_line("const root = document.getElementById('app');");
        
        if let Some(first_component) = program.components.first() {
            self.emit_line(&format!("mount({}, root);", first_component.name));
        }
        
        self.output.clone()
    }
    
    fn generate_component(&mut self, component: &Component) {
        self.emit_line(&format!("export function {}() {{", component.name));
        self.indent += 1;
        
        for signal in &component.signals {
            self.emit_line(&format!("const [get{}, set{}] = signal({:?});", 
                signal.name, signal.name, signal.initial));
        }
        
        for computed in &component.computed {
            self.emit_line(&format!("const {} = computed(() => {{", computed.name));
            self.indent += 1;
            self.emit_line(&format!("return {};", self.expr_to_string(&computed.body)));
            self.indent -= 1;
            self.emit_line("});");
        }
        
        for effect in &component.effects {
            self.emit_line("effect(() => {");
            self.indent += 1;
            for stmt in &effect.body {
                self.generate_stmt(stmt);
            }
            self.indent -= 1;
            self.emit_line("});");
        }
        
        self.emit_line("return (");
        self.indent += 1;
        
        if let Some(stmt) = component.body.first() {
            if let Stmt::Return(expr) = stmt {
                self.generate_expr(expr);
            }
        }
        
        self.indent -= 1;
        self.emit_line(");");
        
        self.indent -= 1;
        self.emit_line("}");
        self.emit_line("");
    }
    
    fn generate_function(&mut self, function: &Function) {
        self.emit_line(&format!("export function {}(", function.name));
        
        let params: Vec<String> = function.params.iter()
            .map(|p| p.name.clone())
            .collect();
        
        self.emit_line(&format!("  {}", params.join(", ")));
        self.emit_line(") {");
        
        self.indent += 1;
        for stmt in &function.body {
            self.generate_stmt(stmt);
        }
        self.indent -= 1;
        
        self.emit_line("}");
        self.emit_line("");
    }
    
    fn generate_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Expr(expr) => {
                self.emit(&self.expr_to_string(expr));
                self.emit_line(";");
            }
            Stmt::Return(expr) => {
                self.emit(&format!("return {}", self.expr_to_string(expr)));
                self.emit_line(";");
            }
            Stmt::Assign(assign) => {
                self.emit(&format!("{} = {}", assign.target, self.expr_to_string(&assign.value)));
                self.emit_line(";");
            }
            Stmt::Var(var) => {
                let kind = if var.is_const { "const" } else { "let" };
                self.emit(&format!("{} {} = {}", kind, var.name, self.expr_to_string(&var.value)));
                self.emit_line(";");
            }
            Stmt::If(if_stmt) => {
                self.emit(&format!("if ({}) {{", self.expr_to_string(&if_stmt.cond)));
                self.emit_line("");
                
                self.indent += 1;
                for s in &if_stmt.body {
                    self.generate_stmt(s);
                }
                self.indent -= 1;
                
                self.emit_line("}");
                
                for (cond, body) in &if_stmt.else_if {
                    self.emit(&format!(" else if ({}) {{", self.expr_to_string(cond)));
                    self.emit_line("");
                    
                    self.indent += 1;
                    for s in body {
                        self.generate_stmt(s);
                    }
                    self.indent -= 1;
                    
                    self.emit_line("}");
                }
                
                if let Some(else_body) = &if_stmt.else_body {
                    self.emit_line(" else {");
                    self.indent += 1;
                    for s in else_body {
                        self.generate_stmt(s);
                    }
                    self.indent -= 1;
                    self.emit_line("}");
                }
            }
            Stmt::For(for_stmt) => {
                self.emit(&format!("for (const {} of {}) {{", 
                    for_stmt.item, self.expr_to_string(&for_stmt.iter)));
                self.emit_line("");
                
                self.indent += 1;
                for s in &for_stmt.body {
                    self.generate_stmt(s);
                }
                self.indent -= 1;
                
                self.emit_line("}");
            }
            Stmt::While(while_stmt) => {
                self.emit(&format!("while ({}) {{", self.expr_to_string(&while_stmt.cond)));
                self.emit_line("");
                
                self.indent += 1;
                for s in &while_stmt.body {
                    self.generate_stmt(s);
                }
                self.indent -= 1;
                
                self.emit_line("}");
            }
            Stmt::Block(block) => {
                self.emit_line("{");
                self.indent += 1;
                for s in block {
                    self.generate_stmt(s);
                }
                self.indent -= 1;
                self.emit_line("}");
            }
        }
    }
    
    fn generate_expr(&mut self, expr: &Expr) {
        self.emit(&self.expr_to_string(expr));
    }
    
    fn expr_to_string(&self, expr: &Expr) -> String {
        match expr {
            Expr::Literal(lit) => lit.raw.clone(),
            Expr::Ident(ident) => ident.name.clone(),
            Expr::Call(call) => {
                let callee = self.expr_to_string(&call.callee);
                let args: Vec<String> = call.args.iter()
                    .map(|a| self.expr_to_string(a))
                    .collect();
                format!("{}({})", callee, args.join(", "))
            }
            Expr::Binary(binary) => {
                let left = self.expr_to_string(&binary.left);
                let right = self.expr_to_string(&binary.right);
                let op = self.binary_op_to_string(&binary.op);
                format!("({} {} {})", left, op, right)
            }
            Expr::Unary(unary) => {
                let op = self.unary_op_to_string(&unary.op);
                let expr = self.expr_to_string(&unary.expr);
                format!("{}{}", op, expr)
            }
            Expr::Member(member) => {
                let obj = self.expr_to_string(&member.obj);
                if member.computed {
                    format!("{}[{}]", obj, member.prop)
                } else {
                    format!("{}.{}", obj, member.prop)
                }
            }
            Expr::Object(object) => {
                let props: Vec<String> = object.props.iter()
                    .map(|p| format!("{}: {}", p.key, self.expr_to_string(&p.value)))
                    .collect();
                format!("{{ {} }}", props.join(", "))
            }
            Expr::Array(array) => {
                let elements: Vec<String> = array.elements.iter()
                    .map(|e| self.expr_to_string(e))
                    .collect();
                format!("[{}]", elements.join(", "))
            }
            Expr::Lambda(lambda) => {
                let params = lambda.params.join(", ");
                let body = self.expr_to_string(&lambda.body);
                format!("({}) => {}", params, body)
            }
            Expr::Ternary(cond, then_expr, else_expr) => {
                format!("{} ? {} : {}", 
                    self.expr_to_string(cond),
                    self.expr_to_string(then_expr),
                    self.expr_to_string(else_expr))
            }
            Expr::Template(template) => {
                let mut result = String::new();
                result.push('`');
                for (i, quasi) in template.quasis.iter().enumerate() {
                    result.push_str(&quasi.value);
                    if i < template.expressions.len() {
                        result.push_str(&format!("${{{}}}", self.expr_to_string(&template.expressions[i])));
                    }
                }
                result.push('`');
                result
            }
            Expr::Await(await_expr) => {
                format!("await {}", self.expr_to_string(&await_expr.expr))
            }
            Expr::Spread(spread) => {
                format!("...{}", self.expr_to_string(&spread.expr))
            }
        }
    }
    
    fn binary_op_to_string(&self, op: &BinaryOp) -> &'static str {
        match op {
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
            BinaryOp::Mod => "%",
            BinaryOp::Eq => "===",
            BinaryOp::NotEq => "!==",
            BinaryOp::Lt => "<",
            BinaryOp::Gt => ">",
            BinaryOp::LtEq => "<=",
            BinaryOp::GtEq => ">=",
            BinaryOp::And => "&&",
            BinaryOp::Or => "||",
            BinaryOp::Nullish => "??",
        }
    }
    
    fn unary_op_to_string(&self, op: &UnaryOp) -> &'static str {
        match op {
            UnaryOp::Not => "!",
            UnaryOp::Neg => "-",
            UnaryOp::Typeof => "typeof ",
            UnaryOp::Void => "void ",
        }
    }
    
    fn emit(&mut self, text: &str) {
        self.output.push_str(text);
    }
    
    fn emit_line(&mut self, text: &str) {
        if !text.is_empty() {
            for _ in 0..self.indent {
                self.output.push_str("  ");
            }
            self.output.push_str(text);
        }
        self.output.push_str("\n");
    }
              }
