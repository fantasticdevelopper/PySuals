use crate::ast::*;
use std::collections::HashMap;

pub struct CSSGenerator {
    output: String,
    indent: usize,
    scoped_counter: u32,
    component_scopes: HashMap<String, String>,
}

impl CSSGenerator {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            indent: 0,
            scoped_counter: 0,
            component_scopes: HashMap::new(),
        }
    }
    
    pub fn generate(&mut self, program: &Program) -> String {
        self.output.clear();
        
        for css in &program.css {
            self.generate_css_block(css);
        }
        
        self.output.clone()
    }
    
    fn generate_css_block(&mut self, css: &CssBlock) {
        let selector = if css.scoped {
            self.generate_scoped_selector(&css.selector)
        } else {
            css.selector.clone()
        };
        
        self.emit_line(&format!("{} {{", selector));
        self.indent += 1;
        
        for rule in &css.rules {
            self.emit_line(&format!("  {}: {};", rule.property, rule.value));
        }
        
        self.indent -= 1;
        self.emit_line("}");
        self.emit_line("");
    }
    
    fn generate_scoped_selector(&mut self, selector: &str) -> String {
        let scope = self.get_next_scope();
        self.component_scopes.insert(selector.to_string(), scope.clone());
        
        if selector.starts_with('.') {
            format!("{}{}", selector, scope)
        } else if selector.starts_with('#') {
            format!("{}{}", selector, scope)
        } else {
            format!("{}{}", selector, scope)
        }
    }
    
    fn get_next_scope(&mut self) -> String {
        self.scoped_counter += 1;
        format!("_scope_{}", self.scoped_counter)
    }
    
    pub fn generate_component_css(&mut self, component: &Component) -> String {
        let mut css = String::new();
        
        if let Some(scope) = &component.css_scope {
            css.push_str(&format!(".{} {{}}\n", scope));
        }
        
        css
    }
    
    pub fn minify(&self, css: &str) -> String {
        let mut minified = String::new();
        let mut in_comment = false;
        
        for ch in css.chars() {
            if in_comment {
                if ch == '*' {
                    // Check for end of comment
                }
                continue;
            }
            
            match ch {
                '/' => {
                    // Start of comment
                }
                ' ' | '\n' | '\t' | '\r' => {
                    if !minified.ends_with(' ') {
                        minified.push(' ');
                    }
                }
                _ => {
                    minified.push(ch);
                }
            }
        }
        
        minified
    }
    
    pub fn autoprefix(&self, css: &str) -> String {
        let mut prefixed = String::new();
        
        for line in css.lines() {
            let mut new_line = line.to_string();
            
            if line.contains("display: flex") {
                new_line = new_line.replace("display: flex", 
                    "display: -webkit-flex;\n  display: flex");
            }
            
            if line.contains("transition:") {
                new_line = new_line.replace("transition:", 
                    "-webkit-transition:;\n  transition:");
            }
            
            if line.contains("transform:") {
                new_line = new_line.replace("transform:", 
                    "-webkit-transform:;\n  transform:");
            }
            
            prefixed.push_str(&new_line);
            prefixed.push('\n');
        }
        
        prefixed
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
