use serde_json::Value;

pub struct Formatter;

impl Formatter {
    pub fn new() -> Self {
        Self
    }
    
    pub fn format(&self, params: Option<&Value>) -> Option<Vec<TextEdit>> {
        let text_doc = params?.get("textDocument")?;
        let uri = text_doc.get("uri")?.as_str()?;
        let content = self.get_content(uri);
        
        let formatted = self.format_content(&content);
        
        Some(vec![TextEdit {
            range: Range {
                start: Position { line: 0, character: 0 },
                end: Position { line: 10000, character: 0 },
            },
            new_text: formatted,
        }])
    }
    
    fn get_content(&self, uri: &str) -> String {
        if let Some(path) = uri.strip_prefix("file://") {
            if let Ok(content) = std::fs::read_to_string(path) {
                return content;
            }
        }
        String::new()
    }
    
    fn format_content(&self, content: &str) -> String {
        let mut formatted = String::new();
        let mut indent = 0;
        
        for line in content.lines() {
            let trimmed = line.trim();
            
            if trimmed.ends_with(':') {
                formatted.push_str(&"    ".repeat(indent));
                formatted.push_str(trimmed);
                formatted.push('\n');
                indent += 1;
            } else if trimmed.starts_with('}') || trimmed.starts_with(')') {
                if indent > 0 {
                    indent -= 1;
                }
                formatted.push_str(&"    ".repeat(indent));
                formatted.push_str(trimmed);
                formatted.push('\n');
            } else {
                formatted.push_str(&"    ".repeat(indent));
                formatted.push_str(trimmed);
                formatted.push('\n');
            }
        }
        
        formatted
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextEdit {
    pub range: Range,
    pub new_text: String,
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
