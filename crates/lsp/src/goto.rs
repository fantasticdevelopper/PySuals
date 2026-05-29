use serde_json::Value;

pub struct GotoProvider;

impl GotoProvider {
    pub fn new() -> Self {
        Self
    }
    
    pub fn get_definition(&self, params: Option<&Value>) -> Vec<Location> {
        let text_doc = params?.get("textDocument")?;
        let position = params?.get("position")?;
        
        let uri = text_doc.get("uri")?.as_str()?;
        let line = position.get("line")?.as_u64()? as usize;
        
        vec![Location {
            uri: uri.to_string(),
            range: Range {
                start: Position { line, character: 0 },
                end: Position { line, character: 10 },
            },
        }]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub uri: String,
    pub range: Range,
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
