pub const VERSION: &str = "0.0.1";

use error::Result;

#[derive(Default)]
pub struct Reservoir {
    pub uris: ast::Deq<String>,
}

impl Reservoir {
    pub fn register<P: Into<String>>(&mut self, path: P) {
        self.uris.push_back(path.into());
    }
}

pub trait Resolve {
    fn resolve(&mut self, reservoir: &mut Reservoir) -> Result<()>;
}

impl<'i> Resolve for ast::Expr<'i> {
    fn resolve(&mut self, r: &mut Reservoir) -> Result<()> {
        use ast::Expr::*;
        match self {
            Term1(t1) => t1.resolve(r),
            Let(defs, val) => (defs, val).resolve(r),
            Lambda(_, typ, val) => (typ, val).resolve(r),
        }
    }
}

impl<'i> Resolve for ast::Term1<'i> {
    fn resolve(&mut self, r: &mut Reservoir) -> Result<()> {
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
    fn resolve(&mut self, r: &mut Reservoir) -> Result<()> {
        use ast::Term::*;
        match self {
            Integer(_, _) => Ok(()),
            FieldAccess(term, _) => term.resolve(r),
            Project(_, term, fields) => (term, fields).resolve(r),
            Path(_) => Ok(()),
            Var(_) => Ok(()),
            Text(_, ts) => ts.resolve(r),
            List(vs) => vs.resolve(r),
            TypeRecord(es) | Record(es) => es.resolve(r),
            TypeEnum(es) => es.resolve(r),
            Import(path, _) => Ok(r.register(*path)),
            Expr(e) => e.resolve(r),
            Merge(d, t) => (d, t).resolve(r),
        }
    }
}

impl<T: Resolve> Resolve for Option<T> {
    fn resolve(&mut self, r: &mut Reservoir) -> Result<()> {
        if let Some(t) = self {
            t.resolve(r)?;
        }
        Ok(())
    }
}

impl<T: Resolve> Resolve for Box<T> {
    fn resolve(&mut self, r: &mut Reservoir) -> Result<()> {
        self.as_mut().resolve(r)
    }
}

impl<A: Resolve, B: Resolve> Resolve for (A, B) {
    fn resolve(&mut self, r: &mut Reservoir) -> Result<()> {
        let (a, b) = self;
        a.resolve(r)?;
        b.resolve(r)
    }
}

impl<A: Resolve, B: Resolve, C: Resolve> Resolve for (A, B, C) {
    fn resolve(&mut self, r: &mut Reservoir) -> Result<()> {
        let (a, b, c) = self;
        a.resolve(r)?;
        b.resolve(r)?;
        c.resolve(r)
    }
}

impl<T: Resolve> Resolve for ast::Deq<T> {
    fn resolve(&mut self, r: &mut Reservoir) -> Result<()> {
        for t in self {
            t.resolve(r)?;
        }
        Ok(())
    }
}

impl<'a, T: Resolve> Resolve for &'a mut T {
    fn resolve(&mut self, r: &mut Reservoir) -> Result<()> {
        T::resolve(self, r)
    }
}

impl<'a> Resolve for &'a str {
    fn resolve(&mut self, _: &mut Reservoir) -> Result<()> {
        Ok(())
    }
}
