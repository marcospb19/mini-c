use logos::Logos;

pub type LexerResult<'a> = std::result::Result<TokenWithSpan<'a>, LexerError>;
pub type TokenWithSpan<'a> = (usize, Token<'a>, usize);
pub type LexerError = ();

pub struct Lexer<'a> {
    lex: logos::SpannedIter<'a, Token<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Lexer {
            lex: Token::lexer(src).spanned(),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = LexerResult<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let (token, span) = self.lex.next()?;

        if matches!(token, Token::Invalid) {
            todo!();
        } else {
            Some(Ok((span.start, token, span.end)))
        }
    }
}

#[derive(Logos, Debug, Clone, Copy)]
pub enum Token<'input> {
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
