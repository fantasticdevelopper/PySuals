use serde_json::Value;
use pysuals::parser;

pub struct CompletionProvider {
    keywords: Vec<String>,
    builtins: Vec<String>,
}

impl CompletionProvider {
    pub fn new() -> Self {
        Self {
            keywords: vec![
                "component".to_string(),
                "signal".to_string(),
                "effect".to_string(),
                "computed".to_string(),
                "def".to_string(),
                "return".to_string(),
                "if".to_string(),
                "else".to_string(),
                "for".to_string(),
                "in".to_string(),
                "while".to_string(),
                "import".to_string(),
                "from".to_string(),
            ],
            builtins: vec![
                "div".to_string(),
                "span".to_string(),
                "button".to_string(),
                "input".to_string(),
                "form".to_string(),
                "h1".to_string(),
                "h2".to_string(),
                "h3".to_string(),
                "p".to_string(),
                "a".to_string(),
                "img".to_string(),
                "ul".to_string(),
                "li".to_string(),
            ],
        }
    }
    
    pub fn get_completions(&self, params: Option<&Value>) -> Vec<CompletionItem> {
        let mut items = Vec::new();
        
        for keyword in &self.keywords {
            items.push(CompletionItem {
                label: keyword.clone(),
                kind: 15,
                detail: Some("Keyword".to_string()),
                documentation: None,
            });
        }
        
        for builtin in &self.builtins {
            items.push(CompletionItem {
                label: builtin.clone(),
                kind: 7,
                detail: Some("HTML element".to_string()),
                documentation: Some(format!("Creates a <{}> element", builtin)),
            });
        }
        
        items
    }
    
    pub fn resolve_completion(&self, item: CompletionItem) -> CompletionItem {
        item
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionItem {
    pub label: String,
    pub kind: u32,
    pub detail: Option<String>,
    pub documentation: Option<String>,
      }
