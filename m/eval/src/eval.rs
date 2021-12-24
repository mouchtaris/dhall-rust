use super::{bail, AsImm, Error, Result, SymTable};
use show::Show;
use std::mem;

pub type Ctx<'i> = &'i mut Context<'i>;

pub fn ctx<'i>() -> Context<'i> {
    Context::new()
}

pub trait Eval<'i> {
    fn eval(&mut self, ctx: Ctx<'i>) -> Result<Ctx<'i>>;
}

fn in_scope<'i, A>(mut ctx: Ctx<'i>, a: &mut A) -> R<'i>
where
    A: Eval<'i>,
{
    ctx.sym_table.enter_scope();
    ctx = a.eval(ctx)?;
    ctx.sym_table.exit_scope();
    Ok(ctx)
}

fn in_scope2<'i, A, B>(mut ctx: Ctx<'i>, a: &mut A, b: &mut B) -> R<'i>
where
    A: Eval<'i>,
    B: Eval<'i>,
{
    ctx = in_scope(ctx, a)?;
    ctx = in_scope(ctx, b)?;
    Ok(ctx)
}

fn in_place_term1<'i>(mut ctx: Ctx<'i>, a: &mut ast::Term1<'i>) -> R<'i> {
    let mut e: ast::Expr = mem::take(a).into();
    ctx = in_scope(ctx, &mut e)?;

    *a = match e {
        ast::Expr::Term1(t1) => t1,
        e => ast::Term1::Term(ast::Term::Expr(ctx.rebox(e))),
    };
    Ok(ctx)
}

fn in_place_term<'i>(mut ctx: Ctx<'i>, a: &mut ast::Term<'i>) -> R<'i> {
    let mut e: ast::Expr = mem::take(a).into();
    ctx = in_scope(ctx, &mut e)?;

    *a = match e {
        ast::Expr::Term1(ast::Term1::Term(t)) => t,
        e => ast::Term::Expr(ctx.rebox(e)),
    };
    Ok(ctx)
}

impl<'i> Eval<'i> for ast::Expr<'i> {
    fn eval(&mut self, mut ctx: Ctx<'i>) -> Result<Ctx<'i>> {
        let mut tmp = <_>::default();
        mem::swap(self, &mut tmp);

        let value = &mut tmp;

        use ast::{Expr::*, Term::*, Term1::*};
        let norm = match value {
            Let(defs, val) => {
                for (name, typ, val) in defs {
                    ctx = in_scope2(ctx, typ, val)?;

                    let typ = match typ {
                        Some(b) => Some(mem::take(b.as_mut())),
                        _ => None,
                    };
                    let val = ctx.unbox(val);

                    ctx.sym_table.add(name, typ, Some(val));
                }

                ctx = val.eval(ctx)?;
                Ok(Some(val.as_mut()))
            }
            Term1(Term(Var(name, scope))) => {
                let scope = u8::from_str_radix(scope, 10).unwrap();
                match ctx.sym_table.lookup(name, scope) {
                    None => Ok(None),
                    Some(info) => Err(info.value.as_ref().map(|e| e.to_owned())),
                }
            }
            Term1(Term(TypeRecord(fields))) => {
                for (_, val) in fields {
                    ctx = in_scope(ctx, val)?;
                }
                Ok(None)
            }
            Term1(Term(TypeEnum(fields))) => {
                for (_, typ) in fields {
                    ctx = in_scope(ctx, typ)?;
                }
                Ok(None)
            }
            Term1(Term(Record(fields))) => {
                for (_, val) in fields {
                    ctx = in_scope(ctx, val)?;
                }
                Ok(None)
            }
            Term1(Term(Integer(_, _))) => Ok(None),
            Term1(Term(Double(_))) => Ok(None),
            Term1(Term(Text(_, _))) => Ok(None),
            Term1(Arrow(_, a, b)) => {
                ctx = in_scope2(ctx, a, b)?;
                Ok(None)
            }
            Term1(Ascribe(a, b)) => {
                ctx = in_place_term1(ctx, a)?;
                ctx = in_scope(ctx, b)?;
                Ok(None)
            }
            Term1(Operation(a, _, b)) => {
                ctx = in_place_term1(ctx, a)?;
                ctx = in_place_term1(ctx, b)?;
                Ok(None)
            }
            Term1(Evaluation(f, x)) => {
                ctx = in_place_term1(ctx, f)?;
                ctx = in_place_term(ctx, x)?;

                match (f.as_mut(), x) {
                    (Term(Expr(e)), x) => match e.as_mut() {
                        Lambda(n, _, b) => {
                            match mem::take(x) {
                                ast::Term::Var(m, "0") if *n == m => (),
                                x => {
                                    let x = ast::Expr::Term1(ast::Term1::Term(x));
                                    ctx.sym_table.add(n, None, Some(x));
                                }
                            }
                            ctx = b.eval(ctx)?;
                            Err(Some(ctx.unbox(b)))
                        }
                        _ => panic!("After-evaluation non lambda expression in substitution"),
                    },
                    (Term(Var(n, s)), _) => {
                        let s_ = u8::from_str_radix(s, 10).unwrap();
                        let info = ctx
                            .sym_table
                            .lookup(n, s_)
                            .ok_or(Error::from(format!("Name not found: {}", n)))?;
                        if info.value.is_none() {
                            // Normal
                            Ok(None)
                        } else {
                            panic!("After-evaluation with non-thunk symbol")
                        }
                    }
                    other => bail!("How to Evaluation {:?}", other),
                }
            }
            Lambda(_, a, b) => {
                ctx = in_scope2(ctx, a, b)?;
                Ok(None)
            }
            other => {
                let code = format!("{}", Show(other.as_imm()));
                let end = code
                    .char_indices()
                    .take(20)
                    .last()
                    .map(|(i, _)| i)
                    .unwrap_or(code.len());
                bail!("How to eval {}\n{:?}", &code[..end], other)
            }
        };

        match norm {
            Ok(Some(n)) => mem::swap(n, self),
            Err(Some(n)) => *self = n,
            _ => mem::swap(self, value),
        };
        Ok(ctx)
    }
}

