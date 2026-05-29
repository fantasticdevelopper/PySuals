use crate::ast::*;
use super::CompilerConfig;

mod js;
mod html;
mod css;
mod sourcemap;

use js::JSGenerator;
use html::HTMLGenerator;
use css::CSSGenerator;
use sourcemap::SourceMapGenerator;

pub struct CodeGenerator {
    js_gen: JSGenerator,
    html_gen: HTMLGenerator,
    css_gen: CSSGenerator,
    sourcemap_gen: SourceMapGenerator,
    config: CompilerConfig,
}

impl CodeGenerator {
    pub fn new(config: &CompilerConfig) -> Self {
        Self {
            js_gen: JSGenerator::new(config),
            html_gen: HTMLGenerator::new(),
            css_gen: CSSGenerator::new(),
            sourcemap_gen: SourceMapGenerator::new(),
            config: config.clone(),
        }
    }
    
    pub fn generate(&mut self, program: Program) -> String {
        let mut output = String::new();
        
        let js = self.js_gen.generate(&program);
        let css = self.css_gen.generate(&program);
        let html = self.html_gen.generate(&program);
        
        if !css.is_empty() {
            output.push_str(&format!("<style>{}</style>\n", css));
        }
        
        output.push_str(&html);
        output.push_str("\n<script type=\"module\">\n");
        output.push_str(&js);
        output.push_str("\n</script>\n");
        
        if self.config.sourcemap {
            let map = self.sourcemap_gen.generate(&js);
            output.push_str(&format!("//# sourceMappingURL=data:application/json;base64,{}\n", 
                base64::encode(map)));
        }
        
        output
    }
    
    pub fn generate_js_only(&mut self, program: Program) -> String {
        self.js_gen.generate(&program)
    }
    
    pub fn generate_css_only(&mut self, program: Program) -> String {
        self.css_gen.generate(&program)
    }
    
    pub fn generate_html_only(&mut self, program: Program) -> String {
        self.html_gen.generate(&program)
    }
}

pub fn generate(program: Program, config: &CompilerConfig) -> String {
    let mut generator = CodeGenerator::new(config);
    generator.generate(program)
}

pub fn generate_sourcemap(source: &str, output: &str) -> String {
    SourceMapGenerator::generate_from_strings(source, output)
}

fn base64_encode(data: &str) -> String {
    use base64::{Engine as _, engine::general_purpose};
    general_purpose::STANDARD.encode(data)
}
