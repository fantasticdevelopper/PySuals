use std::collections::HashMap;
use crate::ast::*;

#[derive(Debug, Clone, PartialEq)]
pub enum PyType {
    String,
    Number,
    Integer,
    Boolean,
    Null,
    Undefined,
    Array(Box<PyType>),
    Object(HashMap<String, PyType>),
    Function(Vec<PyType>, Box<PyType>),
    Signal(Box<PyType>),
    Component,
    Any,
    Void,
}

pub struct TypeChecker {
    types: HashMap<String, PyType>,
    errors: Vec<String>,
    current_component: Option<String>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            types: HashMap::new(),
            errors: Vec::new(),
            current_component: None,
        }
    }
    
    pub fn check(&mut self, program: &Program) {
        for component in &program.components {
            self.current_component = Some(component.name.clone());
            self.check_component(component);
        }
        
        for function in &program.functions {
            self.check_function(function);
        }
    }
    
    fn check_component(&mut self, component: &Component) {
        for signal in &component.signals {
            let typ = self.infer_type(&signal.initial);
            self.types.insert(signal.name.clone(), typ);
        }
        
        for stmt in &component.body {
            self.check_stmt(stmt);
        }
    }
    
    fn check_function(&mut self, function: &Function) {
        for param in &function.params {
            if let Some(type_hint) = &param.type_hint {
                let typ = self.type_from_string(type_hint);
                self.types.insert(param.name.clone(), typ);
            }
        }
        
        for stmt in &function.body {
            self.check_stmt(stmt);
        }
    }
    
    fn check_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Expr(expr) => {
                self.infer_type(expr);
            }
            Stmt::Return(expr) => {
                let typ = self.infer_type(expr);
                self.check_return_type(typ);
            }
            Stmt::If(if_stmt) => {
                let cond_type = self.infer_type(&if_stmt.cond);
                if cond_type != PyType::Boolean {
                    self.errors.push(format!(
                        "If condition must be boolean, got {:?}", cond_type
                    ));
                }
                
                for stmt in &if_stmt.body {
                    self.check_stmt(stmt);
                }
            }
            Stmt::Assign(assign) => {
                let value_type = self.infer_type(&assign.value);
                self.types.insert(assign.target.clone(), value_type);
            }
            Stmt::Var(var) => {
                let value_type = self.infer_type(&var.value);
                self.types.insert(var.name.clone(), value_type);
            }
            _ => {}
        }
    }
    
    fn infer_type(&mut self, expr: &Expr) -> PyType {
        match expr {
            Expr::Literal(lit) => match &lit.value {
                LiteralValue::String(_) => PyType::String,
                LiteralValue::Number(_) => PyType::Number,
                LiteralValue::Integer(_) => PyType::Integer,
                LiteralValue::Boolean(_) => PyType::Boolean,
                LiteralValue::Null => PyType::Null,
                LiteralValue::Undefined => PyType::Undefined,
                LiteralValue::BigInt(_) => PyType::Number,
            }
            Expr::Ident(ident) => {
                self.types.get(&ident.name).cloned().unwrap_or(PyType::Any)
            }
            Expr::Binary(binary) => {
                let left = self.infer_type(&binary.left);
                let right = self.infer_type(&binary.right);
                
                match binary.op {
                    BinaryOp::Add => {
                        if left == PyType::String || right == PyType::String {
                            PyType::String
                        } else {
                            PyType::Number
                        }
                    }
                    BinaryOp::Eq | BinaryOp::NotEq => PyType::Boolean,
                    BinaryOp::Lt | BinaryOp::Gt | BinaryOp::LtEq | BinaryOp::GtEq => PyType::Boolean,
                    BinaryOp::And | BinaryOp::Or => PyType::Boolean,
                    _ => PyType::Number,
                }
            }
            Expr::Unary(unary) => {
                match unary.op {
                    UnaryOp::Not => PyType::Boolean,
                    UnaryOp::Neg => PyType::Number,
                    _ => PyType::Any,
                }
            }
            Expr::Call(call) => {
                let callee_type = self.infer_type(&call.callee);
                
                for arg in &call.args {
                    self.infer_type(arg);
                }
                
                match callee_type {
                    PyType::Function(_, ret) => *ret,
                    _ => PyType::Any,
                }
            }
            Expr::Array(array) => {
                if let Some(first) = array.elements.first() {
                    let elem_type = self.infer_type(first);
                    PyType::Array(Box::new(elem_type))
                } else {
                    PyType::Array(Box::new(PyType::Any))
                }
            }
            Expr::Object(object) => {
                let mut props = HashMap::new();
                for prop in &object.props {
                    let prop_type = self.infer_type(&prop.value);
                    props.insert(prop.key.clone(), prop_type);
                }
                PyType::Object(props)
            }
            Expr::Lambda(lambda) => {
                let param_types = vec![PyType::Any];
                let ret_type = self.infer_type(&lambda.body);
                PyType::Function(param_types, Box::new(ret_type))
            }
            Expr::Ternary(cond, then_expr, else_expr) => {
                let then_type = self.infer_type(then_expr);
                let else_type = self.infer_type(else_expr);
                
                if then_type == else_type {
                    then_type
                } else {
                    PyType::Any
                }
            }
            _ => PyType::Any,
        }
    }
    
    fn check_return_type(&mut self, typ: PyType) {
        if let Some(component) = &self.current_component {
            if typ == PyType::Component {
                return;
            }
        }
    }
    
    fn type_from_string(&self, type_str: &str) -> PyType {
        match type_str {
            "str" => PyType::String,
            "int" => PyType::Integer,
            "float" => PyType::Number,
            "bool" => PyType::Boolean,
            "None" => PyType::Null,
            _ => PyType::Any,
        }
    }
    
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    
    pub fn get_errors(&self) -> Vec<String> {
        self.errors.clone()
    }
    
    pub fn validate(&self, _program: &Program) -> bool {
        !self.has_errors()
    }
    
    pub fn get_type(&self, expr: &Expr) -> Option<TypeInfo> {
        Some(TypeInfo {
            type_name: format!("{:?}", self.infer_type(expr)),
            nullable: false,
            optional: false,
        })
    }
}

pub struct TypeInfo {
    pub type_name: String,
    pub nullable: bool,
    pub optional: bool,
}
