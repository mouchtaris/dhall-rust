pub trait ASubstitution {
    fn commit_or_free_name(&mut self, name: &str, commit: bool);

    fn commit_name(&mut self, name: &str) {
        self.commit_or_free_name(name, true);
    }

    fn free_name(&mut self, name: &str) {
        self.commit_or_free_name(name, false);
    }
}

impl<'i> ASubstitution for ast::Expr<'i> {
    fn commit_or_free_name(&mut self, name: &str, commit: bool) {
        use ast::Expr::*;
        match self {
            Term1(t1) => t1.commit_or_free_name(name, commit),
            Let(defs, val) => {
                for (_, a, b) in defs {
                    a.commit_or_free_name(name, commit);
                    b.commit_or_free_name(name, commit);
                }
                val.commit_or_free_name(name, commit);
            }
            Lambda(_, a, b) => {
                a.commit_or_free_name(name, commit);
                b.commit_or_free_name(name, commit);
            }
        }
    }
}

impl<'i> ASubstitution for ast::Term1<'i> {
    fn commit_or_free_name(&mut self, name: &str, commit: bool) {
        use ast::Term1::*;
        match self {
            Term(t) => {
                t.commit_or_free_name(name, commit);
            }
            Evaluation(a, b) => {
                a.commit_or_free_name(name, commit);
                b.commit_or_free_name(name, commit);
            }
            Arrow(_, a, b) => {
                a.commit_or_free_name(name, commit);
                b.commit_or_free_name(name, commit);
            }
            With(a, _, b) => {
                a.commit_or_free_name(name, commit);
                b.commit_or_free_name(name, commit);
            }
            Operation(a, _, b) => {
                a.commit_or_free_name(name, commit);
                b.commit_or_free_name(name, commit);
            }
            IfThenElse(c, a, b) => {
                c.commit_or_free_name(name, commit);
                a.commit_or_free_name(name, commit);
                b.commit_or_free_name(name, commit);
            }
            Ascribe(a, b) => {
                a.commit_or_free_name(name, commit);
                b.commit_or_free_name(name, commit);
            }
            Construct(a, b) => {
                a.commit_or_free_name(name, commit);
                for (_, b) in b {
                    b.commit_or_free_name(name, commit);
                }
            }
        }
    }
}

impl<'i> ASubstitution for ast::Term<'i> {
    fn commit_or_free_name(&mut self, name: &str, commit: bool) {
        use ast::Term::*;
        match self {
            Var(n, s) if *n == name && commit => *s += 1,
            Var(n, s) if *n == name && *s > 0 => *s -= 1,
            FieldAccess(t, _) => t.commit_or_free_name(name, commit),
            Project(_, t, _) => t.commit_or_free_name(name, commit),
            Text(_, t) => {
                for (_, t) in t {
                    t.commit_or_free_name(name, commit);
                }
            }
            List(t) => {
                for t in t {
                    t.commit_or_free_name(name, commit)
                }
            }
            Record(t) => {
                for (_, t) in t {
                    t.commit_or_free_name(name, commit)
                }
            }
            TypeRecord(t) => {
                for (_, t) in t {
                    t.commit_or_free_name(name, commit)
                }
            }
            TypeEnum(t) => {
                for (_, t) in t {
                    t.commit_or_free_name(name, commit)
                }
            }
            Expr(t) => t.commit_or_free_name(name, commit),
            Merge(a, b) => {
                for (_, a) in a {
                    a.commit_or_free_name(name, commit);
                }
                b.commit_or_free_name(name, commit);
            }
            Integer(_) | Double(_) | Path(_) | Import { .. } | Embed(_) | Var(_, _) => (),
        }
    }
}

impl<T: ASubstitution> ASubstitution for Option<T> {
    fn commit_or_free_name(&mut self, name: &str, commit: bool) {
        match self {
            Some(t) => t.commit_or_free_name(name, commit),
            _ => (),
        }
    }
}

impl<T: ASubstitution> ASubstitution for Box<T> {
    fn commit_or_free_name(&mut self, name: &str, commit: bool) {
        self.as_mut().commit_or_free_name(name, commit)
    }
}
