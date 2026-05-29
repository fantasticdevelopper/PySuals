use serde_json::Value;

pub struct HoverProvider;

impl HoverProvider {
    pub fn new() -> Self {
        Self
    }
    
    pub fn get_hover(&self, params: Option<&Value>) -> Option<Hover> {
        let position = params?.get("position")?;
        let line = position.get("line")?.as_u64()? as usize;
        let character = position.get("character")?.as_u64()? as usize;
        
        Some(Hover {
            contents: HoverContents {
                kind: "markdown".to_string(),
                value: format!("**PySuals Component**\n\nDefined at line {}", line),
            },
            range: Some(Range {
                start: Position { line, character },
                end: Position { line, character: character + 1 },
            }),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hover {
    pub contents: HoverContents,
    pub range: Option<Range>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoverContents {
    pub kind: String,
    pub value: String,
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
