use super::{bail, Evaluator, Result};

/// The main trait for evaluation.
///
/// If something can be evaluated, it's always in-place.
pub trait Eval<'i> {
    fn eval(&mut self, ctx: &mut Evaluator<'i>) -> Result<()>;
}

/// An Option will evaluate the inside, if any.
impl<'i, T> Eval<'i> for Option<T>
where
    T: Eval<'i>,
{
    fn eval(&mut self, ctx: &mut Evaluator<'i>) -> Result<()> {
        match self {
            Some(t) => t.eval(ctx),
            None => Ok(()),
        }
    }
}

/// A Box will evaluate the inside.
impl<'i, T> Eval<'i> for Box<T>
where
    T: Eval<'i>,
{
    fn eval(&mut self, ctx: &mut Evaluator<'i>) -> Result<()> {
        self.as_mut().eval(ctx)
    }
}

/// An expression can be evaluated in place.
///
/// This is the evaluation operation backbone.
impl<'i> Eval<'i> for ast::Expr<'i> {
    fn eval(&mut self, ctx: &mut Evaluator<'i>) -> Result<()> {
        use ast::{Expr::*, Term::*, Term1::*};
        let expr = self;
        match expr {
            Let(defs, val) => {
                for (name, typ, val) in defs {
                    ctx.sym_table.enter_scope();
                    typ.eval(ctx)?;
                    val.eval(ctx)?;
                    ctx.sym_table.exit_scope();

                    ctx.sym_table.add(name, Some(val));
                }
            }
            Term1(Term(Var(name, scope))) => {
                log::warn!("todo::eval name lookup {} {}", file!(), line!());
            }
            o => bail!("How to eval {:?}", o),
        }
        Ok(())
    }
}
