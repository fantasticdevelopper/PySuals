use std::fs;
use std::path::Path;
use std::collections::HashMap;
use sha2::{Sha256, Digest};

pub struct HashCalculator {
    file_hashes: HashMap<String, String>,
}

impl HashCalculator {
    pub fn new() -> Self {
        Self {
            file_hashes: HashMap::new(),
        }
    }
    
    pub fn hash_file(&mut self, path: &Path) -> String {
        let path_str = path.to_str().unwrap();
        
        if let Some(cached) = self.file_hashes.get(path_str) {
            return cached.clone();
        }
        
        let content = fs::read(path).unwrap_or_default();
        let hash = self.hash_bytes(&content);
        
        self.file_hashes.insert(path_str.to_string(), hash.clone());
        hash
    }
    
    pub fn hash_bytes(&self, data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }
    
    pub fn hash_string(&self, text: &str) -> String {
        self.hash_bytes(text.as_bytes())
    }
    
    pub fn hash_ast(&self, ast: &str) -> String {
        self.hash_string(ast)
    }
    
    pub fn combine_hashes(&self, hashes: &[String]) -> String {
        let mut hasher = Sha256::new();
        
        for hash in hashes {
            hasher.update(hash.as_bytes());
        }
        
        format!("{:x}", hasher.finalize())
    }
    
    pub fn hash_dependencies(&self, paths: &[&Path]) -> String {
        let mut hashes = Vec::new();
        
        for path in paths {
            if let Ok(content) = fs::read(path) {
                hashes.push(self.hash_bytes(&content));
            }
        }
        
        self.combine_hashes(&hashes)
    }
    
    pub fn fast_hash(&self, data: &[u8]) -> u64 {
        use std::hash::{Hasher, BuildHasher};
        let mut hasher = std::collections::hash_map::RandomState::new().build_hasher();
        hasher.write(data);
        hasher.finish()
    }
    
    pub fn hash_config(&self, config: &serde_json::Value) -> String {
        let config_str = serde_json::to_string(config).unwrap();
        self.hash_string(&config_str)
    }
    
    pub fn verify_hash(&self, data: &[u8], expected: &str) -> bool {
        self.hash_bytes(data) == expected
    }
}

pub struct IncrementalHash {
    last_hash: Option<String>,
    current_hash: Option<String>,
}

impl IncrementalHash {
    pub fn new() -> Self {
        Self {
            last_hash: None,
            current_hash: None,
        }
    }
    
    pub fn update(&mut self, data: &[u8], calculator: &HashCalculator) -> bool {
        let new_hash = calculator.hash_bytes(data);
        self.current_hash = Some(new_hash.clone());
        
        match &self.last_hash {
            Some(last) if last == &new_hash => false,
            _ => true,
        }
    }
    
    pub fn commit(&mut self) {
        self.last_hash = self.current_hash.clone();
    }
    
    pub fn rollback(&mut self) {
        self.current_hash = self.last_hash.clone();
    }
    
    pub fn has_changed(&self) -> bool {
        self.last_hash != self.current_hash
    }
    
    pub fn get_current(&self) -> Option<&str> {
        self.current_hash.as_deref()
    }
    
    pub fn get_last(&self) -> Option<&str> {
        self.last_hash.as_deref()
    }
}

impl Default for IncrementalHash {
    fn default() -> Self {
        Self::new()
    }
}
