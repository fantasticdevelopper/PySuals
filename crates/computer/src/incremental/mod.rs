use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

mod cache;
mod hash;

use cache::CacheManager;
use hash::HashCalculator;

pub struct IncrementalCompiler {
    cache_dir: PathBuf,
    cache_manager: CacheManager,
    hash_calculator: HashCalculator,
    file_hashes: HashMap<PathBuf, String>,
    last_build_time: u64,
}

#[derive(Debug, Clone)]
pub struct BuildArtifact {
    pub path: PathBuf,
    pub hash: String,
    pub size: u64,
    pub modified: u64,
}

impl IncrementalCompiler {
    pub fn new(cache_dir: PathBuf) -> Self {
        fs::create_dir_all(&cache_dir).unwrap();
        
        Self {
            cache_dir,
            cache_manager: CacheManager::new(),
            hash_calculator: HashCalculator::new(),
            file_hashes: HashMap::new(),
            last_build_time: 0,
        }
    }
    
    pub fn needs_rebuild(&mut self, input_path: &Path) -> bool {
        let current_hash = self.hash_calculator.hash_file(input_path);
        let cached_hash = self.cache_manager.get_hash(input_path);
        
        if current_hash != cached_hash {
            self.file_hashes.insert(input_path.to_path_buf(), current_hash);
            return true;
        }
        
        false
    }
    
    pub fn get_cached_output(&self, input_path: &Path) -> Option<Vec<u8>> {
        self.cache_manager.get_output(input_path)
    }
    
    pub fn store_output(&mut self, input_path: &Path, output: &[u8]) {
        let hash = self.file_hashes.get(input_path).cloned();
        self.cache_manager.store_output(input_path, output, hash);
    }
    
    pub fn clean_cache(&mut self) {
        self.cache_manager.clean();
        self.file_hashes.clear();
    }
    
    pub fn get_cache_stats(&self) -> CacheStats {
        self.cache_manager.get_stats()
    }
}

pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub size_bytes: u64,
    pub file_count: u64,
}

impl Default for CacheStats {
    fn default() -> Self {
        Self {
            hits: 0,
            misses: 0,
            size_bytes: 0,
            file_count: 0,
        }
    }
}
