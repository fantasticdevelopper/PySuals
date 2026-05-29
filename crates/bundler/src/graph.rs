use std::collections::{HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};
use anyhow::Result;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Node {
    pub path: PathBuf,
    pub content: String,
    pub deps: Vec<PathBuf>,
}

pub struct DependencyGraph {
    nodes: HashMap<PathBuf, Node>,
    roots: Vec<PathBuf>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            roots: Vec::new(),
        }
    }
    
    pub fn build(&mut self, entry: &Path) -> Result<()> {
        self.roots.push(entry.to_path_buf());
        self.add_node(entry)?;
        
        let mut queue: VecDeque<PathBuf> = VecDeque::new();
        queue.push_back(entry.to_path_buf());
        
        while let Some(path) = queue.pop_front() {
            let deps = self.extract_deps(&path)?;
            
            for dep in deps {
                if !self.nodes.contains_key(&dep) {
                    self.add_node(&dep)?;
                    queue.push_back(dep);
                }
            }
        }
        
        Ok(())
    }
    
    fn add_node(&mut self, path: &Path) -> Result<()> {
        let content = std::fs::read_to_string(path)?;
        let deps = self.extract_imports(&content);
        
        let node = Node {
            path: path.to_path_buf(),
            content,
            deps,
        };
        
        self.nodes.insert(path.to_path_buf(), node);
        Ok(())
    }
    
    fn extract_imports(&self, content: &str) -> Vec<PathBuf> {
        let mut deps = Vec::new();
        let re = Regex::new(r#"import\s+.*\s+from\s+["']([^"']+)["']"#).unwrap();
        
        for cap in re.captures_iter(content) {
            if let Some(path) = cap.get(1) {
                deps.push(PathBuf::from(path.as_str()));
            }
        }
        
        deps
    }
    
    fn extract_deps(&self, path: &Path) -> Result<Vec<PathBuf>> {
        let content = std::fs::read_to_string(path)?;
        Ok(self.extract_imports(&content))
    }
    
    pub fn get_node(&self, path: &Path) -> Option<&Node> {
        self.nodes.get(path)
    }
    
    pub fn get_all_nodes(&self) -> Vec<&Node> {
        self.nodes.values().collect()
    }
    
    pub fn topological_sort(&self) -> Vec<&Node> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        
        for root in &self.roots {
            self.dfs(root, &mut visited, &mut result);
        }
        
        result
    }
    
    fn dfs(&self, path: &Path, visited: &mut HashSet<PathBuf>, result: &mut Vec<&Node>) {
        if visited.contains(path) {
            return;
        }
        visited.insert(path.to_path_buf());
        
        if let Some(node) = self.nodes.get(path) {
            for dep in &node.deps {
                self.dfs(dep, visited, result);
            }
            result.push(node);
        }
    }
}
