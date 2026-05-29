use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: NodeId,
    pub kind: NodeKind,
    pub span: Span,
    pub children: Vec<Node>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub usize);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

impl Span {
    pub fn new(start: usize, end: usize, line: usize, column: usize) -> Self {
        Self { start, end, line, column }
    }
    
    pub fn len(&self) -> usize {
        self.end - self.start
    }
    
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
    
    pub fn merge(&self, other: &Span) -> Self {
        Self {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
            line: self.line.min(other.line),
            column: self.column,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeKind {
    Program(ProgramNode),
    Component(ComponentNode),
    Function(FunctionNode),
    Signal(SignalNode),
    Effect(EffectNode),
    Computed(ComputedNode),
    Import(ImportNode),
    Export(ExportNode),
    Expression(ExpressionNode),
    Statement(StatementNode),
    Block(BlockNode),
    Return(ReturnNode),
    If(IfNode),
    For(ForNode),
    While(WhileNode),
    Assign(AssignNode),
    Call(CallNode),
    Binary(BinaryNode),
    Unary(UnaryNode),
    Literal(LiteralNode),
    Ident(IdentNode),
    Object(ObjectNode),
    Array(ArrayNode),
    Member(MemberNode),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramNode {
    pub components: Vec<NodeId>,
    pub functions: Vec<NodeId>,
    pub imports: Vec<NodeId>,
    pub exports: Vec<NodeId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentNode {
    pub name: String,
    pub params: Vec<NodeId>,
    pub body: Vec<NodeId>,
    pub signals: Vec<NodeId>,
    pub computed: Vec<NodeId>,
    pub effects: Vec<NodeId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionNode {
    pub name: String,
    pub params: Vec<NodeId>,
    pub body: Vec<NodeId>,
    pub return_type: Option<String>,
    pub is_async: bool,
    pub is_generator: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalNode {
    pub name: String,
    pub initial: NodeId,
    pub scope: SignalScope,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignalScope {
    Local,
    Global,
    Context(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectNode {
    pub deps: Vec<String>,
    pub body: Vec<NodeId>,
    pub immediate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputedNode {
    pub name: String,
    pub deps: Vec<String>,
    pub body: NodeId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportNode {
    pub path: String,
    pub names: Vec<String>,
    pub is_default: bool,
    pub alias: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportNode {
    pub name: String,
    pub is_default: bool,
    pub alias: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressionNode {
    pub expr: Box<NodeKind>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatementNode {
    pub stmt: Box<NodeKind>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockNode {
    pub statements: Vec<NodeId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnNode {
    pub value: Option<NodeId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IfNode {
    pub condition: NodeId,
    pub then_branch: Vec<NodeId>,
    pub else_if_branches: Vec<(NodeId, Vec<NodeId>)>,
    pub else_branch: Option<Vec<NodeId>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForNode {
    pub item: String,
    pub iter: NodeId,
    pub body: Vec<NodeId>,
    pub key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhileNode {
    pub condition: NodeId,
    pub body: Vec<NodeId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignNode {
    pub target: String,
    pub value: NodeId,
    pub op: AssignOp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssignOp {
    Eq,
    AddEq,
    SubEq,
    MulEq,
    DivEq,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallNode {
    pub callee: NodeId,
    pub args: Vec<NodeId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryNode {
    pub left: NodeId,
    pub right: NodeId,
    pub op: BinaryOp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BinaryOp {
    Add, Sub, Mul, Div, Mod,
    Eq, Ne, Lt, Gt, Le, Ge,
    And, Or, Nullish,
    In, InstanceOf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnaryNode {
    pub expr: NodeId,
    pub op: UnaryOp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnaryOp {
    Not, Neg, Plus, Typeof, Void, Delete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiteralNode {
    pub value: LiteralValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LiteralValue {
    String(String),
    Number(f64),
    Integer(i64),
    Boolean(bool),
    Null,
    Undefined,
    Regex(String, String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentNode {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectNode {
    pub properties: Vec<ObjectProperty>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectProperty {
    pub key: String,
    pub value: NodeId,
    pub shorthand: bool,
    pub computed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrayNode {
    pub elements: Vec<NodeId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberNode {
    pub object: NodeId,
    pub property: String,
    pub computed: bool,
    pub optional: bool,
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}
