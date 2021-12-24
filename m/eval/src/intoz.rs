/// A private method of handling "Into".
///
/// This is tuned to AST elements (and their containers) turning themselves
/// into Expr.
///
pub trait Intoz<T> {
    /// Turn self into T.
    fn intoz(self) -> T;
}

impl<T, U> Intoz<T> for U
where
    Self: Stealr<T>,
    T: Default,
{
    fn intoz(mut self) -> T {
        self.stealr_give()
    }
}

pub trait Stealr<T> {
    fn stealr_take(&mut self, t: &mut T);

    fn stealr_give(&mut self) -> T
    where
        T: Default,
    {
        let mut t = <_>::default();
        self.stealr_take(&mut t);
        t
    }
}

impl<A, T> Stealr<Option<T>> for Option<A>
where
    A: Stealr<T>,
    T: Default,
{
    fn stealr_take(&mut self, t: &mut Option<T>) {
        match (self, t) {
            (Some(a), Some(t)) => a.stealr_take(t),
            _ => (),
        }
    }
}

impl<A, T> Stealr<T> for Box<A>
where
    A: Stealr<T>,
{
    fn stealr_take(&mut self, t: &mut T) {
        self.as_mut().stealr_take(t)
    }
}

impl<'a, A, T> Stealr<T> for &'a mut A
where
    A: Stealr<T>,
{
    fn stealr_take(&mut self, t: &mut T) {
        A::stealr_take(*self, t)
    }
}

impl<'i> Stealr<ast::Expr<'i>> for ast::Expr<'i> {
    fn stealr_take(&mut self, t: &mut ast::Expr<'i>) {
        mem::swap(self, t)
    }
}

use std::mem;
