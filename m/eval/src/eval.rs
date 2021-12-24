use super::{bail, Error, Evaluator, Result, Stealr};

pub type Value<'i> = Result<ast::Expr<'i>>;

/// The main trait for evaluation.
///
/// If something can be evaluated, it's always in-place.
pub trait Eval<'i> {
    fn eval(self, ctx: &mut Evaluator<'i>) -> Value<'i>;

    fn eval_inplace(&mut self, ctx: &mut Evaluator<'i>) -> Result<()>
    where
        Self: Stealr<ast::Expr<'i>>,
    {
        let mut expr = <_>::default();

        self.stealr_take(&mut expr);

        expr = expr.eval(ctx)?;

        self.stealr_give(&mut expr);

        Ok(())
    }
}

/// An expression can be evaluated in place.
///
/// This is the evaluation operation backbone.
impl<'i, T> Eval<'i> for T
where
    T: Stealr<ast::Expr<'i>>,
{
    fn eval(mut self, ctx: &mut Evaluator<'i>) -> Value<'i> {
        use ast::{Expr::*, Term::*, Term1::*};

        let expr = self.steal_out();

        Ok(match expr {
            Let(defs, val) => {
                for (name, typ, val) in defs {
                    let typ = match typ {
                        Some(t) => {
                            ctx.enter_scope();
                            let t = Some(t.eval(ctx)?);
                            ctx.exit_scope();
                            t
                        }
                        _ => None,
                    };

                    ctx.enter_scope();
                    let val = val.eval(ctx)?;
                    ctx.exit_scope();

                    ctx.assign(name, typ, Some(val));
                }

                val.eval(ctx)?
            }
            Term1(Term(Var(name, scope_str))) => {
                let scope = u8::from_str_radix(scope_str, 10).unwrap();
                let info = ctx
                    .lookup(name, scope)
                    .ok_or_else(|| Error::from(format!("Symbol not found: {}", name)))?;

                match &info.value {
                    Some(val) => val.to_owned(),
                    None => Term1(Term(Var(name, scope_str))),
                }
            }
            Term1(Evaluation(f, x)) => {
                let f = f.eval(ctx)?;
                match (f, x) {
                    o => panic!("{:?}", o),
                }
            }
            mut in_place => {
                match &mut in_place {
                    Term1(Arrow(_, a, b)) => {
                        ctx.enter_scope();
                        a.eval_inplace(ctx)?;
                        ctx.exit_scope();

                        ctx.enter_scope();
                        b.eval_inplace(ctx)?;
                        ctx.exit_scope();
                    }
                    Term1(Term(TypeRecord(fields))) => {
                        for (_, val) in fields {
                            ctx.enter_scope();
                            val.eval_inplace(ctx)?;
                            ctx.exit_scope();
                        }
                    }
                    o => bail!("How to eval {:?}", o),
                };
                in_place
            }
        })
    }
}
