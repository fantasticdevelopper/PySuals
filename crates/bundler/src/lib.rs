mod graph;
mod chunk;
mod plugin;

pub use graph::DependencyGraph;
pub use chunk::ChunkManager;
pub use plugin::Plugin;

use std::path::PathBuf;
use anyhow::Result;

pub struct Bundler {
    entry: PathBuf,
    output: PathBuf,
    graph: DependencyGraph,
    chunk_manager: ChunkManager,
    plugins: Vec<Box<dyn Plugin>>,
}

impl Bundler {
    pub fn new(entry: PathBuf, output: PathBuf) -> Self {
        Self {
            entry,
            output,
            graph: DependencyGraph::new(),
            chunk_manager: ChunkManager::new(),
            plugins: Vec::new(),
        }
    }
    
    pub fn add_plugin(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }
    
    pub fn bundle(&mut self) -> Result<()> {
        self.graph.build(&self.entry)?;
        self.chunk_manager.split(&self.graph)?;
        
        for plugin in &self.plugins {
            plugin.transform(&mut self.chunk_manager)?;
        }
        
        self.chunk_manager.write_all(&self.output)?;
        Ok(())
    }
}
