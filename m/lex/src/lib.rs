pub const VERSION: &str = "0.0.1";
use ast::Token;

pub type Span<N, T> = (N, T, N);

pub type Item<'s> = Span<usize, ast::Token<'s>>;

pub struct Lex<'s> {
    source: &'s str,
    last_span: Item<'s>,
    mode: u8,
}

impl<'s> Lex<'s> {
    pub fn new(source: &'s str) -> Self {
        Lex {
            source,
            last_span: (0, Token::Empty(""), 0),
            mode: 0,
        }
    }

    pub fn src(&self) -> &'s str {
        let &Self {
            source,
            last_span: (_, _, s),
            ..
        } = self;
        &source[s..]
    }

    pub fn next_dquot_raw(&mut self) -> R<'s> {
        parse_dquot_raw_seg(self.src())
    }

    pub fn next_expr(&mut self) -> R<'s> {
        let inp = self.src();

        parse_whitespace(inp)
            .or_else(|| parse_line_comment(inp))
            .or_else(|| parse_rel_uri(inp))
            .or_else(|| parse_punctuation(inp))
            .or_else(|| parse_natural(inp))
            .or_else(|| parse_ident_or_keyword(inp))
    }

    pub fn next_moody(&mut self) -> R<'s> {
        let phase = self.mode % 5;
        match phase {
            0 | 2 => {
                let tkn = self.next_expr();
                match &tkn {
                    Some(Token::DQuote(_)) if phase == 0 => {
                        self.mode += 1;
                    }
                    Some(Token::DQuote(_)) if phase == 2 => {
                        self.mode -= 2;
                    }
                    Some(Token::TextImbue(_)) if phase == 2 => {
                        self.mode += 3;
                    }
                    Some(Token::RBrace(_)) if phase == 0 && self.mode > 0 => {
                        self.mode -= 4;
                    }
                    _ => (),
                }
                tkn
            }
            1 => {
                let tkn = self.next_dquot_raw();
                self.mode += 1;
                tkn
            }
            o => panic!("{:?}", o),
        }
    }
}

impl<'s> Iterator for Lex<'s> {
    type Item = Item<'s>;
    fn next(&mut self) -> Option<Self::Item> {
        log::trace!("mode = {}", self.mode);

        let mut token = self
            .next_moody()
            .map(|t| (0, t, t.as_str().as_bytes().len()));

        token.as_mut().into_iter().for_each(|span| {
            span_shift(&self.last_span, span);
            self.last_span = span.to_owned();
        });

        log::debug!("expr: {:?}", token);

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

type R<'s> = Option<Token<'s>>;

fn range_parse<T, P>(inp: &str, to_token: T, pred: P) -> R<'_>
where
    P: FnMut(&(usize, char)) -> bool,
    T: FnOnce(&str) -> Token,
{
    inp.char_indices()
        .take_while(pred)
        .last()
        .map(|(i, _)| to_token(&inp[0..=i]))
}

pub fn scan_parse<F, T>(inp: &str, to_token: T, mut length_adjustmentor: F) -> R<'_>
where
    F: FnMut(char) -> Option<i32>,
    T: FnOnce(&str) -> Token,
{
    let len: i32 = inp.chars().scan((), |_, c| length_adjustmentor(c)).sum();

    let len = len as usize;
    let txt = &inp[0..len];
    Some(to_token(txt))
}

fn parse_whitespace(inp: &str) -> R<'_> {
    range_parse(inp, |s| Token::Whitespace(s), |(_, c)| c.is_whitespace())
}

fn parse_natural(inp: &str) -> R<'_> {
    range_parse(inp, |s| Token::Natural(s), |(_, c)| c.is_ascii_digit())
}

fn parse_rel_uri(inp: &str) -> R<'_> {
    range_parse(
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
    range_parse(
        inp,
        |s| Token::Ident(s),
        |&(i, c)| {
            (i == 0 && (c.is_alphabetic() || c == '_'))
                || (i >= 1 && (c.is_alphanumeric() || c == '_' || c == '-' || c == '/'))
        },
    )
    .map(|mut tkn| {
        const STRTOKS: &[(&str, fn(&str) -> Token)] = &[
            ("let", |s| Token::Let(s)),
            ("in", |s| Token::In(s)),
            ("with", |s| Token::With(s)),
        ];

        for &(s, to_token) in STRTOKS {
            match &mut tkn {
                tkn if tkn.is_ident(s) => *tkn = to_token(s),
                _ => (),
            }
        }

        tkn
    })
}

fn parse_line_comment(inp: &str) -> R<'_> {
    let mut state = 0;
    scan_parse(
        inp,
        |s| Token::LineComment(s),
        move |c| {
            match (state, c) {
                (0, '-') => state += 1,
                (1, '-') => state += 1,
                (2, '\n') => state += 1,
                (2, _) => (),
                _ => return None,
            };
            Some(1)
        },
    )
    .and_then(|t| match t {
        t if t.as_str().len() < 2 => None,
        t => Some(t),
    })
}

pub fn parse_dquot_raw_seg(inp: &str) -> R {
    let mut p = '_';
    let mut q = '_';
    let mut done = false;
    scan_parse(
        inp,
        |s| Token::RawText(s),
        move |c| {
            if done {
                return None;
            }
            let n = match (&q, &p, c) {
                ('\\', '$', '{') | (_, '\\', _) => 1,
                (_, _, '"') => {
                    done = true;
                    0
                }
                (_, '$', '{') | (_, '\'', '\'') => {
                    done = true;
                    -1
                }
                _ => 1,
            };
            q = p;
            p = c;
            Some(n)
        },
    )
}

fn parse_verbatim<'i, T>(inp: &'i str, opt1: &str, opt2: &str, to_token: T) -> R<'i>
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
        .or_else(|| parse_verbatim(inp, "\"", "''", |s| DQuote(s)))
        .or_else(|| parse_verbatim(inp, "++", "", |s| TextConcat(s)))
        .or_else(|| parse_verbatim(inp, "${", "", |s| TextImbue(s)))
        .or_else(|| parse_verbatim(inp, "::", "", |s| DColon(s)))
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
}
