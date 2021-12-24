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
        self.steal()
    }
}

pub trait Stealr<T: Default> {
    fn steal(&mut self) -> T;
}

impl<A, T> Stealr<Option<T>> for Option<A>
where
    A: Stealr<T>,
    T: Default,
{
    fn steal(&mut self) -> Option<T> {
        self.as_mut().map(<_>::steal)
    }
}

impl<A, T> Stealr<T> for Box<A>
where
    A: Stealr<T>,
    T: Default,
{
    fn steal(&mut self) -> T {
        self.as_mut().steal()
    }
}

impl<'a, A, T> Stealr<T> for &'a mut A
where
    A: Stealr<T>,
    T: Default,
{
    fn steal(&mut self) -> T {
        A::steal(*self)
    }
}

impl<'i> Stealr<ast::Expr<'i>> for ast::Expr<'i> {
    fn steal(&mut self) -> Self {
        mem::take(self)
    }
}

use std::mem;
