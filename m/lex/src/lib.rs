pub const VERSION: &str = "0.0.1";
use ast::Token;

pub type Span<N, T> = (N, T, N);

pub type Item<'s> = Span<usize, ast::Token<'s>>;

pub struct Lex<'s> {
    source: &'s str,
    last_span: Item<'s>,
}

impl<'s> Lex<'s> {
    pub fn new(source: &'s str) -> Self {
        Lex {
            source,
            last_span: (0, Token::Empty(""), 0),
        }
    }
}

impl<'s> Lex<'s> {
    pub fn input(&'s self) -> &str {
        let Self { source, last_span } = self;
        &source[last_span.2..]
    }
}

impl<'s> Iterator for &'s mut Lex<'s> {
    type Item = Item<'s>;
    fn next(&mut self) -> Option<Self::Item> {
        let inp = &self.source[self.last_span.2..];

        let mut token = parse_whitespace(inp)
            .or_else(|| parse_line_comment(inp))
            .or_else(|| parse_rel_uri(inp))
            .or_else(|| parse_punctuation(inp))
            .or_else(|| parse_natural(inp))
            .or_else(|| parse_ident_or_keyword(inp));

        match &mut token {
            Some(span) => {
                span_shift(&self.last_span, span);
                self.last_span = *span;
            }
            _ => (),
        }

        log::debug!("token: {:?}", token);

        match token {
            Some((_, Token::Whitespace(_) | Token::LineComment(_), _)) => {
                // skip comments
                self.next()
            }
            token => token,
        }
    }
}

fn span_shift<N, T>(&(_, _, base): &Span<N, T>, (s, _, e): &mut Span<N, T>)
where
    N: std::ops::AddAssign + Copy,
    T: Copy,
{
    *s += base;
    *e += base;
}

type R<'s> = Option<Item<'s>>;

fn parse_range<T, P>(inp: &str, to_token: T, pred: P) -> R<'_>
where
    T: FnOnce(&str) -> Token,
    P: FnMut(&(usize, char)) -> bool,
{
    inp.char_indices()
        .take_while(pred)
        .last()
        .map(|(i, c)| (0, to_token(&inp[0..=i]), i + ast::utf8len(c)))
}

fn parse_whitespace(inp: &str) -> R<'_> {
    parse_range(inp, |s| Token::Whitespace(s), |(_, c)| c.is_whitespace())
}

fn parse_line_comment(inp: &str) -> R<'_> {
    parse_range(
        inp,
        |s| Token::LineComment(s),
        |&(i, c)| (i == 0 && c == '-') || (i == 1 && c == '-') || (i >= 2 && c != '\n'),
    )
    .map(|mut span| {
        span.2 += 1; // add '\n' to span
        span
    })
    .and_then(|r| if inp.starts_with("--") { Some(r) } else { None })
}

fn parse_natural(inp: &str) -> R<'_> {
    parse_range(inp, |s| Token::Natural(s), |(_, c)| c.is_ascii_digit())
}

fn parse_rel_uri(inp: &str) -> R<'_> {
    parse_range(
        inp,
        |s| Token::RelUri(s),
        |&(i, c)| {
            (i == 0 && c == '.')
                || (i == 1 && (c == '.' || c == '/'))
                || (i >= 2 && !c.is_whitespace())
        },
    )
}

fn parse_ident_or_keyword(inp: &str) -> R<'_> {
    parse_range(
        inp,
        |s| Token::Ident(s),
        |&(i, c)| {
            (i == 0 && (c.is_alphabetic() || c == '_'))
                || (i >= 1 && (c.is_alphanumeric() || c == '_' || c == '-' || c == '/'))
        },
    )
    .map(|mut span| {
        let strtoks: &[(&str, fn(&str) -> Token)] = &[
            ("let", |s| Token::Let(s)),
            ("in", |s| Token::In(s)),
            ("with", |s| Token::With(s)),
        ];

        let tkn = &mut span.1;

        for (s, to_token) in strtoks {
            match tkn {
                Token::Ident(v) if s == v => *tkn = to_token(s),
                _ => (),
            }
        }
        span
    })
}

fn parse_verbatim<'i, T>(inp: &'i str, opt1: &str, opt2: &str, to_token: T) -> Option<Token<'i>>
where
    T: FnOnce(&str) -> Token,
{
    let tkn = match inp {
        s if s.starts_with(opt1) => opt1,
        s if opt2.len() > 0 && s.starts_with(opt2) => opt2,
        _ => return None,
    };

    let len = tkn.as_bytes().len();
    let tkn = &inp[0..len];
    Some(to_token(tkn))
}

fn parse_punctuation(inp: &str) -> R<'_> {
    use Token::*;
    parse_verbatim(inp, "⩓", "//\\\\", |s| Conj1(s))
        .or_else(|| parse_verbatim(inp, "∧", "/\\", |s| Conj2(s)))
        .or_else(|| parse_verbatim(inp, "⫽", "//", |s| Alt(s)))
        .or_else(|| parse_verbatim(inp, "→", "->", |s| Arrow(s)))
        .or_else(|| parse_verbatim(inp, "λ", "\\", |s| Lambda(s)))
        .or_else(|| parse_verbatim(inp, "∀", "", |s| Forall(s)))
        .or_else(|| parse_verbatim(inp, "\"", "", |s| DQuote(s)))
        .or_else(|| parse_verbatim(inp, "++", "", |s| TextConcat(s)))
        .or_else(|| parse_verbatim(inp, "=", "", |s| Equals(s)))
        .or_else(|| parse_verbatim(inp, "(", "", |s| LPar(s)))
        .or_else(|| parse_verbatim(inp, ")", "", |s| RPar(s)))
        .or_else(|| parse_verbatim(inp, ":", "", |s| Colon(s)))
        .or_else(|| parse_verbatim(inp, "#", "", |s| ListConcat(s)))
        .or_else(|| parse_verbatim(inp, "+", "", |s| Plus(s)))
        .or_else(|| parse_verbatim(inp, "/", "", |s| Div(s)))
        .or_else(|| parse_verbatim(inp, "*", "", |s| Star(s)))
        .or_else(|| parse_verbatim(inp, "-", "", |s| Minus(s)))
        .or_else(|| parse_verbatim(inp, "{", "", |s| LBrace(s)))
        .or_else(|| parse_verbatim(inp, "}", "", |s| RBrace(s)))
        .or_else(|| parse_verbatim(inp, "[", "", |s| LBracket(s)))
        .or_else(|| parse_verbatim(inp, "]", "", |s| RBracket(s)))
        .or_else(|| parse_verbatim(inp, "<", "", |s| LAngle(s)))
        .or_else(|| parse_verbatim(inp, ">", "", |s| RAngle(s)))
        .or_else(|| parse_verbatim(inp, ",", "", |s| Comma(s)))
        .or_else(|| parse_verbatim(inp, ".", "", |s| Dot(s)))
        .or_else(|| parse_verbatim(inp, "|", "", |s| Pipe(s)))
        .or_else(|| parse_verbatim(inp, "'", "", |s| SQuote(s)))
        .or_else(|| parse_verbatim(inp, "?", "", |s| Questionmark(s)))
        .map(|t| (0, t, t.as_str().as_bytes().len()))
}
