use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CacheEntry {
    pub hash: String,
    pub output: Vec<u8>,
    pub timestamp: u64,
    pub size: u64,
}

pub struct CacheManager {
    cache_dir: PathBuf,
    index: HashMap<PathBuf, CacheEntry>,
    stats: super::CacheStats,
}

impl CacheManager {
    pub fn new() -> Self {
        let cache_dir = PathBuf::from(".pysuals/cache");
        fs::create_dir_all(&cache_dir).unwrap();
        
        let mut manager = Self {
            cache_dir,
            index: HashMap::new(),
            stats: super::CacheStats::default(),
        };
        
        manager.load_index();
        manager
    }
    
    pub fn get_hash(&self, path: &Path) -> Option<String> {
        self.index.get(path).map(|e| e.hash.clone())
    }
    
    pub fn get_output(&self, path: &Path) -> Option<Vec<u8>> {
        if let Some(entry) = self.index.get(path) {
            self.stats.hits += 1;
            return Some(entry.output.clone());
        }
        self.stats.misses += 1;
        None
    }
    
    pub fn store_output(&mut self, path: &Path, output: &[u8], hash: Option<String>) {
        let hash = hash.unwrap_or_else(|| self.calculate_hash(output));
        let timestamp = self.get_timestamp();
        
        let entry = CacheEntry {
            hash,
            output: output.to_vec(),
            timestamp,
            size: output.len() as u64,
        };
        
        self.store_to_disk(path, &entry);
        self.index.insert(path.to_path_buf(), entry);
        self.save_index();
    }
    
    fn store_to_disk(&self, path: &Path, entry: &CacheEntry) {
        let cache_file = self.get_cache_path(path);
        let data = bincode::serialize(entry).unwrap();
        fs::write(cache_file, data).unwrap();
    }
    
    fn load_from_disk(&self, path: &Path) -> Option<CacheEntry> {
        let cache_file = self.get_cache_path(path);
        if let Ok(data) = fs::read(cache_file) {
            return bincode::deserialize(&data).ok();
        }
        None
    }
    
    fn load_index(&mut self) {
        let index_file = self.cache_dir.join("index.bin");
        if let Ok(data) = fs::read(index_file) {
            if let Ok(index) = bincode::deserialize(&data) {
                self.index = index;
            }
        }
    }
    
    fn save_index(&self) {
        let index_file = self.cache_dir.join("index.bin");
        let data = bincode::serialize(&self.index).unwrap();
        fs::write(index_file, data).unwrap();
    }
    
    pub fn clean(&mut self) {
        let now = self.get_timestamp();
        let max_age = 30 * 24 * 60 * 60; // 30 days
        
        self.index.retain(|path, entry| {
            if now - entry.timestamp > max_age {
                let cache_file = self.get_cache_path(path);
                let _ = fs::remove_file(cache_file);
                false
            } else {
                true
            }
        });
        
        self.save_index();
        self.update_stats();
    }
    
    fn calculate_hash(&self, data: &[u8]) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }
    
    fn get_cache_path(&self, path: &Path) -> PathBuf {
        let hash = self.calculate_hash(path.to_str().unwrap().as_bytes());
        self.cache_dir.join(format!("{}.cache", hash))
    }
    
    fn get_timestamp(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
    
    pub fn update_stats(&mut self) {
        let mut total_size = 0;
        let mut file_count = 0;
        
        for entry in self.index.values() {
            total_size += entry.size;
            file_count += 1;
        }
        
        self.stats.size_bytes = total_size;
        self.stats.file_count = file_count;
    }
    
    pub fn get_stats(&self) -> super::CacheStats {
        self.stats.clone()
    }
    
    pub fn clear_all(&mut self) {
        for path in self.index.keys() {
            let cache_file = self.get_cache_path(path);
            let _ = fs::remove_file(cache_file);
        }
        
        self.index.clear();
        self.save_index();
        self.update_stats();
    }
    
    pub fn get_disk_usage(&self) -> u64 {
        self.stats.size_bytes
    }
}

impl Default for CacheManager {
    fn default() -> Self {
        Self::new()
    }
}
