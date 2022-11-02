#[derive(logos::Logos, Debug, Clone, Copy)]
enum Token<'input> {
    #[regex(r"[_a-zA-Z][a-zA-Z0-9_]*")]
    Ident(&'input str),
    #[regex(r"[0-9][0-9_]*")]
    Number(&'input str),

    #[token("let")]
    Let,
    #[token("fn")]
    Fn,
    #[token("return")]
    Return,
    #[token("not")]
    Not,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token(";")]
    SemiColon,
    #[token("=")]
    SingleEquals,
    #[token(",")]
    Comma,
    #[token("(")]
    OpenParenthesis,
    #[token(")")]
    CloseParenthesis,
    #[token("{")]
    OpenBracket,
    #[token("}")]
    CloseBracket,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("<")]
    Less,
    #[token("<=")]
    LessOrEqual,
    #[token("==")]
    DoubleEquals,
    #[token("!=")]
    NotEquals,
    #[token(">")]
    Greater,
    #[token(">=")]
    GreaterOrEqual,

    #[error]
    #[regex(r"//.*\n", logos::skip)]
    #[regex(r"\s+", logos::skip)]
    Invalid,
}
