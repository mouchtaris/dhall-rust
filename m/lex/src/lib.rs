pub const VERSION: &str = "0.0.1";
use ast::Token;

pub type Span<N, T> = (N, T, N);

pub type Item<'s> = Span<usize, ast::Token<'s>>;

pub struct Lex<'s> {
    source: &'s str,
    last_span: Item<'s>,
    mode: u16,
    strstack: Vec<u16>,
}

impl<'s> Lex<'s> {
    pub fn new(source: &'s str) -> Self {
        Lex {
            source,
            last_span: (0, Token::Empty(""), 0),
            mode: 0,
            strstack: <_>::default(),
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

    pub fn next_expr(&mut self) -> R<'s> {
        let inp = self.src();

        parse_whitespace(inp)
            .or_else(|| parse_block_comment(inp))
            .or_else(|| parse_line_comment1(inp))
            .or_else(|| parse_line_comment2(inp))
            .or_else(|| parse_rel_uri(inp))
            .or_else(|| parse_http_uri(inp))
            .or_else(|| parse_sha256(inp))
            .or_else(|| parse_punctuation(inp))
            .or_else(|| parse_natural(inp))
            .or_else(|| parse_ident_or_keyword(inp))
    }

    pub fn next_moody(&mut self) -> R<'s> {
        log::trace!("Mode: {}", self.mode);
        match self.mode {
            n if n % 3 == 0 => {
                let tkn = self.next_expr();
                match &tkn {
                    Some(Token::DQuote(_)) => self.mode += 1,
                    Some(Token::DDQuote(_)) => self.mode += 4,
                    Some(Token::RBrace(_)) if n > 0 => {
                        self.mode -= self.strstack.pop().map(|x| x + 1).unwrap_or(0)
                    }
                    _ => (),
                }
                tkn
            }
            n if n % 3 == 1 => {
                let m = n % 6;
                self.mode += 1;
                match m {
                    1 => parse_dquot_raw_seg(self.src()),
                    4 => parse_ddquote_raw_seg(self.src()),
                    o => panic!("{:?}", o),
                }
            }
            n if n % 3 == 2 => {
                let m = n % 6;
                let tkn = self.next_expr();
                match tkn {
                    Some(Token::DQuote(_) | Token::DDQuote(_)) => {
                        self.mode -= m;
                    }
                    Some(Token::TextImbue(_)) => {
                        let s = 6 - m;
                        log::trace!("strstack.push({})", s);
                        self.strstack.push(s);
                        self.mode += s
                    }
                    _ => (),
                }
                tkn
            }
            o => panic!("{:?}", o),
        }
    }
}

impl<'s> Iterator for Lex<'s> {
    type Item = Item<'s>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut token = self
            .next_moody()
            .map(|t| (0, t, t.as_str().as_bytes().len()));

        token.as_mut().into_iter().for_each(|span| {
            span_shift(&self.last_span, span);
            self.last_span = span.to_owned();
        });

        match token {
            Some((_, Token::Whitespace(_) | Token::Comment(_), _)) => {
                log::trace!("tkn: {:?}", token);
                // skip comments
                self.next()
            }
            token => {
                log::debug!("tkn: {:?}", token);
                token
            }
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

fn scan_parse<F, T>(inp: &str, to_token: T, mut length_adjustmentor: F) -> R<'_>
where
    F: FnMut(char) -> Option<i32>,
    T: FnOnce(&str) -> Token,
{
    let len: i32 = inp.chars().scan((), |_, c| length_adjustmentor(c)).sum();

    let len = len as usize;
    let bytelen = inp
        .char_indices()
        .take(len)
        .last()
        .map(|(i, c)| ast::utf8len(c) + i)
        .unwrap_or(0);
    let txt = &inp[0..bytelen];
    Some(to_token(txt))
}

fn longer_than(n: usize) -> impl FnOnce(Token) -> R {
    move |t| match t {
        t if t.as_str().len() < n => None,
        t => Some(t),
    }
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
                || (i >= 2 && !c.is_whitespace() && c != ')' && c != ',')
        },
    )
    .and_then(longer_than(2))
}

