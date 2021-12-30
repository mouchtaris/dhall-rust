use super::{Deq, Expr, Term, Term1, Val};

pub trait IsList<'i> {
    fn is_list(&self) -> bool;
    fn get_list_mut(&mut self) -> &mut Deq<Val<'i>>;
}

impl<'i> IsList<'i> for Expr<'i> {
    fn is_list(&self) -> bool {
        match self {
            Expr::Term1(t) => t.is_list(),
            _ => false,
        }
    }
    fn get_list_mut(&mut self) -> &mut Deq<Val<'i>> {
        match self {
            Expr::Term1(t) => t.get_list_mut(),
            _ => panic!(),
        }
    }
}

impl<'i> IsList<'i> for Term1<'i> {
    fn is_list(&self) -> bool {
        match self {
            Term1::Term(t) => t.is_list(),
            _ => false,
        }
    }
    fn get_list_mut(&mut self) -> &mut Deq<Val<'i>> {
        match self {
            Term1::Term(t) => t.get_list_mut(),
            _ => panic!(),
        }
    }
}

impl<'i> IsList<'i> for Term<'i> {
    fn is_list(&self) -> bool {
        match self {
            Term::List(_) => true,
            _ => false,
        }
    }
    fn get_list_mut(&mut self) -> &mut Deq<Val<'i>> {
        match self {
            Term::List(l) => l,
            _ => panic!(),
        }
    }
}

impl<'a, 'i, T> IsList<'i> for &'a mut T
where
    T: IsList<'i>,
{
    fn is_list(&self) -> bool {
        T::is_list(&*self)
    }
    fn get_list_mut(&mut self) -> &mut Deq<Val<'i>> {
        T::get_list_mut(*self)
    }
}

impl<'i, T> IsList<'i> for Box<T>
where
    T: IsList<'i>,
{
    fn is_list(&self) -> bool {
        T::is_list(self.as_ref())
    }
    fn get_list_mut(&mut self) -> &mut Deq<Val<'i>> {
        T::get_list_mut(self.as_mut())
    }
}
