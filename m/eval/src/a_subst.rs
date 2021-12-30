pub trait ASubstitution {
    fn commit_name(&mut self, name: &str);
}

impl<'i> ASubstitution for ast::Expr<'i> {
    fn commit_name(&mut self, name: &str) {
        use ast::Expr::*;
        match self {
            Term1(t1) => t1.commit_name(name),
            Let(defs, val) => {
                for (_, a, b) in defs {
                    a.commit_name(name);
                    b.commit_name(name);
                }
                val.commit_name(name);
            }
            Lambda(_, a, b) => {
                a.commit_name(name);
                b.commit_name(name);
            }
        }
    }
}

impl<'i> ASubstitution for ast::Term1<'i> {
    fn commit_name(&mut self, name: &str) {
        use ast::Term1::*;
        match self {
            Term(t) => {
                t.commit_name(name);
            }
            Evaluation(a, b) => {
                a.commit_name(name);
                b.commit_name(name);
            }
            Arrow(_, a, b) => {
                a.commit_name(name);
                b.commit_name(name);
            }
            With(a, _, b) => {
                a.commit_name(name);
                b.commit_name(name);
            }
            Operation(a, _, b) => {
                a.commit_name(name);
                b.commit_name(name);
            }
            IfThenElse(c, a, b) => {
                c.commit_name(name);
                a.commit_name(name);
                b.commit_name(name);
            }
            Ascribe(a, b) => {
                a.commit_name(name);
                b.commit_name(name);
            }
            Construct(a, b) => {
                a.commit_name(name);
                for (_, b) in b {
                    b.commit_name(name);
                }
            }
        }
    }
}

impl<'i> ASubstitution for ast::Term<'i> {
    fn commit_name(&mut self, name: &str) {
        use ast::Term::*;
        match self {
            Var(n, s) if *n == name => *s += 1,
            FieldAccess(t, _) => t.commit_name(name),
            Project(_, t, _) => t.commit_name(name),
            Text(_, t) => {
                for (_, t) in t {
                    t.commit_name(name);
                }
            }
            List(t) => {
                for t in t {
                    t.commit_name(name)
                }
            }
            Record(t) => {
                for (_, t) in t {
                    t.commit_name(name)
                }
            }
            TypeRecord(t) => {
                for (_, t) in t {
                    t.commit_name(name)
                }
            }
            TypeEnum(t) => {
                for (_, t) in t {
                    t.commit_name(name)
                }
            }
            Expr(t) => t.commit_name(name),
            Merge(a, b) => {
                for (_, a) in a {
                    a.commit_name(name);
                }
                b.commit_name(name);
            }
            Integer(_) | Double(_) | Path(_) | Import { .. } | Embed(_) | Var(_, _) => (),
        }
    }
}

impl<T: ASubstitution> ASubstitution for Option<T> {
    fn commit_name(&mut self, name: &str) {
        match self {
            Some(t) => t.commit_name(name),
            _ => (),
        }
    }
}

impl<T: ASubstitution> ASubstitution for Box<T> {
    fn commit_name(&mut self, name: &str) {
        self.as_mut().commit_name(name)
    }
}
