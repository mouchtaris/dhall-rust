pub const VERSION: &str = "0.0.1";

mod is_list;
pub use is_list::IsList;
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

#[derive(Clone, Debug)]
pub enum Expr<'i> {
    Term1(Term1<'i>),
    Let(Deq<LetStmt<'i>>, Val<'i>),
    Lambda(Ident<'i>, Option<Val<'i>>, Val<'i>),
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub enum Term<'i> {
    Integer(i32),
    Double(f32),
    FieldAccess(Box<Term<'i>>, Ident<'i>),
    Project(u8, Box<Term<'i>>, Deq<Term1<'i>>),
    Path(TermPath<'i>),
    Var(Ident<'i>, u16),
    Text(u8, Deq<TextEntry<'i>>),
    List(Deq<Val<'i>>),
    Record(RecordData<'i>),
    TypeRecord(RecordData<'i>),
    TypeEnum(TypeEnumData<'i>),
    Import {
        path: &'i str,
        as_: Option<&'i str>,
        guard: Option<&'i str>,
        fall: Option<(&'i str, Option<&'i str>)>,
    },
    Expr(Val<'i>),
    Merge(RecordData<'i>, Box<Term<'i>>),
    Embed(String),
}

#[derive(Copy, Clone, Debug)]
pub enum Token<'i> {
    Ident(&'i str),
    Natural(&'i str),
    Negative(&'i str),
    Double(&'i str),
    Text(&'i str),
    RelUri(&'i str),
    HttpUri(&'i str),
    Missing(&'i str),
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
    Equiv(&'i str),
    LogicConj(&'i str),
    LogicDisj(&'i str),
    LogicEq(&'i str),
    LogicNeq(&'i str),
    Scope(&'i str),
    As(&'i str),
}

impl<'s> AsRef<str> for Token<'s> {
    fn as_ref(&self) -> &str {
        use Token::*;
        match self {
            As(s) | Missing(s) | LogicNeq(s) | LogicEq(s) | Natural(s) | Scope(s)
            | LogicConj(s) | LogicDisj(s) | Equiv(s) | Double(s) | Merge(s) | DDQuote(s)
            | DColon(s) | RawText(s) | Ident(s) | Negative(s) | Text(s) | RelUri(s)
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

    pub fn is_keyword(&self) -> bool {
        match self {
            Token::Let(_)
            | Token::In(_)
            | Token::With(_)
            | Token::If(_)
            | Token::Then(_)
            | Token::Else(_)
            | Token::Merge(_)
            | Token::Missing(_)
            | Token::As(_) => true,
            _ => false,
        }
    }

    pub fn set_val<'u>(&self, val: &'u str) -> Token<'u> {
        use Token::*;
        match self {
            Ident(_) => Ident(val),
            Natural(_) => Natural(val),
            Negative(_) => Negative(val),
            Double(_) => Double(val),
            Text(_) => Text(val),
            RelUri(_) => RelUri(val),
            HttpUri(_) => HttpUri(val),
            Missing(_) => Missing(val),
            Sha256(_) => Sha256(val),
            Conj1(_) => Conj1(val),
            Conj2(_) => Conj2(val),
            Alt(_) => Alt(val),
            Lambda(_) => Lambda(val),
            Arrow(_) => Arrow(val),
            Equals(_) => Equals(val),
            Let(_) => Let(val),
            In(_) => In(val),
            LPar(_) => LPar(val),
            RPar(_) => RPar(val),
            Colon(_) => Colon(val),
            DColon(_) => DColon(val),
            Forall(_) => Forall(val),
            TextConcat(_) => TextConcat(val),
            ListConcat(_) => ListConcat(val),
            Plus(_) => Plus(val),
            Div(_) => Div(val),
            Star(_) => Star(val),
            Minus(_) => Minus(val),
            LBrace(_) => LBrace(val),
            RBrace(_) => RBrace(val),
            LBracket(_) => LBracket(val),
            RBracket(_) => RBracket(val),
            LAngle(_) => LAngle(val),
            RAngle(_) => RAngle(val),
            Comma(_) => Comma(val),
            Dot(_) => Dot(val),
            Pipe(_) => Pipe(val),
            DDQuote(_) => DDQuote(val),
            DQuote(_) => DQuote(val),
            SQuote(_) => SQuote(val),
            Questionmark(_) => Questionmark(val),
            If(_) => If(val),
            Then(_) => Then(val),
            Else(_) => Else(val),
            TextImbue(_) => TextImbue(val),
            With(_) => With(val),
            Comment(_) => Comment(val),
            Empty(_) => Empty(val),
            Whitespace(_) => Whitespace(val),
            RawText(_) => RawText(val),
            Merge(_) => Merge(val),
            Equiv(_) => Equiv(val),
            LogicConj(_) => LogicConj(val),
            LogicDisj(_) => LogicDisj(val),
            LogicEq(_) => LogicEq(val),
            LogicNeq(_) => LogicNeq(val),
            Scope(_) => Scope(val),
            As(_) => As(val),
        }
    }
}

pub fn deq<T>(t: T) -> Deq<T> {
    let mut d = Deq::new();
    d.push_front(t);
    d
}

pub fn path<'i, P>(p: P) -> Path<'i>
where
    P: IntoIterator,
    <P as IntoIterator>::Item: ToOwned<Owned = &'i str>,
{
    p.into_iter().map(|s| s.to_owned()).collect()
}

pub fn utf8len(c: char) -> usize {
    let mut buf = [0u8; 4];
    c.encode_utf8(&mut buf).as_bytes().len()
}

pub fn const_0_term<'i>() -> Term<'i> {
    Term::Integer(0)
}
pub fn const_0_term1<'i>() -> Term1<'i> {
    Term1::Term(const_0_term())
}

pub fn const_0_expr<'i>() -> Expr<'i> {
    Expr::Term1(const_0_term1())
}

pub fn var_expr(s: &str) -> Expr {
    let s = match s {
        "Type" => "`Type`",
        "Kind" => "`Kind`",
        "Sort" => "`Sort`",
        s => s,
    };
    Expr::Term1(Term1::Term(Term::Var(s, 0)))
}

impl<'i> Default for Term<'i> {
    fn default() -> Self {
        const_0_term()
    }
}
impl<'i> Default for Term1<'i> {
    fn default() -> Self {
        Self::Term(<_>::default())
    }
}
impl<'i> Default for Expr<'i> {
    fn default() -> Self {
        Self::Term1(<_>::default())
    }
}

impl<'i> Into<Expr<'i>> for Term<'i> {
    fn into(self) -> Expr<'i> {
        <Term1<'i> as From<Term<'i>>>::from(self).into()
    }
}

impl<'i> From<Term1<'i>> for Expr<'i> {
    fn from(t1: Term1) -> Expr {
        Expr::Term1(t1)
    }
}

impl<'i> From<Term<'i>> for Term1<'i> {
    fn from(t: Term) -> Term1 {
        Term1::Term(t)
    }
}
