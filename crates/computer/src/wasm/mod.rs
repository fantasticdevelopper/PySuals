use wasm_bindgen::prelude::*;
use crate::Compiler;
use crate::CompilerConfig;
use std::path::PathBuf;

#[wasm_bindgen]
pub struct WasmCompiler {
    compiler: Option<Compiler>,
}

#[wasm_bindgen]
impl WasmCompiler {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        Self {
            compiler: None,
        }
    }
    
    #[wasm_bindgen]
    pub fn compile(&mut self, source: &str, source_path: &str) -> Result<String, String> {
        let config = CompilerConfig::default();
        let compiler = Compiler::new(
            PathBuf::from(source_path),
            PathBuf::from("output.js"),
        ).with_config(config);
        
        let result = compiler.compile();
        match result {
            Ok(js) => Ok(js),
            Err(e) => Err(e.to_string()),
        }
    }
    
    #[wasm_bindgen]
    pub fn compile_with_options(&mut self, source: &str, source_path: &str, minify: bool, sourcemap: bool) -> Result<String, String> {
        let config = CompilerConfig {
            minify,
            sourcemap,
            ..CompilerConfig::default()
        };
        
        let compiler = Compiler::new(
            PathBuf::from(source_path),
            PathBuf::from("output.js"),
        ).with_config(config);
        
        match compiler.compile() {
            Ok(js) => Ok(js),
            Err(e) => Err(e.to_string()),
        }
    }
    
    #[wasm_bindgen]
    pub fn analyze(&mut self, source: &str) -> Result<JsValue, String> {
        let compiler = Compiler::new(
            PathBuf::from("input.pys"),
            PathBuf::from("output.js"),
        );
        
        match compiler.analyze() {
            Ok(analysis) => Ok(serde_wasm_bindgen::to_value(&analysis).unwrap()),
            Err(e) => Err(e.to_string()),
        }
    }
    
    #[wasm_bindgen]
    pub fn get_version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }
}

#[wasm_bindgen]
pub fn compile_sync(source: &str) -> String {
    let config = CompilerConfig::default();
    let compiler = Compiler::new(
        PathBuf::from("input.pys"),
        PathBuf::from("output.js"),
    ).with_config(config);
    
    compiler.compile().unwrap_or_else(|e| format!("Error: {}", e))
}

#[wasm_bindgen]
pub fn tokenize(source: &str) -> JsValue {
    let tokens = crate::parser::tokenize(source).unwrap_or_default();
    serde_wasm_bindgen::to_value(&tokens).unwrap()
}

#[wasm_bindgen]
pub fn parse(source: &str) -> JsValue {
    let ast = crate::parser::parse(source).unwrap_or_default();
    serde_wasm_bindgen::to_value(&ast).unwrap()
}
