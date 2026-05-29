use serde_json::Value;
use pysuals::parser;

pub struct DiagnosticProvider;

impl DiagnosticProvider {
    pub fn new() -> Self {
        Self
    }
    
    pub fn get_diagnostics(&self, uri: &str, content: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        
        let lines: Vec<&str> = content.lines().collect();
        
        for (i, line) in lines.iter().enumerate() {
            if line.contains("@compnent") {
                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position { line: i, character: 0 },
                        end: Position { line: i, character: 10 },
                    },
                    severity: Some(1),
                    code: None,
                    source: Some("PySuals".to_string()),
                    message: "Did you mean @component?".to_string(),
                });
            }
            
            if line.contains("siganl") {
                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position { line: i, character: 0 },
                        end: Position { line: i, character: 6 },
                    },
                    severity: Some(1),
                    code: None,
                    source: Some("PySuals".to_string()),
                    message: "Did you mean signal?".to_string(),
                });
            }
            
            if line.trim().starts_with("def") && !line.contains("->") {
                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position { line: i, character: 0 },
                        end: Position { line: i, character: 3 },
                    },
                    severity: Some(2),
                    code: None,
                    source: Some("PySuals".to_string()),
                    message: "Missing return type hint".to_string(),
                });
            }
        }
        
        diagnostics
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub range: Range,
    pub severity: Option<u32>,
    pub code: Option<String>,
    pub source: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub line: usize,
    pub character: usize,
}
