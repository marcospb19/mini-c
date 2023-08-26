use logos::Logos;

pub type LexerResult<'a> = std::result::Result<TokenWithSpan<'a>, LexerError>;
pub type TokenWithSpan<'a> = (usize, Token<'a>, usize);
pub type LexerError = MiniCParseError;

// This shouldn't be here.
#[derive(Debug, Clone)]
pub enum MiniCParseError {
    // This is, arguably, not a parser error lol
    InvalidToken(String, std::ops::Range<usize>),
    InvalidType(String),
    InvalidInteger(String, std::num::ParseIntError),
}

pub struct Lexer<'a> {
    lexer: logos::SpannedIter<'a, Token<'a>>,
    source: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            lexer: Token::lexer(source).spanned(),
            source,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = LexerResult<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lexer.next().map(|(token, span)| {
            if let Token::Invalid = token {
                Err(MiniCParseError::InvalidToken(
                    self.source[span.clone()].to_owned(),
                    span,
                ))
            } else {
                Ok((span.start, token, span.end))
            }
        })
    }
}

#[derive(Logos, Debug, Clone, Copy)]
pub enum Token<'input> {
    #[regex(r"[a-zA-Z][a-zA-Z0-9_]*")]
    Ident(&'input str),
    #[regex(r"[0-9][0-9_]*")]
    Int(&'input str),

    #[token("#include")]
    IncludeMinicio,
    #[token(r#""minicio.h""#)]
    MinicioHeader,
    #[token("return")]
    Return,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("for")]
    For,
    #[token("while")]
    While,
    #[token("break")]
    Break,
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
    OpenBraces,
    #[token("}")]
    CloseBraces,
    #[token("[")]
    OpenBrackets,
    #[token("]")]
    CloseBrackets,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Modulo,
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token("!")]
    Not,
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
