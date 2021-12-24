/// A private method of handling "Into".
///
/// This is tuned to AST elements (and their containers) turning themselves
/// into Expr.
///
pub trait Stealr<T> {
    /// Transmute self into &mut T and call block.
    ///
    /// Swap not guaranteed.
    fn stealr_borrow<F: FnOnce(&mut T)>(&mut self, block: F);

    /// "Take" Stealr's value (into `t`).
    ///
    /// Swap not guaranteed.
    fn stealr_take(&mut self, t: &mut T) {
        self.stealr_borrow(|u| mem::swap(u, t));
    }

    /// "Give" Stealr t's value.
    ///
    /// Swap not guaranteed.
    fn stealr_give(&mut self, t: &mut T) {
        self.stealr_borrow(|u| mem::swap(u, t));
    }

    fn steal_out(&mut self) -> T
    where
        T: Default,
    {
        let mut t = <_>::default();
        self.stealr_take(&mut t);
        t
    }
}

impl<A, T> Stealr<T> for Box<A>
where
    A: Stealr<T>,
{
    fn stealr_borrow<F: FnOnce(&mut T)>(&mut self, block: F) {
        self.as_mut().stealr_borrow(block)
    }

    fn stealr_take(&mut self, t: &mut T) {
        self.as_mut().stealr_take(t)
    }

    fn stealr_give(&mut self, t: &mut T) {
        self.as_mut().stealr_give(t)
    }
}

impl<'a, A, T> Stealr<T> for &'a mut A
where
    A: Stealr<T>,
{
    fn stealr_borrow<F: FnOnce(&mut T)>(&mut self, block: F) {
        A::stealr_borrow(self, block)
    }

    fn stealr_take(&mut self, t: &mut T) {
        A::stealr_take(*self, t)
    }

    fn stealr_give(&mut self, t: &mut T) {
        A::stealr_give(*self, t)
    }
}

impl<'i> Stealr<ast::Expr<'i>> for ast::Expr<'i> {
    fn stealr_borrow<F: FnOnce(&mut ast::Expr<'i>)>(&mut self, block: F) {
        block(self)
    }
}

impl<'i> Stealr<ast::Expr<'i>> for ast::Term1<'i> {
    fn stealr_take(&mut self, t: &mut ast::Expr<'i>) {
        *t = ast::Expr::Term1(mem::take(self));
    }

    fn stealr_give(&mut self, t: &mut ast::Expr<'i>) {
        *self = ast::Term1::Term(ast::Term::Expr(Box::new(mem::take(t))));
        panic!("Should not be actually used");
    }

    fn stealr_borrow<F: FnOnce(&mut ast::Expr<'i>)>(&mut self, _: F) {
        panic!("Should not be actually used");
    }
}

impl<T> Stealr<Option<T>> for Option<T> {
    fn stealr_borrow<F: FnOnce(&mut Option<T>)>(&mut self, block: F) {
        block(self)
    }
}

use std::mem;
