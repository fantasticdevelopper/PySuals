use anyhow::Result;
use regex::Regex;

use crate::{Stylesheet, CssRule, CssDeclaration};

pub struct CssParser;

impl CssParser {
    pub fn new() -> Self {
        Self
    }
    
    pub fn parse(&self, css: &str) -> Result<Stylesheet> {
        let mut stylesheet = Stylesheet {
            rules: Vec::new(),
            imports: Vec::new(),
        };
        
        let rule_re = Regex::new(r"([^{]+)\{([^}]+)\}").unwrap();
        
        for cap in rule_re.captures_iter(css) {
            let selector = cap[1].trim().to_string();
            let declarations_str = &cap[2];
            
            if selector.starts_with("@import") {
                let import_re = Regex::new(r#"@import\s+["']([^"']+)["']"#).unwrap();
                if let Some(import_cap) = import_re.captures(&selector) {
                    stylesheet.imports.push(import_cap[1].to_string());
                }
                continue;
            }
            
            let mut declarations = Vec::new();
            let decl_re = Regex::new(r"([^:;]+):([^;]+);?").unwrap();
            
            for decl_cap in decl_re.captures_iter(declarations_str) {
                let property = decl_cap[1].trim().to_string();
                let mut value = decl_cap[2].trim().to_string();
                let mut important = false;
                
                if value.ends_with("!important") {
                    important = true;
                    value = value.trim_end_matches("!important").trim().to_string();
                }
                
                declarations.push(CssDeclaration {
                    property,
                    value,
                    important,
                });
            }
            
            if !declarations.is_empty() {
                stylesheet.rules.push(CssRule {
                    selector,
                    declarations,
                });
            }
        }
        
        Ok(stylesheet)
    }
    
    pub fn parse_selector(&self, selector: &str) -> Vec<String> {
        selector.split(',').map(|s| s.trim().to_string()).collect()
    }
    
    pub fn parse_declaration(&self, decl: &str) -> Option<CssDeclaration> {
        let parts: Vec<&str> = decl.split(':').collect();
        if parts.len() == 2 {
            Some(CssDeclaration {
                property: parts[0].trim().to_string(),
                value: parts[1].trim().to_string(),
                important: false,
            })
        } else {
            None
        }
    }
}
