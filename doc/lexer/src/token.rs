use logos::Logos;
use std::fmt; // to implement the Display trait

#[derive(Logos, Clone, Debug, PartialEq)]
pub enum Token {
    #[token("var")]
    KeywordVar,
    #[token("print")]
    KeywordPrint,

    #[regex("[_a-zA-Z][_0-9a-zA-Z]*", |lex| lex.slice().parse())]
    Identifier(String),
    #[regex(r"\d+", |lex| lex.slice().parse())]
    Integer(i64),

    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("=")]
    Assign,
    #[token(";")]
    Semicolon,

    #[token("+")]
    OperatorAdd,
    #[token("-")]
    OperatorSub,
    #[token("*")]
    OperatorMul,
    #[token("/")]
    OperatorDiv,

    #[regex(r"#.*\n?", logos::skip)]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[error]
    Error,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
