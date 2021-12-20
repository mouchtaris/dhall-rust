pub const VERSION: &str = "0.0.1";

use std::fmt;

pub struct Show<T>(pub T);

impl<'i> fmt::Display for Show<&'i ast::Expr<'i>> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Self(obj) = self;
        use ast::Expr::*;
        match obj {
            Term1(t1) => write!(f, "{}", Show(t1)),
            Let(defs, val) => {
                for (name, typ, val) in defs {
                    write!(f, "let {}", name)?;
                    if let Some(typ) = typ {
                        write!(f, ": {}", Show(typ.as_ref()))?;
                    }
                    writeln!(f, " = {}", Show(val.as_ref()))?;
                }
                write!(f, "in {}", Show(val.as_ref()))?;
                Ok(())
            }
            Lambda(name, Some(typ), val) => {
                writeln!(
                    f,
                    "\\({} : {}) -> {}",
                    name,
                    Show(typ.as_ref()),
                    Show(val.as_ref())
                )
            }
            Lambda(name, None, val) => {
                writeln!(f, "\\({}) -> {}", name, Show(val.as_ref()))
            }
        }
    }
}

impl<'i> fmt::Display for Show<&'i ast::Term1<'i>> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Self(obj) = self;
        use ast::Term1::*;
        match obj {
            Term(t) => write!(f, "{}", Show(t)),
            Arrow(Some(name), typ, val) => write!(
                f,
                "forall({} : {}) -> {}",
                name,
                Show(typ.as_ref()),
                Show(val.as_ref())
            ),
            Arrow(None, typ, val) => write!(f, "{} -> {}", Show(typ.as_ref()), Show(val.as_ref())),
            Evaluation(func, t) => write!(f, "({} {})", Show(func.as_ref()), Show(t)),
            Operation(a, op, b) => write!(f, "{} {} {}", Show(a.as_ref()), op, Show(b)),
            Ascribe(term, typ) => write!(f, "{} : {}", Show(term.as_ref()), Show(typ.as_ref())),
            With(term, path, val) => write!(
                f,
                "({}) with {}",
                Show(term.as_ref()),
                Show(ListEntry("=", &(path, val)))
            ),
            IfThenElse(c, a, b) => {
                write!(
                    f,
                    "if {} then {} else {}",
                    Show(c.as_ref()),
                    Show(a.as_ref()),
                    Show(b.as_ref()),
                )
            }
        }
    }
}

impl<'i> fmt::Display for Show<&'i ast::Term<'i>> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Self(obj) = self;
        use ast::Term::*;
        match obj {
            Natural(s) => write!(f, "{}", s),
            Var(name) => write!(f, "{}", name),
            Import(path, Some((guard, Some(fallback)))) => {
                write!(f, "({} {} ? {})", path, guard, fallback)
            }
            Import(path, Some((guard, None))) => write!(f, "({} {})", path, guard),
            Import(path, None) => write!(f, "({})", path),
            Record(fields) => print_list(f, "{", "}", "=", ",", true, fields),
            TypeRecord(fields) => print_list(f, "{", "}", ":", ",", true, fields),
            TypeEnum(fields) => print_list(f, "<", ">", ":", "|", true, fields),
            List(fields) => print_list(f, "[", "]", "", ",", true, fields),
            Expr(expr) => write!(f, "{}", Show(expr.as_ref())),
            Text(n, entries) => {
                write!(f, "(")?;
                let mut first = true;
                for (text, imbue) in entries {
                    match (first, *text, imbue) {
                        (true, "", Some(val)) => write!(f, "({})", Show(val.as_ref()))?,
                        (true, txt, None) => write!(f, "{}", Show(SText(*n, txt)))?,
                        (true, txt, Some(val)) => {
                            write!(f, "{:?} ++ ({})", txt, Show(val.as_ref()))?
                        }
                        (false, "", None) => (),
                        (false, "", Some(val)) => write!(f, "++ ({})", Show(val.as_ref()))?,
                        (false, txt, None) => write!(f, "++ {:?}", txt)?,
                        (false, txt, Some(val)) => {
                            write!(f, "++ {:?} ++ ({})", txt, Show(val.as_ref()))?
                        }
                    }
                    first = false;
                }
                write!(f, ")")?;
                Ok(())
            }
            Negative(n) => write!(f, "({})", n),
            o => panic!("How to show {:?}", o),
        }
    }
}

fn print_list<'i, P>(
    f: &mut fmt::Formatter,
    open: &str,
    close: &str,
    assign: &'i str,
    sep: &str,
    first_sep: bool,
    list: P,
) -> fmt::Result
where
    P: IntoIterator,
    Show<ListEntry<'i, P::Item>>: fmt::Display,
{
    write!(f, "{}", open)?;

    let mut first = true;
    for entry in list {
        if !first || first_sep {
            write!(f, "{} ", sep)?;
        }
        first = false;
        write!(f, "{}", Show(ListEntry(assign, entry)))?;
    }
    write!(f, "{}", close)?;
    Ok(())
}

struct ListEntry<'i, E>(&'i str, E);
struct Path<T>(T);
struct SText<'i>(u8, &'i str);

impl<'i> fmt::Display for Show<ListEntry<'i, &'i (ast::Path<'i>, Box<ast::Expr<'i>>)>> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ListEntry(assign, (name, expr)) = self.0;
        write!(f, "{} {} {}", Show(Path(name)), assign, Show(expr.as_ref()))
    }
}

impl<'i> fmt::Display for Show<ListEntry<'i, &'i (ast::Ident<'i>, Option<Box<ast::Expr<'i>>>)>> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ListEntry(assign, (name, val)) = self.0;
        write!(f, "{}", name)?;
        if let Some(val) = val {
            write!(f, "{} {}", assign, Show(val.as_ref()))?;
        }
        Ok(())
    }
}

impl<'i> fmt::Display for Show<ListEntry<'i, &'i Box<ast::Expr<'i>>>> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ListEntry(_, val) = self.0;
        write!(f, "{}", Show(val.as_ref()))
    }
}

impl<'i> fmt::Display for Show<ListEntry<'i, &'i (&'i ast::Path<'i>, &'i ast::Term<'i>)>> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ListEntry(assign, &(name, expr)) = self.0;
        write!(f, "{} {} {}", Show(Path(name)), assign, Show(expr))
    }
}

impl<'i, P> fmt::Display for Show<Path<&'i P>>
where
    &'i P: IntoIterator,
    <&'i P as IntoIterator>::Item: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Path(path) = self.0;
        let mut sep = "";
        for p in path {
            write!(f, "{}{}", sep, p)?;
            sep = ".";
        }
        Ok(())
    }
}

impl<'i> fmt::Display for Show<SText<'i>> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let SText(style, text) = self.0;
        let mark = match style {
            1 => "\"",
            _ => "''",
        };
        write!(f, "{mark}{text}{mark}", mark = mark, text = text)
    }
}