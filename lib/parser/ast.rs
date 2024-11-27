pub type Program = Vec<Stmt>;
pub enum Stmt {
    LetStmt(Ident, Expr),
    ReturnStmt(Expr),
    ExprStmt(Expr)
}

pub enum Expr {
    IdentExpr(Ident),
    LiteralExpr(Literal),
    PrefixExpr(Prefix, Box<Expr>),
    InfixExpr(Box<Expr>, Infix, Box<Expr>),
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
pub enum Prefix {
    PrefixPlus,
    PrefixMinus,
    Not,
}
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Infix {
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