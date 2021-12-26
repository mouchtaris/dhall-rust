use super::{bail, AsImm, Result, SymTable};
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

                    ctx.sym_table.enter_shadow();
                    ctx.sym_table.add(name, typ, Some(val));
                }

                ctx = val.eval(ctx)?;
                Ok(Some(val.as_mut()))
            }
            Term1(Term(Var(name, scope, scope_fix))) => {
                // Fix this name to the current active scope.
                // Prevents destruction like
                //
                //    let Head = { head : Bool }
                //
                //    in  λ(ls : Head) →
                //
                //      let ls = ls.head            -- Scope danger!!!
                //
                //      in ls { head = True }
                //
                let info = ctx.sym_table.lookup(name, *scope)?;
                *scope_fix = Some(info.scope_id);

                match &info.value {
                    None => Ok(None), // thunk value - ok
                    Some(val) => Err(Some(val.to_owned())),
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
            Term1(Term(List(fields))) => {
                for val in fields {
                    ctx = in_scope(ctx, val)?;
                }
                Ok(None)
            }
            Term1(Term(Integer(_, _))) => Ok(None),
            Term1(Term(Double(_))) => Ok(None),
            Term1(Term(Text(_, _))) => Ok(None),
            Term1(Term(Expr(e))) => {
                ctx = in_scope(ctx, e)?;
                let e = ctx.unbox(e);

                // Replace with inner expr
                Err(Some(e))
            }
            Term1(Term(FieldAccess(t, name))) => {
                ctx = in_place_term(ctx, t)?;
                match t.as_mut() {
                    Record(fields) => {
                        fields.retain(|(path, _)| path.front().map(|s| name == s).unwrap_or(false));

                        let mut parafields = mem::take(fields);
                        let mut emerged = None;
                        let mut retained = ast::Deq::new();

                        for (path, field) in &mut parafields {
                            ctx = in_scope(ctx, field)?;

                            path.pop_front();
                            if path.is_empty() {
                                emerged = Some(field);
                            } else {
                                retained.push_back((mem::take(path), mem::take(field)));
                            }
                        }

                        if let Some(inner) = emerged {
                            let mut inner = ctx.unbox(inner);
                            // replace with inner value
                            match &mut inner {
                                Term1(Term(Record(inner_fields))) => {
                                    inner_fields.append(&mut retained);
                                }
                                _ if retained.is_empty() => {}
                                _ => panic! {
                                    "How to access {} from {}?",
                                    name, Show(t.as_ref())
                                },
                            }
                            Err(Some(inner))
                        } else {
                            Err(Some(Term1(Term(Record(retained)))))
                        }
                    }
                    t if ctx.is_thunk_term(t)? => Ok(None),
                    other => panic!(
                        "Field Access of: {:?} {}",
                        other,
                        ctx.is_thunk_term(&other)?
                    ),
                }
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
                                ast::Term::Var(m, 0, None) if *n == m => (),
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
                    (t, _) if ctx.is_thunk_term1(t)? => Ok(None),
                    other => panic!("How to Evaluation {:?}", other),
                }
            }
            Term1(Term(Merge(merge_fields, t))) => {
                ctx = in_place_term(ctx, t.as_mut())?;
                for (_, field) in merge_fields {
                    ctx = in_scope(ctx, field)?;
                }
                match t {
                    t if ctx.is_thunk_term(t)? => Ok(None),
                    o => panic!("How to merge {:?}", o),
                }
            }
            Term1(Arrow(n, a, b)) => {
                ctx = in_scope(ctx, a)?;

                ctx.sym_table.enter_scope();
                if let Some(name) = n {
                    ctx.sym_table.add_thunk(name);
                }
                ctx = in_scope(ctx, b)?;
                ctx.sym_table.exit_scope();

                Ok(None)
            }
            Term1(IfThenElse(c, a, b)) => {
                ctx = in_scope2(ctx, a, b)?;
                ctx = in_scope(ctx, c)?;
                match c.as_ref() {
                    Term1(Term(Var("True", 0, _))) => Err(Some(ctx.unbox(a))),
                    Term1(Term(Var("False", 0, _))) => Err(Some(ctx.unbox(b))),
                    Term1(t1) if ctx.is_thunk_term1(t1)? => Ok(None),
                    other => panic!("How to eval if-then-else? {:?}", other),
                }
            }
            Lambda(name, a, b) => {
                ctx = in_scope(ctx, a)?;

                ctx.sym_table.enter_scope();
                ctx.sym_table.add_thunk(name);
                ctx = in_scope(ctx, b)?;
                ctx.sym_table.exit_scope();

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
                panic!("How to eval {}\n{:?}", &code[..end], other)
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

        {
            let mut def_thunk = |a| sym_table.add(a, None, None);

            // Install core
            def_thunk("assert");
            def_thunk("Bool");
            def_thunk("Double");
            def_thunk("Double/show");
            def_thunk("False");
            def_thunk("Integer");
            def_thunk("Integer/clamp");
            def_thunk("Integer/negate");
            def_thunk("Integer/show");
            def_thunk("Kind");
            def_thunk("List");
            def_thunk("List/build");
            def_thunk("List/fold");
            def_thunk("List/reverse");
            def_thunk("Natural");
            def_thunk("Natural/even");
            def_thunk("Natural/isZero");
            def_thunk("Natural/show");
            def_thunk("Natural/toInteger");
            def_thunk("None");
            def_thunk("Optional");
            def_thunk("Some");
            def_thunk("Text");
            def_thunk("Text/replace");
            def_thunk("Text/show");
            def_thunk("toMap");
            def_thunk("True");
            def_thunk("Type");
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

    fn is_thunk_term(&self, t: &ast::Term) -> Result<bool> {
        use ast::Term::*;

        Ok(match t {
            Var(n, s, Some(f)) => self.sym_table.is_thunk1(*f, n, *s)?,
            FieldAccess(t, _) => self.is_thunk_term(t)?,
            Record(_) => false,
            Merge(_, t) => self.is_thunk_term(t)?,
            Expr(e) => self.is_thunk_expr(e)?,
            TypeEnum(_) => true,
            other => panic!("How to know if thunk term? {:?} ", other,),
        })
    }

    fn is_thunk_term1(&self, t: &ast::Term1) -> Result<bool> {
        use ast::Term1::*;

        Ok(match t {
            Term(t) => self.is_thunk_term(t)?,
            Evaluation(t, _) => self.is_thunk_term1(t)?,
            Operation(a, _, b) => self.is_thunk_term1(a)? || self.is_thunk_term1(b)?,
            other => panic!("How to know if thunk term1? {:?}", other,),
        })
    }

    fn is_thunk_expr(&self, t: &ast::Expr) -> Result<bool> {
        use ast::Expr::*;

        Ok(match t {
            Term1(t1) => self.is_thunk_term1(t1)?,
            other => panic!("How to know if thunk expr? {:?}", other,),
        })
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
