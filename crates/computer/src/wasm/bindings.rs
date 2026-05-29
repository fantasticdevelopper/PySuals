use wasm_bindgen::prelude::*;
use js_sys::{Array, Object, Map};
use std::collections::HashMap;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
    
    #[wasm_bindgen(js_namespace = performance)]
    fn now() -> f64;
}

#[wasm_bindgen]
pub struct PerformanceTimer {
    start_time: f64,
    name: String,
}

#[wasm_bindgen]
impl PerformanceTimer {
    #[wasm_bindgen(constructor)]
    pub fn new(name: &str) -> Self {
        Self {
            start_time: now(),
            name: name.to_string(),
        }
    }
    
    #[wasm_bindgen]
    pub fn stop(&self) -> f64 {
        let elapsed = now() - self.start_time;
        log(&format!("{} took {:.2}ms", self.name, elapsed));
        elapsed
    }
}

#[wasm_bindgen]
pub struct DiagnosticCollector {
    errors: Array,
    warnings: Array,
}

#[wasm_bindgen]
impl DiagnosticCollector {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            errors: Array::new(),
            warnings: Array::new(),
        }
    }
    
    #[wasm_bindgen]
    pub fn add_error(&mut self, message: &str, line: u32, column: u32) {
        let error = Object::new();
        js_sys::Reflect::set(&error, &"message".into(), &message.into()).unwrap();
        js_sys::Reflect::set(&error, &"line".into(), &line.into()).unwrap();
        js_sys::Reflect::set(&error, &"column".into(), &column.into()).unwrap();
        self.errors.push(&error);
    }
    
    #[wasm_bindgen]
    pub fn add_warning(&mut self, message: &str, line: u32, column: u32) {
        let warning = Object::new();
        js_sys::Reflect::set(&warning, &"message".into(), &message.into()).unwrap();
        js_sys::Reflect::set(&warning, &"line".into(), &line.into()).unwrap();
        js_sys::Reflect::set(&warning, &"column".into(), &column.into()).unwrap();
        self.warnings.push(&warning);
    }
    
    #[wasm_bindgen]
    pub fn has_errors(&self) -> bool {
        self.errors.length() > 0
    }
    
    #[wasm_bindgen]
    pub fn get_errors(&self) -> Array {
        self.errors.clone()
    }
    
    #[wasm_bindgen]
    pub fn get_warnings(&self) -> Array {
        self.warnings.clone()
    }
    
    #[wasm_bindgen]
    pub fn clear(&mut self) {
        self.errors = Array::new();
        self.warnings = Array::new();
    }
}

#[wasm_bindgen]
pub struct SourceMapProcessor {
    mappings: Map,
}

#[wasm_bindgen]
impl SourceMapProcessor {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            mappings: Map::new(),
        }
    }
    
    #[wasm_bindgen]
    pub fn add_mapping(&mut self, generated_line: u32, generated_col: u32, source_line: u32, source_col: u32, source: &str) {
        let key = format!("{}:{}", generated_line, generated_col);
        let value = Object::new();
        js_sys::Reflect::set(&value, &"sourceLine".into(), &source_line.into()).unwrap();
        js_sys::Reflect::set(&value, &"sourceCol".into(), &source_col.into()).unwrap();
        js_sys::Reflect::set(&value, &"source".into(), &source.into()).unwrap();
        self.mappings.set(&key.into(), &value);
    }
    
    #[wasm_bindgen]
    pub fn generate(&self, output: &str) -> String {
        format!("{}", output)
    }
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn debug_print(value: &JsValue) {
    log(&format!("{:?}", value));
}

#[wasm_bindgen(start)]
pub fn main() {
    init_panic_hook();
    log("PySuals WASM module loaded");
}
