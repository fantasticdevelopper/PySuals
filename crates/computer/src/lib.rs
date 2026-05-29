use anyhow::{Result, anyhow};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

mod ast;
mod parser;
mod analyzer;
mod codegen;
mod optimizer;

#[derive(Debug, Clone)]
pub struct CompilerConfig {
    pub sourcemap: bool,
    pub minify: bool,
    pub target: String,
    pub hmr: bool,
    pub optimize: bool,
    pub output_dir: PathBuf,
}

impl Default for CompilerConfig {
    fn default() -> Self {
        Self {
            sourcemap: true,
            minify: false,
            target: "es2020".to_string(),
            hmr: false,
            optimize: true,
            output_dir: PathBuf::from("dist"),
        }
    }
}

pub struct Compiler {
    source: String,
    source_path: PathBuf,
    output_path: PathBuf,
    config: CompilerConfig,
}

impl Compiler {
    pub fn new(source_path: PathBuf, output_path: PathBuf) -> Self {
        let source = fs::read_to_string(&source_path).unwrap_or_default();
        Self {
            source,
            source_path,
            output_path,
            config: CompilerConfig::default(),
        }
    }
    
    pub fn with_config(mut self, config: CompilerConfig) -> Self {
        self.config = config;
        self
    }
    
    pub fn compile(&self) -> Result<String> {
        let start = SystemTime::now();
        
        let tokens = parser::tokenize(&self.source)?;
        let ast = parser::parse(tokens)?;
        let validated = analyzer::validate(&ast)?;
        let transformed = analyzer::transform(validated)?;
        
        let optimized = if self.config.optimize {
            optimizer::optimize(transformed, &self.config)?
        } else {
            transformed
        };
        
        let js = codegen::generate(optimized, &self.config)?;
        
        if self.config.sourcemap {
            let map = codegen::generate_sourcemap(&self.source, &js);
            let map_path = self.output_path.with_extension("js.map");
            fs::write(&map_path, map)?;
        }
        
        let elapsed = start.elapsed().unwrap().as_millis();
        println!("Compiled {} in {}ms", self.source_path.display(), elapsed);
        
        Ok(js)
    }
    
    pub fn compile_to_file(&self) -> Result<()> {
        let js = self.compile()?;
        fs::write(&self.output_path, js)?;
        Ok(())
    }
    
    pub fn compile_watch<F>(&self, callback: F) -> Result<()>
    where
        F: Fn(String),
    {
        let mut last_modified = self.get_modified_time()?;
        
        loop {
            std::thread::sleep(std::time::Duration::from_millis(100));
            
            let current = self.get_modified_time()?;
            if current > last_modified {
                last_modified = current;
                match self.compile() {
                    Ok(js) => {
                        if let Err(e) = fs::write(&self.output_path, &js) {
                            callback(format!("Write error: {}", e));
                        } else {
                            callback(format!("OK - {}", self.output_path.display()));
                        }
                    }
                    Err(e) => callback(format!("Error: {}", e)),
                }
            }
        }
    }
    
    fn get_modified_time(&self) -> Result<SystemTime> {
        let metadata = fs::metadata(&self.source_path)?;
        Ok(metadata.modified()?)
    }
    
    pub fn get_dependencies(&self) -> Result<Vec<String>> {
        let mut deps = Vec::new();
        for line in self.source.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("import") || trimmed.starts_with("from") {
                if let Some(path) = self.extract_import_path(trimmed) {
                    deps.push(path);
                }
            }
        }
        Ok(deps)
    }
    
    fn extract_import_path(&self, line: &str) -> Option<String> {
        let parts: Vec<&str> = line.split('"').collect();
        if parts.len() > 1 {
            return Some(parts[1].to_string());
        }
        let parts: Vec<&str> = line.split("'").collect();
        if parts.len() > 1 {
            return Some(parts[1].to_string());
        }
        None
    }
    
    pub fn analyze(&self) -> Result<CompileAnalysis> {
        let tokens = parser::tokenize(&self.source)?;
        let ast = parser::parse(tokens)?;
        
        let mut signal_count = 0;
        let mut component_count = 0;
        let mut effect_count = 0;
        let mut line_count = self.source.lines().count();
        
        for component in &ast.components {
            component_count += 1;
            signal_count += component.signals.len();
            effect_count += component.effects.len();
        }
        
        Ok(CompileAnalysis {
            signal_count,
            component_count,
            effect_count,
            line_count,
            import_count: ast.imports.len(),
        })
    }
}

pub struct CompileAnalysis {
    pub signal_count: usize,
    pub component_count: usize,
    pub effect_count: usize,
    pub line_count: usize,
    pub import_count: usize,
}
