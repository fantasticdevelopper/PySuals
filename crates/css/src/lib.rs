mod parser;
mod scoped;
mod minify;
mod vendor;

pub use parser::CssParser;
pub use scoped::ScopedCss;
pub use minify::CssMinifier;
pub use vendor::VendorPrefixer;

use anyhow::Result;

#[derive(Debug, Clone)]
pub struct CssRule {
    pub selector: String,
    pub declarations: Vec<CssDeclaration>,
}

#[derive(Debug, Clone)]
pub struct CssDeclaration {
    pub property: String,
    pub value: String,
    pub important: bool,
}

#[derive(Debug, Clone)]
pub struct Stylesheet {
    pub rules: Vec<CssRule>,
    pub imports: Vec<String>,
}

pub struct CssProcessor {
    parser: CssParser,
    scoper: ScopedCss,
    minifier: CssMinifier,
    prefixer: VendorPrefixer,
}

impl CssProcessor {
    pub fn new() -> Self {
        Self {
            parser: CssParser::new(),
            scoper: ScopedCss::new(),
            minifier: CssMinifier::new(),
            prefixer: VendorPrefixer::new(),
        }
    }
    
    pub fn process(&mut self, css: &str, scope: Option<&str>) -> Result<String> {
        let stylesheet = self.parser.parse(css)?;
        let stylesheet = self.prefixer.process(stylesheet);
        let stylesheet = if let Some(scope_id) = scope {
            self.scoper.scope(stylesheet, scope_id)
        } else {
            stylesheet
        };
        let output = self.minifier.minify(&stylesheet);
        Ok(output)
    }
}

impl Default for CssProcessor {
    fn default() -> Self {
        Self::new()
    }
}
