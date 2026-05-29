use crate::{Stylesheet, CssRule, CssDeclaration};

pub struct VendorPrefixer;

impl VendorPrefixer {
    pub fn new() -> Self {
        Self
    }
    
    pub fn process(&self, mut stylesheet: Stylesheet) -> Stylesheet {
        for rule in &mut stylesheet.rules {
            let mut prefixed_declarations = Vec::new();
            
            for decl in &rule.declarations {
                let prefixed = self.prefix_declaration(decl);
                prefixed_declarations.extend(prefixed);
            }
            
            rule.declarations = prefixed_declarations;
        }
        
        stylesheet
    }
    
    fn prefix_declaration(&self, decl: &CssDeclaration) -> Vec<CssDeclaration> {
        let mut result = Vec::new();
        
        match decl.property.as_str() {
            "user-select" => {
                result.push(CssDeclaration {
                    property: "-webkit-user-select".to_string(),
                    value: decl.value.clone(),
                    important: decl.important,
                });
                result.push(decl.clone());
            }
            "transform" => {
                result.push(CssDeclaration {
                    property: "-webkit-transform".to_string(),
                    value: decl.value.clone(),
                    important: decl.important,
                });
                result.push(decl.clone());
            }
            "transition" => {
                result.push(CssDeclaration {
                    property: "-webkit-transition".to_string(),
                    value: decl.value.clone(),
                    important: decl.important,
                });
                result.push(decl.clone());
            }
            "animation" => {
                result.push(CssDeclaration {
                    property: "-webkit-animation".to_string(),
                    value: decl.value.clone(),
                    important: decl.important,
                });
                result.push(decl.clone());
            }
            "display" if decl.value == "flex" => {
                result.push(CssDeclaration {
                    property: "display".to_string(),
                    value: "-webkit-flex".to_string(),
                    important: decl.important,
                });
                result.push(decl.clone());
            }
            "appearance" => {
                result.push(CssDeclaration {
                    property: "-webkit-appearance".to_string(),
                    value: decl.value.clone(),
                    important: decl.important,
                });
                result.push(CssDeclaration {
                    property: "-moz-appearance".to_string(),
                    value: decl.value.clone(),
                    important: decl.important,
                });
                result.push(decl.clone());
            }
            _ => {
                result.push(decl.clone());
            }
        }
        
        result
    }
    
    pub fn prefix_keyframes(&self, name: &str) -> Vec<String> {
        vec![
            format!("@-webkit-keyframes {}", name),
            format!("@keyframes {}", name),
        ]
    }
    
    pub fn needs_prefix(&self, property: &str) -> bool {
        matches!(
            property,
            "user-select" | "transform" | "transition" | "animation" | "appearance"
        )
    }
}
