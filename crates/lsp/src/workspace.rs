use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct WorkspaceManager {
    root_path: Option<PathBuf>,
    files: HashMap<String, String>,
}

impl WorkspaceManager {
    pub fn new() -> Self {
        Self {
            root_path: None,
            files: HashMap::new(),
        }
    }
    
    pub fn initialize(&mut self, root_uri: &str) {
        if let Some(path) = root_uri.strip_prefix("file://") {
            self.root_path = Some(PathBuf::from(path));
            self.scan_workspace();
        }
    }
    
    fn scan_workspace(&mut self) {
        if let Some(root) = &self.root_path {
            self.scan_directory(root);
        }
    }
    
    fn scan_directory(&mut self, dir: &Path) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                
                if path.is_dir() {
                    let name = path.file_name().unwrap_or_default();
                    if name != "node_modules" && name != ".git" && name != "dist" {
                        self.scan_directory(&path);
                    }
                } else if path.extension().map_or(false, |e| e == "pys" || e == "pydom") {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        let uri = format!("file://{}", path.display());
                        self.files.insert(uri, content);
                    }
                }
            }
        }
    }
    
    pub fn get_file(&self, uri: &str) -> Option<&String> {
        self.files.get(uri)
    }
    
    pub fn get_all_files(&self) -> Vec<(String, String)> {
        self.files.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }
    
    pub fn search_symbols(&self, query: &str) -> Vec<SymbolInfo> {
        let mut symbols = Vec::new();
        
        for (uri, content) in &self.files {
            for (i, line) in content.lines().enumerate() {
                if line.contains("@component") {
                    if let Some(name) = line.split_whitespace().nth(1) {
                        symbols.push(SymbolInfo {
                            name: name.to_string(),
                            kind: 2,
                            location: Location {
                                uri: uri.clone(),
                                range: Range {
                                    start: Position { line: i, character: 0 },
                                    end: Position { line: i, character: name.len() },
                                },
                            },
                        });
                    }
                }
                
                if line.contains(&query) {
                    symbols.push(SymbolInfo {
                        name: query.to_string(),
                        kind: 5,
                        location: Location {
                            uri: uri.clone(),
                            range: Range {
                                start: Position { line: i, character: 0 },
                                end: Position { line: i, character: query.len() },
                            },
                        },
                    });
                }
            }
        }
        
        symbols
    }
    
    pub fn get_references(&self, uri: &str, line: usize, character: usize) -> Vec<Location> {
        let mut references = Vec::new();
        let word = self.get_word_at_position(uri, line, character);
        
        if let Some(word) = word {
            for (file_uri, content) in &self.files {
                for (i, content_line) in content.lines().enumerate() {
                    if content_line.contains(&word) {
                        references.push(Location {
                            uri: file_uri.clone(),
                            range: Range {
                                start: Position { line: i, character: 0 },
                                end: Position { line: i, character: word.len() },
                            },
                        });
                    }
                }
            }
        }
        
        references
    }
    
    fn get_word_at_position(&self, uri: &str, line: usize, character: usize) -> Option<String> {
        if let Some(content) = self.files.get(uri) {
            let lines: Vec<&str> = content.lines().collect();
            if line < lines.len() {
                let line_content = lines[line];
                let chars: Vec<char> = line_content.chars().collect();
                
                if character < chars.len() && chars[character].is_alphabetic() {
                    let mut start = character;
                    let mut end = character;
                    
                    while start > 0 && chars[start - 1].is_alphabetic() {
                        start -= 1;
                    }
                    
                    while end < chars.len() && chars[end].is_alphabetic() {
                        end += 1;
                    }
                    
                    return Some(chars[start..end].iter().collect());
                }
            }
        }
        None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolInfo {
    pub name: String,
    pub kind: u32,
    pub location: Location,
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
