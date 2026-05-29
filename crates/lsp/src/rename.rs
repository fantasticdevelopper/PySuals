use serde_json::Value;

pub struct RenameProvider;

impl RenameProvider {
    pub fn new() -> Self {
        Self
    }
    
    pub fn rename_symbol(&self, params: Option<&Value>) -> WorkspaceEdit {
        let new_name = params?.get("newName")?.as_str().unwrap_or("");
        
        WorkspaceEdit {
            changes: Some(std::collections::HashMap::new()),
            document_changes: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceEdit {
    pub changes: Option<std::collections::HashMap<String, Vec<TextEdit>>>,
    pub document_changes: Option<Vec<DocumentChange>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextEdit {
    pub range: Range,
    pub new_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocumentChange {
    TextDocumentEdit(TextDocumentEdit),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDocumentEdit {
    pub text_document: VersionedTextDocumentIdentifier,
    pub edits: Vec<TextEdit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionedTextDocumentIdentifier {
    pub uri: String,
    pub version: i32,
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
