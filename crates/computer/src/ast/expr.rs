use serde::{Serialize, Deserialize};
use super::node::NodeId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expr {
    Literal(LiteralExpr),
    Ident(IdentExpr),
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Call(CallExpr),
    Member(MemberExpr),
    Object(ObjectExpr),
    Array(ArrayExpr),
    Ternary(TernaryExpr),
    Lambda(LambdaExpr),
    Template(TemplateExpr),
    Await(AwaitExpr),
    Spread(SpreadExpr),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiteralExpr {
    pub value: LiteralValue,
    pub raw: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LiteralValue {
    String(String),
    Number(f64),
    Integer(i64),
    Boolean(bool),
    Null,
    Undefined,
    BigInt(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentExpr {
    pub name: String,
    pub global: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub op: BinaryOp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Eq,
    NotEq,
    StrictEq,
    StrictNotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    And,
    Or,
    Nullish,
    In,
    InstanceOf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnaryExpr {
    pub expr: Box<Expr>,
    pub op: UnaryOp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnaryOp {
    Not,
    Neg,
    Plus,
    Typeof,
    Void,
    Delete,
    BitwiseNot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallExpr {
    pub callee: Box<Expr>,
    pub args: Vec<Expr>,
    pub optional: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberExpr {
    pub object: Box<Expr>,
    pub property: Box<Expr>,
    pub computed: bool,
    pub optional: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectExpr {
    pub properties: Vec<ObjectProp>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectProp {
    pub key: PropKey,
    pub value: Expr,
    pub shorthand: bool,
    pub method: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropKey {
    Ident(String),
    String(String),
    Number(f64),
    Computed(Box<Expr>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrayExpr {
    pub elements: Vec<ArrayElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArrayElement {
    Value(Expr),
    Spread(Box<Expr>),
    Hole,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TernaryExpr {
    pub condition: Box<Expr>,
    pub then_expr: Box<Expr>,
    pub else_expr: Box<Expr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LambdaExpr {
    pub params: Vec<LambdaParam>,
    pub body: LambdaBody,
    pub async_: bool,
    pub generator: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LambdaParam {
    pub name: String,
    pub default: Option<Box<Expr>>,
    pub rest: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LambdaBody {
    Expr(Box<Expr>),
    Block(Vec<NodeId>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateExpr {
    pub quasis: Vec<TemplateElement>,
    pub expressions: Vec<Expr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateElement {
    pub value: String,
    pub tail: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwaitExpr {
    pub expr: Box<Expr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpreadExpr {
    pub expr: Box<Expr>,
}

impl Expr {
    pub fn is_constant(&self) -> bool {
        match self {
            Expr::Literal(_) => true,
            Expr::Binary(bin) => bin.left.is_constant() && bin.right.is_constant(),
            Expr::Unary(unary) => unary.expr.is_constant(),
            _ => false,
        }
    }
    
    pub fn get_type(&self) -> &'static str {
        match self {
            Expr::Literal(lit) => match lit.value {
                LiteralValue::String(_) => "string",
                LiteralValue::Number(_) => "number",
                LiteralValue::Integer(_) => "number",
                LiteralValue::Boolean(_) => "boolean",
                LiteralValue::Null => "null",
                LiteralValue::Undefined => "undefined",
                LiteralValue::BigInt(_) => "bigint",
            },
            Expr::Ident(_) => "any",
            Expr::Binary(_) => "any",
            Expr::Unary(_) => "any",
            Expr::Call(_) => "any",
            Expr::Member(_) => "any",
            Expr::Object(_) => "object",
            Expr::Array(_) => "array",
            Expr::Ternary(_) => "any",
            Expr::Lambda(_) => "function",
            Expr::Template(_) => "string",
            Expr::Await(_) => "promise",
            Expr::Spread(_) => "array",
        }
    }
}
