pub const VERSION: &str = "0.0.1";

pub use std::collections::VecDeque as Deq;

pub type Ident<'i> = &'i str;
pub type Path<'i> = Deq<Ident<'i>>;

pub type RecField<'i> = (Path<'i>, Val<'i>);

pub type Val<'i> = Box<Expr<'i>>;

#[derive(Debug)]
pub enum Expr<'i> {
    Term1(Term1<'i>),
}

#[derive(Debug)]
pub enum Term1<'i> {
    With(Box<Term1<'i>>, Path<'i>, Term<'i>),
    Substitution(Box<Term1<'i>>, Term<'i>),
    Term(Term<'i>),
}

#[derive(Debug)]
pub enum Term<'i> {
    Natural(&'i str),
    Path(Path<'i>),
}
