pub const VERSION: &str = "0.0.1";

pub use std::collections::VecDeque as Deq;

pub type Ident<'i> = &'i str;
pub type Path<'i> = Deq<Ident<'i>>;
pub type TermPath<'i> = Deq<Box<Term<'i>>>;

pub type RecField<'i> = (Path<'i>, Val<'i>);

pub type Val<'i> = Box<Expr<'i>>;

pub type LetStmt<'i> = (Ident<'i>, Option<Val<'i>>, Val<'i>);

pub type TextEntry<'i> = (&'i str, Option<Val<'i>>);

pub type RecordEntry<'i> = (Path<'i>, Val<'i>);
pub type RecordData<'i> = Deq<RecordEntry<'i>>;

pub type TypeEnumEntry<'i> = (Ident<'i>, Option<Val<'i>>);
pub type TypeEnumData<'i> = Deq<TypeEnumEntry<'i>>;

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
    Arrow(Option<Ident<'i>>, Val<'i>, Val<'i>),
    With(Box<Term1<'i>>, Path<'i>, Box<Term1<'i>>),
    Operation(Box<Term1<'i>>, &'i str, Box<Term1<'i>>),
    IfThenElse(Val<'i>, Val<'i>, Val<'i>),
    Ascribe(Box<Term1<'i>>, Val<'i>),
    Construct(Box<Term1<'i>>, RecordData<'i>),
}

#[derive(Debug)]
pub enum Term<'i> {
    Integer(bool, &'i str),
    FieldAccess(Box<Term<'i>>, Ident<'i>),
    Project(u8, Box<Term<'i>>, Deq<Ident<'i>>),
    Path(TermPath<'i>),
    Var(Ident<'i>),
    Text(u8, Deq<TextEntry<'i>>),
    List(Deq<Val<'i>>),
    Record(RecordData<'i>),
    TypeRecord(RecordData<'i>),
    TypeEnum(TypeEnumData<'i>),
    Import(&'i str, Option<(&'i str, Option<&'i str>)>),
    Expr(Val<'i>),
    Merge(RecordData<'i>, Box<Term<'i>>),
}

#[derive(Debug, Clone, Copy)]
pub enum Token<'i> {
    Ident(&'i str),
    Integer(&'i str),
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
    Merge(&'i str),
}

impl<'s> AsRef<str> for Token<'s> {
    fn as_ref(&self) -> &str {
        use Token::*;
        match self {
            Merge(s) | DDQuote(s) | DColon(s) | RawText(s) | Ident(s) | Integer(s) | Text(s)
            | RelUri(s) | HttpUri(s) | Sha256(s) | Conj1(s) | Conj2(s) | Alt(s) | Lambda(s)
            | Arrow(s) | Equals(s) | Let(s) | In(s) | LPar(s) | RPar(s) | Colon(s) | Forall(s)
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

pub fn utf8len(c: char) -> usize {
    let mut buf = [0u8; 4];
    c.encode_utf8(&mut buf).as_bytes().len()
}

pub fn const_0_term<'i>() -> Term<'i> {
    Term::Integer(false, "0")
}
pub fn const_0_term1<'i>() -> Term1<'i> {
    Term1::Term(const_0_term())
}

pub fn const_0_expr<'i>() -> Expr<'i> {
    Expr::Term1(const_0_term1())
}

pub fn var_expr(s: &str) -> Expr {
    Expr::Term1(Term1::Term(Term::Var(s)))
}
