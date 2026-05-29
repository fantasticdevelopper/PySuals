use crate::ast::*;

pub struct HTMLGenerator {
    output: String,
    indent: usize,
}

impl HTMLGenerator {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            indent: 0,
        }
    }
    
    pub fn generate(&mut self, program: &Program) -> String {
        self.output.clear();
        
        self.emit_line("<!DOCTYPE html>");
        self.emit_line("<html lang=\"en\">");
        self.indent += 1;
        
        self.generate_head();
        self.generate_body(program);
        
        self.indent -= 1;
        self.emit_line("</html>");
        
        self.output.clone()
    }
    
    fn generate_head(&mut self) {
        self.emit_line("<head>");
        self.indent += 1;
        
        self.emit_line("<meta charset=\"UTF-8\">");
        self.emit_line("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">");
        self.emit_line("<title>PySuals App</title>");
        
        self.indent -= 1;
        self.emit_line("</head>");
    }
    
    fn generate_body(&mut self, program: &Program) {
        self.emit_line("<body>");
        self.indent += 1;
        
        self.emit_line("<div id=\"app\"></div>");
        
        for css in &program.css {
            self.generate_css(css);
        }
        
        self.indent -= 1;
        self.emit_line("</body>");
    }
    
    fn generate_css(&mut self, css: &CssBlock) {
        let selector = if css.scoped {
            format!("{}[data-pysuals-scope]", css.selector)
        } else {
            css.selector.clone()
        };
        
        self.emit_line("<style>");
        self.indent += 1;
        
        self.emit_line(&format!("{} {{", selector));
        self.indent += 1;
        
        for rule in &css.rules {
            self.emit_line(&format!("{}: {};", rule.property, rule.value));
        }
        
        self.indent -= 1;
        self.emit_line("}");
        
        self.indent -= 1;
        self.emit_line("</style>");
    }
    
    pub fn generate_component_html(&mut self, component: &Component) -> String {
        let mut html = String::new();
        
        for stmt in &component.body {
            if let Stmt::Return(expr) = stmt {
                html = self.expr_to_html(expr);
                break;
            }
        }
        
        html
    }
    
    fn expr_to_html(&self, expr: &Expr) -> String {
        match expr {
            Expr::Call(call) => {
                if let Expr::Ident(ident) = &*call.callee {
                    match ident.name.as_str() {
                        "div" => self.element_to_html("div", &call.args),
                        "span" => self.element_to_html("span", &call.args),
                        "button" => self.element_to_html("button", &call.args),
                        "h1" => self.element_to_html("h1", &call.args),
                        "h2" => self.element_to_html("h2", &call.args),
                        "p" => self.element_to_html("p", &call.args),
                        "a" => self.element_to_html("a", &call.args),
                        "img" => self.element_to_html("img", &call.args),
                        "input" => self.element_to_html("input", &call.args),
                        "form" => self.element_to_html("form", &call.args),
                        _ => format!("<{} />", ident.name),
                    }
                } else {
                    String::new()
                }
            }
            _ => String::new(),
        }
    }
    
    fn element_to_html(&self, tag: &str, args: &[Expr]) -> String {
        let mut attributes = Vec::new();
        let mut children = Vec::new();
        
        for arg in args {
            if let Expr::Object(obj) = arg {
                for prop in &obj.props {
                    if prop.key == "className" || prop.key == "class" {
                        if let Expr::Literal(lit) = &prop.value {
                            attributes.push(format!("class=\"{}\"", lit.raw));
                        }
                    } else if prop.key == "id" {
                        if let Expr::Literal(lit) = &prop.value {
                            attributes.push(format!("id=\"{}\"", lit.raw));
                        }
                    } else if prop.key.starts_with("on") {
                        let event = &prop.key[2..];
                        attributes.push(format!("on{}={{}}", event));
                    }
                }
            } else if let Expr::Literal(lit) = arg {
                children.push(lit.raw.clone());
            } else if let Expr::Call(call) = arg {
                children.push(self.expr_to_html(arg));
            } else if let Expr::Array(arr) = arg {
                for elem in &arr.elements {
                    children.push(self.expr_to_html(elem));
                }
            }
        }
        
        if children.is_empty() {
            format!("<{}{} />", tag, self.attributes_to_string(&attributes))
        } else {
            format!("<{}{}>{}</{}>", 
                tag, 
                self.attributes_to_string(&attributes),
                children.join(""),
                tag)
        }
    }
    
    fn attributes_to_string(&self, attrs: &[String]) -> String {
        if attrs.is_empty() {
            String::new()
        } else {
            format!(" {}", attrs.join(" "))
        }
    }
    
    fn emit(&mut self, text: &str) {
        self.output.push_str(text);
    }
    
    fn emit_line(&mut self, text: &str) {
        for _ in 0..self.indent {
            self.output.push_str("  ");
        }
        self.output.push_str(text);
        self.output.push_str("\n");
    }
}
