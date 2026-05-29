use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub components: Vec<Component>,
    pub functions: Vec<Function>,
    pub imports: Vec<Import>,
    pub exports: Vec<Export>,
    pub css: Vec<CssBlock>,
}

impl Default for Program {
    fn default() -> Self {
        Self {
            components: Vec::new(),
            functions: Vec::new(),
            imports: Vec::new(),
            exports: Vec::new(),
            css: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub name: String,
    pub params: Vec<Param>,
    pub body: Vec<Stmt>,
    pub signals: Vec<Signal>,
    pub computed: Vec<Computed>,
    pub effects: Vec<Effect>,
    pub css_scope: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signal {
    pub name: String,
    pub initial: Expr,
    pub scope: SignalScope,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignalScope {
    Component,
    Global,
    Context,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Computed {
    pub name: String,
    pub deps: Vec<String>,
    pub body: Expr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Effect {
    pub deps: Vec<String>,
    pub body: Vec<Stmt>,
    pub immediate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub body: Vec<Stmt>,
    pub return_type: Option<String>,
    pub is_async: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Import {
    pub path: String,
    pub names: Vec<String>,
    pub is_default: bool,
    pub alias: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Export {
    pub name: String,
    pub is_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Param {
    pub name: String,
    pub type_hint: Option<String>,
    pub default: Option<Expr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Stmt {
    Expr(Expr),
    Return(Expr),
    If(IfStmt),
    For(ForStmt),
    While(WhileStmt),
    Assign(AssignStmt),
    Var(VarStmt),
    Block(Vec<Stmt>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IfStmt {
    pub cond: Expr,
    pub body: Vec<Stmt>,
    pub else_if: Vec<(Expr, Vec<Stmt>)>,
    pub else_body: Option<Vec<Stmt>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForStmt {
    pub item: String,
    pub iter: Expr,
    pub body: Vec<Stmt>,
    pub key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhileStmt {
    pub cond: Expr,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignStmt {
    pub target: String,
    pub value: Expr,
    pub op: AssignOp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssignOp {
    Eq,
    AddEq,
    SubEq,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VarStmt {
    pub name: String,
    pub value: Expr,
    pub is_const: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expr {
    Literal(Literal),
    Ident(String),
    Call(CallExpr),
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Lambda(LambdaExpr),
    Object(ObjectExpr),
    Array(ArrayExpr),
    Member(MemberExpr),
    Ternary(TernaryExpr),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallExpr {
    pub callee: Box<Expr>,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub op: BinOp,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnaryExpr {
    pub op: UnOp,
    pub expr: Box<Expr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LambdaExpr {
    pub params: Vec<String>,
    pub body: Box<Expr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectExpr {
    pub props: Vec<ObjectProp>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectProp {
    pub key: String,
    pub value: Expr,
    pub shorthand: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrayExpr {
    pub elements: Vec<Expr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberExpr {
    pub obj: Box<Expr>,
    pub prop: String,
    pub computed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TernaryExpr {
    pub cond: Box<Expr>,
    pub then: Box<Expr>,
    pub else_: Box<Expr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Literal {
    String(String),
    Number(f64),
    Integer(i64),
    Boolean(bool),
    Null,
    Undefined,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BinOp {
    Add, Sub, Mul, Div, Mod,
    Eq, Ne, Lt, Gt, Le, Ge,
    And, Or, Nullish,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnOp {
    Not, Neg, Typeof, Void,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CssBlock {
    pub selector: String,
    pub rules: Vec<CssRule>,
    pub scoped: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CssRule {
    pub property: String,
    pub value: String,
}
