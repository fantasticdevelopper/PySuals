use std::io::{stdin, stdout, Write};
use std::sync::Arc;
use tokio::sync::Mutex;
use serde_json::{Value, from_str, to_string};

use crate::completion::CompletionProvider;
use crate::hover::HoverProvider;
use crate::goto::GotoProvider;
use crate::rename::RenameProvider;
use crate::diagnostics::DiagnosticProvider;
use crate::formatting::Formatter;
use crate::workspace::WorkspaceManager;

pub struct LspServer {
    completion: CompletionProvider,
    hover: HoverProvider,
    goto: GotoProvider,
    rename: RenameProvider,
    diagnostics: DiagnosticProvider,
    formatter: Formatter,
    workspace: WorkspaceManager,
    documents: Arc<Mutex<Vec<Document>>>,
}

#[derive(Debug, Clone)]
pub struct Document {
    pub uri: String,
    pub content: String,
    pub version: i32,
}

impl LspServer {
    pub fn new() -> Self {
        Self {
            completion: CompletionProvider::new(),
            hover: HoverProvider::new(),
            goto: GotoProvider::new(),
            rename: RenameProvider::new(),
            diagnostics: DiagnosticProvider::new(),
            formatter: Formatter::new(),
            workspace: WorkspaceManager::new(),
            documents: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    pub fn start(&self) {
        let mut input = String::new();
        
        loop {
            input.clear();
            let mut stdin = stdin();
            stdin.read_line(&mut input).unwrap();
            
            if input.is_empty() {
                continue;
            }
            
            if let Ok(request) = from_str::<Value>(&input) {
                self.handle_request(request);
            }
        }
    }
    
    fn handle_request(&self, request: Value) {
        let method = request.get("method").and_then(|m| m.as_str());
        
        match method {
            Some("initialize") => self.send_initialize_response(request),
            Some("textDocument/completion") => self.send_completion_response(request),
            Some("textDocument/hover") => self.send_hover_response(request),
            Some("textDocument/definition") => self.send_definition_response(request),
            Some("textDocument/rename") => self.send_rename_response(request),
            Some("textDocument/didOpen") => self.handle_did_open(request),
            Some("textDocument/didChange") => self.handle_did_change(request),
            Some("textDocument/formatting") => self.send_formatting_response(request),
            Some("workspace/didChangeConfiguration") => self.handle_config_change(request),
            Some("shutdown") => self.handle_shutdown(request),
            _ => {}
        }
    }
    
    fn send_initialize_response(&self, request: Value) {
        let id = request.get("id");
        let response = serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "capabilities": {
                    "textDocumentSync": {
                        "openClose": true,
                        "change": 1,
                        "save": true
                    },
                    "completionProvider": {
                        "resolveProvider": true,
                        "triggerCharacters": [".", ":", "@"]
                    },
                    "hoverProvider": true,
                    "definitionProvider": true,
                    "renameProvider": true,
                    "documentFormattingProvider": true,
                    "workspaceSymbolProvider": true,
                    "referencesProvider": true
                },
                "serverInfo": {
                    "name": "PySuals LSP",
                    "version": "0.1.0"
                }
            }
        });
        self.send_response(&response);
    }
    
    fn send_completion_response(&self, request: Value) {
        let id = request.get("id");
        let params = request.get("params");
        
        let items = self.completion.get_completions(params);
        
        let response = serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": items
        });
        self.send_response(&response);
    }
    
    fn send_hover_response(&self, request: Value) {
        let id = request.get("id");
        let params = request.get("params");
        
        let hover = self.hover.get_hover(params);
        
        let response = serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": hover
        });
        self.send_response(&response);
    }
    
    fn send_definition_response(&self, request: Value) {
        let id = request.get("id");
        let params = request.get("params");
        
        let locations = self.goto.get_definition(params);
        
        let response = serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": locations
        });
        self.send_response(&response);
    }
    
    fn send_rename_response(&self, request: Value) {
        let id = request.get("id");
        let params = request.get("params");
        
        let edit = self.rename.rename_symbol(params);
        
        let response = serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": edit
        });
        self.send_response(&response);
    }
    
    fn send_formatting_response(&self, request: Value) {
        let id = request.get("id");
        let params = request.get("params");
        
        let edits = self.formatter.format(params);
        
        let response = serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": edits
        });
        self.send_response(&response);
    }
    
    fn handle_did_open(&self, request: Value) {
        let params = request.get("params");
        if let Some(text_doc) = params.and_then(|p| p.get("textDocument")) {
            let uri = text_doc.get("uri").and_then(|u| u.as_str()).unwrap_or("");
            let content = text_doc.get("text").and_then(|t| t.as_str()).unwrap_or("");
            let version = text_doc.get("version").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
            
            let doc = Document {
                uri: uri.to_string(),
                content: content.to_string(),
                version,
            };
            
            let docs = self.documents.clone();
            tokio::spawn(async move {
                docs.lock().await.push(doc);
            });
            
            self.publish_diagnostics(uri, content);
        }
    }
    
    fn handle_did_change(&self, request: Value) {
        let params = request.get("params");
        if let Some(text_doc) = params.and_then(|p| p.get("textDocument")) {
            let uri = text_doc.get("uri").and_then(|u| u.as_str()).unwrap_or("");
            let version = text_doc.get("version").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
            
            if let Some(content_changes) = params.and_then(|p| p.get("contentChanges")) {
                if let Some(first_change) = content_changes.get(0) {
                    let new_content = first_change.get("text").and_then(|t| t.as_str()).unwrap_or("");
                    
                    self.update_document(uri, version, new_content);
                    self.publish_diagnostics(uri, new_content);
                }
            }
        }
    }
    
    fn handle_config_change(&self, _request: Value) {}
    
    fn handle_shutdown(&self, request: Value) {
        let id = request.get("id");
        let response = serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": null
        });
        self.send_response(&response);
        std::process::exit(0);
    }
    
    fn update_document(&self, uri: &str, version: i32, content: &str) {
        let docs = self.documents.clone();
        let uri_owned = uri.to_string();
        let content_owned = content.to_string();
        
        tokio::spawn(async move {
            let mut documents = docs.lock().await;
            if let Some(doc) = documents.iter_mut().find(|d| d.uri == uri_owned) {
                doc.content = content_owned;
                doc.version = version;
            }
        });
    }
    
    fn publish_diagnostics(&self, uri: &str, content: &str) {
        let diagnostics = self.diagnostics.get_diagnostics(uri, content);
        
        let notification = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "textDocument/publishDiagnostics",
            "params": {
                "uri": uri,
                "diagnostics": diagnostics
            }
        });
        
        self.send_notification(&notification);
    }
    
    fn send_response(&self, response: &Value) {
        let output = to_string(response).unwrap();
        let _ = stdout().write_all(output.as_bytes());
        let _ = stdout().write_all(b"\n");
        let _ = stdout().flush();
    }
    
    fn send_notification(&self, notification: &Value) {
        let output = to_string(notification).unwrap();
        let _ = stdout().write_all(output.as_bytes());
        let _ = stdout().write_all(b"\n");
        let _ = stdout().flush();
    }
}
