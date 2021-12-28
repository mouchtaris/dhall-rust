use super::{bail, AsImm, Result, Set, Show, SymTable};
use std::mem;

pub type Ctx<'i> = &'i mut Context<'i>;

pub fn ctx<'i>() -> Context<'i> {
    Context::new()
}

pub trait Eval<'i> {
    fn eval(&mut self, ctx: Ctx<'i>) -> Result<Ctx<'i>>;
}

fn in_place_term1<'i>(mut ctx: Ctx<'i>, a: &mut ast::Term1<'i>) -> R<'i> {
    let mut e: ast::Expr = mem::take(a).into();
    ctx = e.eval(ctx)?;

    *a = match e {
        ast::Expr::Term1(t1) => t1,
        e => ast::Term1::Term(ast::Term::Expr(ctx.rebox(e))),
    };
    Ok(ctx)
}

fn in_place_term<'i>(mut ctx: Ctx<'i>, a: &mut ast::Term<'i>) -> R<'i> {
    let mut e: ast::Expr = mem::take(a).into();
    ctx = e.eval(ctx)?;

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
                ctx.sym_table.mark_scope();

                for (name, typ, val) in defs {
                    ctx = typ.eval(ctx)?;
                    ctx = val.eval(ctx)?;

                    let typ = match typ {
                        Some(b) => Some(mem::take(b.as_mut())),
                        _ => None,
                    };
                    let val = ctx.unbox(val);

                    ctx.sym_table.enter_scope();
                    ctx.sym_table.add(name, typ, Some(val));
                }

                ctx = val.eval(ctx)?;
                ctx.sym_table.return_to_marked_scope();
                Ok(Some(val.as_mut()))
            }
            Term1(Term(Var(name, scope, scope_fix))) => {
                log::trace!(
                    "{:4} eval Var {} @{} >{:?}",
                    line!(),
                    name,
                    scope,
                    scope_fix
                );

                let info = match scope_fix {
                    None => {
                        let info = ctx.sym_table.lookup(name, *scope)?;
                        log::trace!("{:4} Offset {:?}/{:?}", line!(), name, scope,);
                        info
                    }
                    Some(soff) => ctx.sym_table.lookup_from_offset(*soff, name)?,
                };

                match &info.value {
                    None => Ok(None), // thunk value - ok
                    Some(val) => Err(Some(val.to_owned())),
                }
            }
            Term1(Term(TypeRecord(fields))) => {
                for (_, val) in fields {
                    ctx = val.eval(ctx)?;
                }
                Ok(None)
            }
            Term1(Term(TypeEnum(fields))) => {
                for (_, typ) in fields {
                    ctx = typ.eval(ctx)?;
                }
                Ok(None)
            }
            Term1(Term(Record(fields))) => {
                for (_, val) in fields {
                    ctx = val.eval(ctx)?;
                }
                Ok(None)
            }
            Term1(Term(List(fields))) => {
                for val in fields {
                    ctx = val.eval(ctx)?;
                }
                Ok(None)
            }
            Term1(Term(Integer(_, _))) => Ok(None),
            Term1(Term(Double(_))) => Ok(None),
            Term1(Term(Text(_, _))) => Ok(None),
            Term1(Term(Expr(e))) => {
                ctx = e.eval(ctx)?;
                let e = ctx.unbox(e);

                // Replace with inner expr
                Err(Some(e))
            }
            Term1(Term(Project(1, t, selectors))) => {
                log::trace!(
                    "{:4} eval Project(1) {} . {:?}",
                    line!(),
                    Show(t.as_ref()),
                    selectors
                );
                ctx = in_place_term(ctx, t)?;
                let mut names = Set::new();
                for name in selectors {
                    // DO NOT Eval! These are declaring identifiers, not to be looked up.
                    // (dispite being a term in ast, this is parsing bogus).
                    //
                    //ctx = in_place_term1(ctx, name)?;
                    //
                    // Simply extract to names:
                    match name {
                        Term(Var(n, _, _)) => {
                            names.insert(*n);
                        }
                        o => panic!("Projection selectors must be identifiers: {:?}", o),
                    }
                }
                match t.as_mut() {
                    Record(fields) => {
                        fields.retain(|(name, _)| {
                            name.front().map(|p| names.contains(p)).unwrap_or(false)
                        });
                        let mut t = mem::take(t);
                        let t = ctx.unbox(&mut t);
                        Err(Some(Term1(Term(t))))
                    }
                    t if ctx.is_thunk_term(t)? => Ok(None),
                    o => panic!(
                        "Projection term must be a type record (or thunk term): {:?}",
                        o
                    ),
                }
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
                            ctx = field.eval(ctx)?;

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
                    TypeEnum(_) => Ok(None),
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
                ctx = b.eval(ctx)?;
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
                log::trace!(
                    "{:4} eval Evaluation( {} , {} )",
                    line!(),
                    Show(f.as_ref()),
                    Show(&*x)
                );

                match (f.as_mut(), x) {
                    (Term(Expr(e)), x) => match e.as_mut() {
                        Lambda(n, t, b) => {
                            let x: ast::Term = mem::take(x);
                            ctx.sym_table.enter_scope();
                            match x {
                                ast::Term::Var(m, 0, None) if *n == m => {
                                    log::trace!("{:4} Evaluation: skip defining {}", line!(), m);
                                }
                                x => {
                                    let x = ast::Expr::Term1(ast::Term1::Term(x));
                                    ctx.sym_table.add(n, None, Some(x));
                                }
                            }
                            ctx = t.eval(ctx)?;
                            ctx = b.eval(ctx)?;
                            ctx.sym_table.exit_scope();
                            Err(Some(ctx.unbox(b)))
                        }
                        Term1(Evaluation(f, _)) if ctx.is_thunk_term1(f.as_ref())? => Ok(None),
                        o => panic!(
                            "After-evaluation non lambda expression in substitution: {:?}",
                            o
                        ),
                    },
                    (Term(FieldAccess(t, _)), _) => match t.as_mut() {
                        t if ctx.is_thunk_term(t)? => Ok(None),
                        o => panic!("After-evaluation non field accessible: {:?}", o),
                    },
                    (t, _) if ctx.is_thunk_term1(t)? => Ok(None),
                    other => panic!("How to Evaluation {:?}", other),
                }
                .map(|r| {
                    log::trace!("{:4} eval Evaluation => {:?}", line!(), r);
                    r
                })
            }
            Term1(Term(Merge(merge_table, t))) => {
                ctx = in_place_term(ctx, t.as_mut())?;
                log::trace!(
                    "{:4} eval Merge ( {:?} , {} )",
                    line!(),
                    merge_table,
                    Show(t.as_ref())
                );
                match t.as_mut() {
                    Expr(e) => match e.as_mut() {
                        Term1(Evaluation(eval_f, eval_a)) => match eval_f.as_mut() {
                            Term(FieldAccess(fields_t, fields_n)) => {
                                match fields_t.as_mut() {
                                    TypeEnum(type_enum) => {
                                        // This is in fact a type-enum-variant construction -- the
                                        // only thing expected to evalluate as merge's argument.
                                        //
                                        //  < a : Natural >.a 12
                                        //
                                        // In typeless dust, we actually only care for the field
                                        // name...
                                        let _ = type_enum;
                                        // And look it up in the merge table
                                        let mut mti = merge_table.iter_mut();
                                        loop {
                                            match mti.next() {
                                                Some((name, data_handler)) => {
                                                    ctx = data_handler.eval(ctx)?;

                                                    // Only care for the first/only element:
                                                    let name = name.front().unwrap();
                                                    if fields_n == name {
                                                        // Call the merge handler with the variant data.
                                                        match data_handler.as_mut() {
                                                            Lambda(_, _, _) => {
                                                                let data_handler = ctx.rebox(Term(Expr(mem::take(data_handler))));
                                                                let eval_a = mem::take(eval_a);
                                                                let mut re_eval = Term1(Evaluation(data_handler, eval_a));
                                                                log::trace!("Merge result re-evaluation: {:?}", re_eval);
                                                                ctx = re_eval.eval(ctx)?;
                                                                break Err(Some(re_eval))
                                                            }
                                                            o => panic!("Expecting Lambda for merge handler data: {:?}", o),
                                                        }
                                                    }
                                                }
                                                None => {
                                                    bail!(
                                                        "Field not found: {} in {:?}",
                                                        fields_n,
                                                        type_enum
                                                    );
                                                }
                                            }
                                        }
                                    }
                                    o => panic!("Merge expression has to evaluate to an enum-type construction: Evaluation(FiledAccess(TypeEnum(...), ...), ...): {:?}", o),
                                }
                            }
                            t if ctx.is_thunk_term1(&*t)? => Ok(None),
                            o => panic!("Merge expression has to evaluate to an enum-type construction: Evaluation(FieldAccess, ...): {:?}", o),
                        },
                        o => panic!("Merge expression has to evaluate to an enum-type construction: {:?}", o),
                    },
                    t if ctx.is_thunk_term(t)? => Ok(None),
                    o => panic!("Merge term can be an expression or thunk term: {:?}", o),
                }
            }
            Term1(Arrow(n, a, b)) => {
                log::trace!(
                    "{:4} eval Arrow {:?} {} {}",
                    line!(),
                    n,
                    Show(a.as_ref()),
                    Show(b.as_ref())
                );
                ctx = a.eval(ctx)?;

                ctx.sym_table.enter_scope();
                if let Some(name) = n {
                    ctx.sym_table.add_thunk(name);
                }
                ctx = b.eval(ctx)?;
                ctx.sym_table.exit_scope();

                Ok(None)
            }
            Term1(IfThenElse(c, a, b)) => {
                ctx = a.eval(ctx)?;
                ctx = b.eval(ctx)?;
                ctx = c.eval(ctx)?;
                match c.as_ref() {
                    Term1(Term(Var("True", 0, _))) => Err(Some(ctx.unbox(a))),
                    Term1(Term(Var("False", 0, _))) => Err(Some(ctx.unbox(b))),
                    Term1(t1) if ctx.is_thunk_term1(t1)? => Ok(None),
                    other => panic!("How to eval if-then-else? {:?}", other),
                }
            }
            Lambda(name, a, b) => {
                log::trace!(
                    "{:4} eval Lambda {} : {} → {}",
                    line!(),
                    name,
                    Show(a.as_ref().map(|a| a.as_ref()).unwrap_or(&<_>::default())),
                    Show(b.as_ref())
                );
                ctx = a.eval(ctx)?;

                ctx.sym_table.enter_scope();
                ctx.sym_table.add_thunk(name);
                ctx = b.eval(ctx)?;
                ctx.sym_table.exit_scope();

                log::trace!(
                    "{:4} eval Lambda => {} → {}",
                    line!(),
                    name,
                    Show(b.as_ref())
                );
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
    _shelf_box_term: Vec<Box<ast::Term<'i>>>,
}

impl<'i> Context<'i> {
    /// Create an empty evaluation context.
    pub fn new() -> Self {
        let mut ctx = Self {
            sym_table: <_>::default(),
            _shelf_box_expr: <_>::default(),
            _shelf_box_term1: <_>::default(),
            _shelf_box_term: <_>::default(),
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
            def_thunk("Integer/toDouble");
            def_thunk("Kind");
            def_thunk("List");
            def_thunk("List/build");
            def_thunk("List/fold");
            def_thunk("List/head");
            def_thunk("List/indexed");
            def_thunk("List/last");
            def_thunk("List/length");
            def_thunk("List/reverse");
            def_thunk("Natural");
            def_thunk("Natural/even");
            def_thunk("Natural/fold");
            def_thunk("Natural/isZero");
            def_thunk("Natural/odd");
            def_thunk("Natural/show");
            def_thunk("Natural/subtract");
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
            Var(n, s, _) => self.sym_table.is_thunk1(0, n, *s)?,
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

impl<'i> Unboxer<ast::Term<'i>> for Context<'i> {
    fn box_shelf(&mut self) -> &mut Vec<Box<ast::Term<'i>>> {
        &mut self._shelf_box_term
    }
}
