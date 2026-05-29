use anyhow::Result;
use crate::chunk::ChunkManager;

pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn transform(&self, chunk_manager: &mut ChunkManager) -> Result<()>;
    fn before_bundle(&self) -> Result<()> { Ok(()) }
    fn after_bundle(&self) -> Result<()> { Ok(()) }
}

pub struct MinifyPlugin;

impl Plugin for MinifyPlugin {
    fn name(&self) -> &str {
        "minify"
    }
    
    fn transform(&self, chunk_manager: &mut ChunkManager) -> Result<()> {
        for chunk in chunk_manager.get_chunks() {
            let minified = self.minify_js(&chunk.code);
            // chunk.code = minified; (need mut access)
        }
        Ok(())
    }
}

impl MinifyPlugin {
    fn minify_js(&self, code: &str) -> String {
        let mut result = String::new();
        let mut in_comment = false;
        
        for line in code.lines() {
            let trimmed = line.trim();
            
            if trimmed.starts_with("//") {
                continue;
            }
            
            if !trimmed.is_empty() {
                result.push_str(trimmed);
            }
        }
        
        result
    }
}

pub struct SourceMapPlugin;

impl Plugin for SourceMapPlugin {
    fn name(&self) -> &str {
        "sourcemap"
    }
    
    fn transform(&self, _chunk_manager: &mut ChunkManager) -> Result<()> {
        Ok(())
    }
}

pub struct TreeShakePlugin;

impl Plugin for TreeShakePlugin {
    fn name(&self) -> &str {
        "tree-shake"
    }
    
    fn transform(&self, _chunk_manager: &mut ChunkManager) -> Result<()> {
        Ok(())
    }
}