fn parse_http_uri(inp: &str) -> R<'_> {
    range_parse(
        inp,
        |s| Token::HttpUri(s),
        |&(i, c)| {
            (i == 0 && c == 'h')
                || (i == 1 && c == 't')
                || (i == 2 && c == 't')
                || (i == 3 && c == 'p')
                || (i == 4 && (c == ':' || c == 's'))
                || (i == 5 && (c == '/' || c == ':'))
                || (i == 6 && (c == '/' || c == '/'))
                || (i >= 7 && !c.is_whitespace() && c != ')')
        },
    )
    .and_then(longer_than(7))
}

fn parse_sha256(inp: &str) -> R<'_> {
    range_parse(
        inp,
        |s| Token::Sha256(s),
        |&(i, c)| {
            (i == 0 && c == 's')
                || (i == 1 && c == 'h')
                || (i == 2 && c == 'a')
                || (i == 3 && c == '2')
                || (i == 4 && c == '5')
                || (i == 5 && c == '6')
                || (i == 6 && c == ':')
                || (i >= 7 && i < (7 + 64))
        },
    )
    .and_then(longer_than(7 + 63))
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
    .or_else(|| {
        scan_parse(inp, |s| Token::Ident(s), {
            let mut state = 0;
            move |c| {
                Some(match (state, c) {
                    (0, '`') => {
                        state += 1;
                        1
                    }
                    (1, '`') => {
                        state += 1;
                        1
                    }
                    (2, _) => return None,
                    _ => 1,
                })
            }
        })
        .and_then(longer_than(2))
    })
    .map(|mut tkn| {
        const STRTOKS: &[(&str, fn(&str) -> Token)] = &[
            ("let", |s| Token::Let(s)),
            ("in", |s| Token::In(s)),
            ("with", |s| Token::With(s)),
            ("if", |s| Token::If(s)),
            ("then", |s| Token::Then(s)),
            ("else", |s| Token::Else(s)),
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

fn parse_line_comment1(inp: &str) -> R<'_> {
    let mut state = 0;
    scan_parse(
        inp,
        |s| Token::Comment(s),
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
    .and_then(longer_than(2))
}

fn parse_line_comment2(inp: &str) -> R<'_> {
    let mut state = 0;
    scan_parse(
        inp,
        |s| Token::Comment(s),
        move |c| {
            match (state, c) {
                (0, '#') => state += 1,
                (1, '!') => state += 1,
                (2, '\n') => state += 1,
                (2, _) => (),
                _ => return None,
            };
            Some(1)
        },
    )
    .and_then(longer_than(2))
}

fn parse_block_comment(inp: &str) -> R<'_> {
    let mut p = '_';
    let mut done = false;
    let mut booted = false;
    scan_parse(
        inp,
        |s| Token::Comment(s),
        move |c| {
            if done {
                return None;
            }
            let n = match (booted, &p, c) {
                (false, _, '{') => 1,
                (false, '{', '-') => {
                    booted = true;
                    1
                }
                (true, '-', '}') => {
                    done = true;
                    1
                }
                (true, _, _) => 1,
                _ => {
                    done = true;
                    0
                }
            };
            p = c;
            Some(n)
        },
    )
    .and_then(longer_than(4))
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
                (_, '$', '{') => {
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

pub fn parse_ddquote_raw_seg(inp: &str) -> R {
    let mut p = '_';
    let mut q = '_';
    let mut r = '_';
    let mut done = false;
    scan_parse(
        inp,
        |s| Token::RawText(s),
        move |c| {
            if done {
                return None;
            }
            let n = match (&r, &q, &p, c) {
                ('\'', '\'', '$', '{') => 1,
                ('\'', '\'', _, _) => {
                    done = true;
                    -3
                }
                (_, _, '$', '{') => {
                    done = true;
                    -1
                }
                _ => 1,
            };
            r = q;
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
        .or_else(|| parse_verbatim(inp, "''", "", |s| DDQuote(s)))
        .or_else(|| parse_verbatim(inp, "\"", "", |s| DQuote(s)))
        .or_else(|| parse_verbatim(inp, "++", "", |s| TextConcat(s)))
        .or_else(|| parse_verbatim(inp, "${", "", |s| TextImbue(s)))
        .or_else(|| parse_verbatim(inp, "::", "", |s| DColon(s)))
        .or_else(|| parse_verbatim(inp, "''", "", |s| DDQuote(s)))
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
