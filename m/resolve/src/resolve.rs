use super::*;

pub trait Visitor {
    fn visit_import(&mut self, path: &str, term: &mut ast::Term) -> Result<()> {
        let _ = (path, term);
        Ok(())
    }
    fn visit_register(&mut self, path: &str) -> Result<()> {
        let _ = path;
        Ok(())
    }
}

pub trait Resolve {
    fn resolve<V: Visitor>(&mut self, reservoir: &mut V) -> Result<()>;

    fn visit_import<F>(&mut self, f: F) -> Result<()>
    where
        F: FnMut(&str, &mut ast::Term) -> Result<()>,
    {
        self.resolve(&mut ImportVisitor(f))
    }

    fn visit_register<F>(&mut self, f: F) -> Result<()>
    where
        F: FnMut(&str) -> Result<()>,
    {
        self.resolve(&mut RegisterVisitor(f))
    }
}

impl<'i> Resolve for ast::Expr<'i> {
    fn resolve<V: Visitor>(&mut self, r: &mut V) -> Result<()> {
        use ast::Expr::*;
        match self {
            Term1(t1) => t1.resolve(r),
            Let(defs, val) => (defs, val).resolve(r),
            Lambda(_, typ, val) => (typ, val).resolve(r),
        }
    }
}

impl<'i> Resolve for ast::Term1<'i> {
    fn resolve<V: Visitor>(&mut self, r: &mut V) -> Result<()> {
        use ast::Term1::*;
        match self {
            Term(t) => t.resolve(r),
            Evaluation(f, x) => (f, x).resolve(r),
            Arrow(_, a, b) => (a, b).resolve(r),
            With(t, _, v) => (t, v).resolve(r),
            Operation(a, _, b) => (a, b).resolve(r),
            IfThenElse(c, a, b) => (c, a, b).resolve(r),
            Ascribe(t, v) => (t, v).resolve(r),
            Construct(t, d) => (t, d).resolve(r),
        }
    }
}

impl<'i> Resolve for ast::Term<'i> {
    fn resolve<V: Visitor>(&mut self, r: &mut V) -> Result<()> {
        use ast::Term::*;
        match self {
            Integer(_, _) => Ok(()),
            Double(_) => Ok(()),
            FieldAccess(term, _) => term.resolve(r),
            Project(_, term, fields) => (term, fields).resolve(r),
            Path(_) => Ok(()),
            Var(_, _, _) => Ok(()),
            Text(_, ts) => ts.resolve(r),
            List(vs) => vs.resolve(r),
            TypeRecord(es) | Record(es) => es.resolve(r),
            TypeEnum(es) => es.resolve(r),
            Import {
                path: "missing", ..
            } => Ok(()),
            &mut Import { path, .. } => {
                r.visit_register(path)?;
                r.visit_import(path, self)?;
                Ok(())
            }
            Embed(_) => Ok(()),
            Expr(e) => e.resolve(r),
            Merge(d, t) => (d, t).resolve(r),
        }
    }
}

impl<T: Resolve> Resolve for Option<T> {
    fn resolve<V: Visitor>(&mut self, r: &mut V) -> Result<()> {
        if let Some(t) = self {
            t.resolve(r)?;
        }
        Ok(())
    }
}

impl<T: Resolve> Resolve for Box<T> {
    fn resolve<V: Visitor>(&mut self, r: &mut V) -> Result<()> {
        self.as_mut().resolve(r)
    }
}

impl<A: Resolve, B: Resolve> Resolve for (A, B) {
    fn resolve<V: Visitor>(&mut self, r: &mut V) -> Result<()> {
        let (a, b) = self;
        a.resolve(r)?;
        b.resolve(r)
    }
}

impl<A: Resolve, B: Resolve, C: Resolve> Resolve for (A, B, C) {
    fn resolve<V: Visitor>(&mut self, r: &mut V) -> Result<()> {
        let (a, b, c) = self;
        a.resolve(r)?;
        b.resolve(r)?;
        c.resolve(r)
    }
}

impl<T: Resolve> Resolve for ast::Deq<T> {
    fn resolve<V: Visitor>(&mut self, r: &mut V) -> Result<()> {
        for t in self {
            t.resolve(r)?;
        }
        Ok(())
    }
}

impl<'a, T: Resolve> Resolve for &'a mut T {
    fn resolve<V: Visitor>(&mut self, r: &mut V) -> Result<()> {
        T::resolve(self, r)
    }
}

impl<'a> Resolve for &'a str {
    fn resolve<V: Visitor>(&mut self, _: &mut V) -> Result<()> {
        Ok(())
    }
}

struct ImportVisitor<F>(F);
impl<F> Visitor for ImportVisitor<F>
where
    F: FnMut(&str, &mut ast::Term) -> Result<()>,
{
    fn visit_import(&mut self, path: &str, term: &mut ast::Term) -> Result<()> {
        (self.0)(path, term)
    }
}

struct RegisterVisitor<F>(F);
impl<F> Visitor for RegisterVisitor<F>
where
    F: FnMut(&str) -> Result<()>,
{
    fn visit_register(&mut self, path: &str) -> Result<()> {
        (self.0)(path)
    }
}
