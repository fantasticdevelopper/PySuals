use crate::{Stylesheet, CssRule, CssDeclaration};

pub struct CssMinifier;

impl CssMinifier {
    pub fn new() -> Self {
        Self
    }
    
    pub fn minify(&self, stylesheet: &Stylesheet) -> String {
        let mut result = String::new();
        
        for rule in &stylesheet.rules {
            result.push_str(&self.minify_selector(&rule.selector));
            result.push('{');
            
            for decl in &rule.declarations {
                result.push_str(&self.minify_declaration(decl));
            }
            
            result.push('}');
        }
        
        result
    }
    
    fn minify_selector(&self, selector: &str) -> String {
        selector
            .replace(" :", ":")
            .replace(" > ", ">")
            .replace(" + ", "+")
            .replace(" ~ ", "~")
            .split(',')
            .map(|s| s.trim())
            .collect::<Vec<_>>()
            .join(",")
    }
    
    fn minify_declaration(&self, decl: &CssDeclaration) -> String {
        let prop = decl.property.trim();
        let value = self.minify_value(&decl.value);
        
        if decl.important {
            format!("{}:{}!important;", prop, value)
        } else {
            format!("{}:{};", prop, value)
        }
    }
    
    fn minify_value(&self, value: &str) -> String {
        let mut result = value
            .replace("  ", " ")
            .replace(" 0px", " 0")
            .replace("0px ", "0 ")
            .replace("0px", "0")
            .replace(" 0%", " 0")
            .replace("0% ", "0 ");
        
        if result.starts_with("rgba(") && result.ends_with(')') {
            result = self.minify_rgba(&result);
        }
        
        result
    }
    
    fn minify_rgba(&self, rgba: &str) -> String {
        let parts: Vec<&str> = rgba[5..rgba.len()-1].split(',').collect();
        if parts.len() == 4 {
            let r = parts[0].trim();
            let g = parts[1].trim();
            let b = parts[2].trim();
            let a = parts[3].trim();
            
            if a == "1" {
                return format!("rgb({},{},{})", r, g, b);
            }
        }
        rgba.to_string()
    }
    
    pub fn remove_comments(&self, css: &str) -> String {
        let mut result = String::new();
        let mut in_comment = false;
        let chars: Vec<char> = css.chars().collect();
        let mut i = 0;
        
        while i < chars.len() {
            if !in_comment && chars[i] == '/' && i + 1 < chars.len() && chars[i + 1] == '*' {
                in_comment = true;
                i += 2;
                continue;
            }
            
            if in_comment && chars[i] == '*' && i + 1 < chars.len() && chars[i + 1] == '/' {
                in_comment = false;
                i += 2;
                continue;
            }
            
            if !in_comment {
                result.push(chars[i]);
            }
            
            i += 1;
        }
        
        result
    }
}
