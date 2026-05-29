use serde::{Serialize, Deserialize};
use super::expr::Expr;
use super::node::NodeId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Stmt {
    Block(BlockStmt),
    Expr(ExprStmt),
    If(IfStmt),
    For(ForStmt),
    While(WhileStmt),
    Return(ReturnStmt),
    Assign(AssignStmt),
    Var(VarStmt),
    Try(TryStmt),
    Throw(ThrowStmt),
    Break(BreakStmt),
    Continue(ContinueStmt),
    Switch(SwitchStmt),
    Labeled(LabeledStmt),
    Debugger(DebuggerStmt),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockStmt {
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExprStmt {
    pub expr: Expr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IfStmt {
    pub test: Expr,
    pub consequent: Box<Stmt>,
    pub alternate: Option<Box<Stmt>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForStmt {
    pub init: Option<ForInit>,
    pub test: Option<Expr>,
    pub update: Option<Expr>,
    pub body: Box<Stmt>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ForInit {
    Var(VarDecl),
    Assign(AssignStmt),
    Expr(Expr),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForInStmt {
    pub left: ForTarget,
    pub right: Expr,
    pub body: Box<Stmt>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForOfStmt {
    pub left: ForTarget,
    pub right: Expr,
    pub body: Box<Stmt>,
    pub await_: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ForTarget {
    Var(VarDecl),
    Assign(AssignStmt),
    Expr(Expr),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhileStmt {
    pub test: Expr,
    pub body: Box<Stmt>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoWhileStmt {
    pub body: Box<Stmt>,
    pub test: Expr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnStmt {
    pub argument: Option<Expr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignStmt {
    pub left: AssignTarget,
    pub right: Expr,
    pub op: AssignOp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssignTarget {
    Ident(String),
    Member(MemberTarget),
    Pattern(Pattern),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberTarget {
    pub object: Expr,
    pub property: Expr,
    pub computed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Pattern {
    Ident(String),
    Array(Vec<PatternElement>),
    Object(Vec<ObjectPatternProp>),
    Rest(Box<Pattern>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternElement {
    Pattern(Pattern),
    Default(Pattern, Expr),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectPatternProp {
    pub key: PropKey,
    pub value: Pattern,
    pub shorthand: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropKey {
    Ident(String),
    String(String),
    Number(f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssignOp {
    Eq,
    AddEq,
    SubEq,
    MulEq,
    DivEq,
    ModEq,
    PowEq,
    AndEq,
    OrEq,
    NullishEq,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VarStmt {
    pub declarations: Vec<VarDecl>,
    pub kind: VarKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VarDecl {
    pub id: Pattern,
    pub init: Option<Expr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VarKind {
    Var,
    Let,
    Const,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TryStmt {
    pub block: BlockStmt,
    pub handler: Option<CatchClause>,
    pub finalizer: Option<BlockStmt>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatchClause {
    pub param: Option<Pattern>,
    pub body: BlockStmt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThrowStmt {
    pub argument: Expr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakStmt {
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinueStmt {
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwitchStmt {
    pub discriminant: Expr,
    pub cases: Vec<SwitchCase>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwitchCase {
    pub test: Option<Expr>,
    pub consequent: Vec<Stmt>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabeledStmt {
    pub label: String,
    pub body: Box<Stmt>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebuggerStmt;

impl Stmt {
    pub fn is_empty(&self) -> bool {
        match self {
            Stmt::Block(block) => block.body.is_empty(),
            _ => false,
        }
    }
    
    pub fn contains_return(&self) -> bool {
        match self {
            Stmt::Block(block) => block.body.iter().any(|s| s.contains_return()),
            Stmt::Return(_) => true,
            Stmt::If(if_stmt) => {
                if_stmt.consequent.contains_return() 
                    && if_stmt.alternate.as_ref().map_or(false, |a| a.contains_return())
            }
            _ => false,
        }
    }
}
