use logos::Logos;

pub type LexerResult<'a> = std::result::Result<TokenWithSpan<'a>, LexerError>;
pub type TokenWithSpan<'a> = (usize, Token<'a>, usize);
pub type LexerError = ();

pub struct Lexer<'a> {
    lexer: logos::SpannedIter<'a, Token<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Lexer {
            lexer: Token::lexer(src).spanned(),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = LexerResult<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let (token, span) = self.lexer.next()?;

        if matches!(token, Token::Invalid) {
            // Some(Err(()))
            todo!();
        } else {
            Some(Ok((span.start, token, span.end)))
        }
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
    #[token("minicio.h")]
    MinicioHeader,
    #[token("return")]
    Return,
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
