pub type Program = Vec<Stmt>;
pub enum Stmt {
    LetStmt(Ident, Expr),
    ReturnStmt(Option<Expr>),
    ExprStmt(Expr),
    BlockStmt(Vec<Stmt>),
    IfStmt(Expr, Box<Stmt>, Option<Box<Stmt>>),
    WhileStmt(Expr, Box<Stmt>),
    FnStmt(Ident, Vec<Expr>, Box<Stmt>),
}

pub enum Expr {
    IdentExpr(Ident),
    LiteralExpr(Literal),
    AssignmentExpr(Ident, Box<Expr>),
    UnaryExpr(UnaryOp, Box<Expr>),
    BinExpr(Box<Expr>, BinOp, Box<Expr>),
    CallExpr(Box<Expr>, Vec<Expr>),
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Literal {
    IntLiteral(i64),
    BoolLiteral(bool),
    StringLiteral(String),
}
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Ident(pub String);
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum UnaryOp {
    UnaryPlus,
    UnaryMinus,
    Not,
}
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum BinOp {
    Plus,
    Minus,
    Divide,
    Multiply,

    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Equal,
    NotEqual,
}