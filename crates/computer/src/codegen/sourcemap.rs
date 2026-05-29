use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceMap {
    pub version: u32,
    pub sources: Vec<String>,
    pub names: Vec<String>,
    pub mappings: String,
    pub sourcesContent: Option<Vec<String>>,
}

pub struct SourceMapGenerator {
    mappings: Vec<Mapping>,
    sources: Vec<String>,
    names: Vec<String>,
    source_index: HashMap<String, usize>,
    name_index: HashMap<String, usize>,
}

#[derive(Debug, Clone)]
struct Mapping {
    generated_line: usize,
    generated_col: usize,
    source_line: usize,
    source_col: usize,
    source_idx: usize,
    name_idx: Option<usize>,
}

impl SourceMapGenerator {
    pub fn new() -> Self {
        Self {
            mappings: Vec::new(),
            sources: Vec::new(),
            names: Vec::new(),
            source_index: HashMap::new(),
            name_index: HashMap::new(),
        }
    }
    
    pub fn generate(&mut self, js: &str) -> String {
        let map = SourceMap {
            version: 3,
            sources: self.sources.clone(),
            names: self.names.clone(),
            mappings: self.encode_mappings(),
            sourcesContent: None,
        };
        
        serde_json::to_string(&map).unwrap()
    }
    
    pub fn add_mapping(&mut self, mapping: Mapping) {
        self.mappings.push(mapping);
    }
    
    pub fn add_source(&mut self, source: String) -> usize {
        if let Some(&idx) = self.source_index.get(&source) {
            idx
        } else {
            let idx = self.sources.len();
            self.sources.push(source.clone());
            self.source_index.insert(source, idx);
            idx
        }
    }
    
    pub fn add_name(&mut self, name: String) -> usize {
        if let Some(&idx) = self.name_index.get(&name) {
            idx
        } else {
            let idx = self.names.len();
            self.names.push(name.clone());
            self.name_index.insert(name, idx);
            idx
        }
    }
    
    fn encode_mappings(&self) -> String {
        let mut mappings = String::new();
        let mut prev_gen_line = 0;
        let mut prev_gen_col = 0;
        let mut prev_source_idx = 0;
        let mut prev_source_line = 0;
        let mut prev_source_col = 0;
        let mut prev_name_idx = 0;
        
        let mut current_line = 0;
        let mut line_mappings: Vec<&Mapping> = self.mappings.iter()
            .filter(|m| m.generated_line == current_line)
            .collect();
        
        while !line_mappings.is_empty() {
            for (i, mapping) in line_mappings.iter().enumerate() {
                if i > 0 {
                    mappings.push(',');
                }
                
                let gen_col = mapping.generated_col as i32 - prev_gen_col as i32;
                mappings.push_str(&self.encode_vlq(gen_col));
                
                let source_idx = mapping.source_idx as i32 - prev_source_idx as i32;
                mappings.push_str(&self.encode_vlq(source_idx));
                
                let source_line = mapping.source_line as i32 - prev_source_line as i32;
                mappings.push_str(&self.encode_vlq(source_line));
                
                let source_col = mapping.source_col as i32 - prev_source_col as i32;
                mappings.push_str(&self.encode_vlq(source_col));
                
                if let Some(name_idx) = mapping.name_idx {
                    let name = name_idx as i32 - prev_name_idx as i32;
                    mappings.push_str(&self.encode_vlq(name));
                    prev_name_idx = name_idx;
                }
                
                prev_gen_col = mapping.generated_col;
                prev_source_idx = mapping.source_idx;
                prev_source_line = mapping.source_line;
                prev_source_col = mapping.source_col;
            }
            
            current_line += 1;
            prev_gen_line = current_line;
            prev_gen_col = 0;
            
            line_mappings = self.mappings.iter()
                .filter(|m| m.generated_line == current_line)
                .collect();
            
            if !line_mappings.is_empty() {
                mappings.push(';');
            }
        }
        
        mappings
    }
    
    fn encode_vlq(&self, value: i32) -> String {
        let mut val = if value < 0 {
            ((-value) << 1) | 1
        } else {
            value << 1
        };
        
        let mut result = String::new();
        
        loop {
            let mut digit = (val & 31) as u8;
            val >>= 5;
            
            if val != 0 {
                digit |= 32;
            }
            
            result.push(self.to_base64(digit));
            
            if val == 0 {
                break;
            }
        }
        
        result
    }
    
    fn to_base64(&self, value: u8) -> char {
        const BASE64: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        BASE64[value as usize] as char
    }
    
    pub fn generate_from_strings(source: &str, output: &str) -> String {
        let mut generator = Self::new();
        let source_idx = generator.add_source("input.pys".to_string());
        
        let source_lines: Vec<&str> = source.lines().collect();
        let output_lines: Vec<&str> = output.lines().collect();
        
        for (line_idx, output_line) in output_lines.iter().enumerate() {
            if let Some(source_line) = source_lines.get(line_idx) {
                let mapping = Mapping {
                    generated_line: line_idx,
                    generated_col: 0,
                    source_line: line_idx,
                    source_col: 0,
                    source_idx,
                    name_idx: None,
                };
                generator.add_mapping(mapping);
            }
        }
        
        generator.generate(output)
    }
}
