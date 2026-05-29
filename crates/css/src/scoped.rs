use crate::{Stylesheet, CssRule};

pub struct ScopedCss {
    scope_id: u32,
}

impl ScopedCss {
    pub fn new() -> Self {
        Self {
            scope_id: 0,
        }
    }
    
    pub fn scope(&mut self, mut stylesheet: Stylesheet, scope: &str) -> Stylesheet {
        for rule in &mut stylesheet.rules {
            rule.selector = self.scope_selector(&rule.selector, scope);
        }
        stylesheet
    }
    
    fn scope_selector(&self, selector: &str, scope: &str) -> String {
        let parts: Vec<&str> = selector.split(',').collect();
        let mut scoped_parts = Vec::new();
        
        for part in parts {
            let trimmed = part.trim();
            let scoped = if trimmed.starts_with(':') || trimmed.starts_with('@') {
                trimmed.to_string()
            } else {
                format!("{}[data-pysuals-{}]", trimmed, scope)
            };
            scoped_parts.push(scoped);
        }
        
        scoped_parts.join(", ")
    }
    
    pub fn generate_scope_id(&mut self) -> String {
        self.scope_id += 1;
        format!("_scope_{}", self.scope_id)
    }
    
    pub fn scope_keyframes(&self, name: &str, scope: &str) -> String {
        format!("{}_{}", name, scope)
    }
    
    pub fn add_data_attribute(&self, selector: &str, scope: &str) -> String {
        if selector.contains('[',) {
            format!("{}[data-pysuals-{}]", selector, scope)
        } else {
            format!("{}[data-pysuals-{}]", selector, scope)
        }
    }
}