/// The context of an evaluation.
pub struct Context<'i> {
    sym_table: SymTable<'i>,
    _shelf_box_expr: Vec<Box<ast::Expr<'i>>>,
    _shelf_box_term1: Vec<Box<ast::Term1<'i>>>,
}

impl<'i> Context<'i> {
    /// Create an empty evaluation context.
    pub fn new() -> Self {
        let mut ctx = Self {
            sym_table: <_>::default(),
            _shelf_box_expr: <_>::default(),
            _shelf_box_term1: <_>::default(),
        };
        ctx.init();
        ctx
    }

    fn init(&mut self) {
        let Self { sym_table, .. } = self;
        sym_table.enter_scope();

        log::warn!("TODO: Core-lib types [{} {}]", line!(), file!());

        fn n<'i>(n: &'i str) -> ast::Expr<'i> {
            ast::Expr::Term1(ast::Term1::Term(ast::Term::Var(n, "0")))
        }

        {
            let mut def_thunk = |a| sym_table.add(a, None, None);

            // Install core
            def_thunk("Type");
            def_thunk("List");
        }
    }

    fn unbox<T>(&mut self, val: &mut Box<T>) -> T
    where
        T: Default,
        Self: Unboxer<T>,
    {
        let mut boxval = mem::take(val);
        let val = mem::take(boxval.as_mut());
        self.box_shelf().push(boxval);
        val
    }

    fn rebox<T>(&mut self, mut val: T) -> Box<T>
    where
        Self: Unboxer<T>,
    {
        match self.box_shelf().pop() {
            Some(mut b) => {
                mem::swap(&mut val, b.as_mut());
                b
            }
            None => Box::new(val),
        }
    }
}

type R<'i> = Result<Ctx<'i>>;

impl<'i, T> Eval<'i> for &'i mut T
where
    T: Eval<'i>,
{
    fn eval(&mut self, ctx: Ctx<'i>) -> R<'i> {
        T::eval(*self, ctx)
    }
}

impl<'i, T> Eval<'i> for Option<T>
where
    T: Eval<'i>,
{
    fn eval(&mut self, ctx: Ctx<'i>) -> R<'i> {
        match self {
            Some(t) => T::eval(t, ctx),
            _ => Ok(ctx),
        }
    }
}

impl<'i, T> Eval<'i> for Box<T>
where
    T: Eval<'i>,
{
    fn eval(&mut self, ctx: Ctx<'i>) -> R<'i> {
        T::eval(self.as_mut(), ctx)
    }
}

pub trait Unboxer<T> {
    fn box_shelf(&mut self) -> &mut Vec<Box<T>>;
}

impl<'i> Unboxer<ast::Expr<'i>> for Context<'i> {
    fn box_shelf(&mut self) -> &mut Vec<Box<ast::Expr<'i>>> {
        &mut self._shelf_box_expr
    }
}

impl<'i> Unboxer<ast::Term1<'i>> for Context<'i> {
    fn box_shelf(&mut self) -> &mut Vec<Box<ast::Term1<'i>>> {
        &mut self._shelf_box_term1
    }
}
