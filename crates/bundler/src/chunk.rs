use std::collections::HashMap;
use std::path::{Path, PathBuf};
use anyhow::Result;
use crate::graph::DependencyGraph;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub name: String,
    pub modules: Vec<PathBuf>,
    pub code: String,
    pub is_entry: bool,
}

pub struct ChunkManager {
    chunks: Vec<Chunk>,
}

impl ChunkManager {
    pub fn new() -> Self {
        Self {
            chunks: Vec::new(),
        }
    }
    
    pub fn split(&mut self, graph: &DependencyGraph) -> Result<()> {
        let sorted = graph.topological_sort();
        
        let mut entry_chunk = Chunk {
            name: "main".to_string(),
            modules: Vec::new(),
            code: String::new(),
            is_entry: true,
        };
        
        for node in sorted {
            entry_chunk.modules.push(node.path.clone());
            entry_chunk.code.push_str(&format!("// {}\n", node.path.display()));
            entry_chunk.code.push_str(&node.content);
            entry_chunk.code.push_str("\n\n");
        }
        
        self.chunks.push(entry_chunk);
        
        self.split_by_size();
        
        Ok(())
    }
    
    fn split_by_size(&mut self) {
        let max_size = 1024 * 100;
        let mut new_chunks = Vec::new();
        
        for chunk in &self.chunks {
            if chunk.code.len() > max_size && chunk.modules.len() > 1 {
                let half = chunk.modules.len() / 2;
                let (first, second) = chunk.modules.split_at(half);
                
                let mut chunk1 = Chunk {
                    name: format!("{}.1", chunk.name),
                    modules: first.to_vec(),
                    code: String::new(),
                    is_entry: false,
                };
                
                let mut chunk2 = Chunk {
                    name: format!("{}.2", chunk.name),
                    modules: second.to_vec(),
                    code: String::new(),
                    is_entry: false,
                };
                
                new_chunks.push(chunk1);
                new_chunks.push(chunk2);
            } else {
                new_chunks.push(chunk.clone());
            }
        }
        
        self.chunks = new_chunks;
    }
    
    pub fn write_all(&self, output_dir: &Path) -> Result<()> {
        std::fs::create_dir_all(output_dir)?;
        
        for chunk in &self.chunks {
            let filename = if chunk.is_entry {
                "main.js"
            } else {
                &format!("chunk_{}.js", chunk.name)
            };
            
            let output_path = output_dir.join(filename);
            std::fs::write(output_path, &chunk.code)?;
        }
        
        Ok(())
    }
    
    pub fn get_chunks(&self) -> &[Chunk] {
        &self.chunks
    }
}
