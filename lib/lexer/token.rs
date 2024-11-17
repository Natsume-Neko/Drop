
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Token {
    Illegal,
    EOF,

    Ident(String),
    IntLiteral(i64),
    BooleanLiteral(bool),

    Assign,
    Plus,
    Minus,
    Divide,
    Multiply,
    Not,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Equal,
    NotEqual,

    Comma,
    SemiColon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    Function,
    Let,
    If,
    Else,
    Return,
}