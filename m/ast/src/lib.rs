pub const VERSION: &str = "0.0.1";

pub use std::collections::VecDeque as Deq;

pub type Ident<'i> = &'i str;
pub type Path<'i> = Deq<Ident<'i>>;
pub type TermPath<'i> = Deq<Box<Term<'i>>>;

pub type RecField<'i> = (Path<'i>, Val<'i>);

pub type Val<'i> = Box<Expr<'i>>;

pub type LetStmt<'i> = (Ident<'i>, Option<Val<'i>>, Val<'i>);

pub type TextEntry<'i> = (&'i str, Option<Val<'i>>);

#[derive(Debug)]
pub enum Expr<'i> {
    Term1(Term1<'i>),
    Let(Deq<LetStmt<'i>>, Val<'i>),
    Lambda(Ident<'i>, Option<Val<'i>>, Val<'i>),
}

#[derive(Debug)]
pub enum Term1<'i> {
    Term(Term<'i>),
    Evaluation(Box<Term1<'i>>, Term<'i>),
    Arrow(Option<Ident<'i>>, Box<Term1<'i>>, Box<Term1<'i>>),
    With(Box<Term1<'i>>, Path<'i>, Term<'i>),
    Operation(Box<Term1<'i>>, &'i str, Term<'i>),
    IfThenElse(Val<'i>, Val<'i>, Val<'i>),
    Ascribe(Box<Term1<'i>>, Val<'i>),
}

#[derive(Debug)]
pub enum Term<'i> {
    Natural(&'i str),
    Negative(&'i str),
    Path(TermPath<'i>),
    Var(Ident<'i>),
    Text(Deq<TextEntry<'i>>),
    List(Deq<Val<'i>>),
    Record(Deq<(Path<'i>, Val<'i>)>),
    TypeRecord(Deq<(Path<'i>, Val<'i>)>),
    TypeEnum(Deq<(Ident<'i>, Option<Val<'i>>)>),
    Expr(Val<'i>),
    Import(&'i str, Option<(&'i str, Option<&'i str>)>),
}

#[derive(Debug, Clone, Copy)]
pub enum Token<'i> {
    Ident(&'i str),
    Natural(&'i str),
    Text(&'i str),
    RelUri(&'i str),
    HttpUri(&'i str),
    Sha256(&'i str),
    Conj1(&'i str),
    Conj2(&'i str),
    Alt(&'i str),
    Lambda(&'i str),
    Arrow(&'i str),
    Equals(&'i str),
    Let(&'i str),
    In(&'i str),
    LPar(&'i str),
    RPar(&'i str),
    Colon(&'i str),
    DColon(&'i str),
    Forall(&'i str),
    TextConcat(&'i str),
    ListConcat(&'i str),
    Plus(&'i str),
    Div(&'i str),
    Star(&'i str),
    Minus(&'i str),
    LBrace(&'i str),
    RBrace(&'i str),
    LBracket(&'i str),
    RBracket(&'i str),
    LAngle(&'i str),
    RAngle(&'i str),
    Comma(&'i str),
    Dot(&'i str),
    Pipe(&'i str),
    DDQuote(&'i str),
    DQuote(&'i str),
    SQuote(&'i str),
    Questionmark(&'i str),
    If(&'i str),
    Then(&'i str),
    Else(&'i str),
    TextImbue(&'i str),
    With(&'i str),
    Comment(&'i str),
    Empty(&'i str),
    Whitespace(&'i str),
    RawText(&'i str),
}

impl<'s> AsRef<str> for Token<'s> {
    fn as_ref(&self) -> &str {
        use Token::*;
        match self {
            DDQuote(s) | DColon(s) | RawText(s) | Ident(s) | Natural(s) | Text(s) | RelUri(s)
            | HttpUri(s) | Sha256(s) | Conj1(s) | Conj2(s) | Alt(s) | Lambda(s) | Arrow(s)
            | Equals(s) | Let(s) | In(s) | LPar(s) | RPar(s) | Colon(s) | Forall(s)
            | TextConcat(s) | ListConcat(s) | Plus(s) | Div(s) | Star(s) | Minus(s) | LBrace(s)
            | RBrace(s) | LBracket(s) | RBracket(s) | LAngle(s) | RAngle(s) | Comma(s) | Dot(s)
            | Pipe(s) | DQuote(s) | SQuote(s) | Questionmark(s) | If(s) | Then(s) | Else(s)
            | TextImbue(s) | With(s) | Comment(s) | Empty(s) | Whitespace(s) => s,
        }
    }
}

impl<'s> Token<'s> {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }

    pub fn is_ident<I: ?Sized>(&self, name: &I) -> bool
    where
        I: std::cmp::PartialEq<str>,
    {
        match self {
            &Token::Ident(v) if name == v => true,
            _ => false,
        }
    }
}

pub fn deq<T>(t: T) -> Deq<T> {
    let mut d = Deq::new();
    d.push_front(t);
    d
}

pub fn path_expr(path: Path) -> Expr<'_> {
    Expr::Term1(Term1::Term(Term::Var(path.front().unwrap())))
}

pub fn utf8len(c: char) -> usize {
    let mut buf = [0u8; 4];
    c.encode_utf8(&mut buf).as_bytes().len()
}

pub fn const_0_term<'i>() -> Term<'i> {
    Term::Natural("0")
}
pub fn const_0_term1<'i>() -> Term1<'i> {
    Term1::Term(const_0_term())
}

pub fn const_0_expr<'i>() -> Expr<'i> {
    Expr::Term1(const_0_term1())
}

pub fn obj_construct<'i>() -> Expr<'i> {
    // TODO
    const_0_expr()
}
