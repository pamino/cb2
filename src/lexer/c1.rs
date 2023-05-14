use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum C1Token {
    #[regex(r"[ \n\r\t]", |_| logos::Skip)]
    Whitespace,
    #[regex(r"/\*[^(\*/)]*\*/", |_| logos::Skip)]
    CComment,
    #[regex(r"//.*", |_| logos::Skip)]
    CppComment,
    #[error]
    Error,

// keywords
    #[token("bool")]
    KwBoolean,
    #[token("do")]
    KwDo,
    #[token("else")]
    KwElse,
    #[token("float")]
    KwFloat,
    #[token("for")]
    KwFor,
    #[token("if")]
    KwIf,
    #[token("int")]
    KwInt,
    #[token("printf")]
    KwPrintf,
    #[token("return")]
    KwReturn,
    #[token("void")]
    KwVoid,
    #[token("while")]
    KwWhile,

// operators
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Asterisk,
    #[token("/")]
    Slash,
    #[token("=")]
    Assign,
    #[token("==")]
    Eq,
    #[token("!=")]
    Neq,
    #[token("<")]
    Lss,
    #[token(">")]
    Grt,
    #[token("<=")]
    Leq,
    #[token(">=")]
    Geq,
    #[token("&&")]
    And,
    #[token("||")]
    Or,

// other tokens
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,

// term variables
    #[regex(r"[0-9]+")]
    ConstInt,
    #[regex(r"([0-9]+\.[0-9]+|\.[0-9]+)([eE]([-+])?[0-9]+)?|[0-9]+[eE]([-+])?[0-9]+")]
    ConstFloat,
    #[regex(r"(true|false)")]
    ConstBoolean,
    #[regex(r#""([^"\\]|\\.)*""#)]
    ConstString,
    #[regex(r"([a-zA-Z])+([0-9]|[a-zA-Z])*")]
    Id,
}
