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
            Evaluation(func, t) => write!(f, "{} {}", Show(func.as_ref()), Show(t)),
            Operation(a, op, b) => write!(f, "{} {} {}", Show(a.as_ref()), op, Show(b.as_ref())),
            Ascribe(term, typ) => write!(f, "{} : {}", Show(term.as_ref()), Show(typ.as_ref())),
            With(term, path, val) => write!(
                f,
                "{} with {}",
                Show(term.as_ref()),
                Show(ListEntry("=", (path, val)))
            ),
            IfThenElse(c, a, b) => {
                write!(
                    f,
                    "(if {} then {} else {})",
                    Show(c.as_ref()),
                    Show(a.as_ref()),
                    Show(b.as_ref()),
                )
            }
            Construct(term, data) => {
                write!(
                    f,
                    "{}::{}",
                    Show(term.as_ref()),
                    Show(ShowList(SHOW_LIST_STYLE_REC, data))
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
            &Integer(v) => write!(f, "{}", v),
            Embed(code) => writeln!(f, "{}", code),
            &Var(name, n) => {
                let ear = if lex::is_keyword(name) { "`" } else { "" };

                write!(f, "{}{}", ear, name)?;
                if *n != 0 {
                    write!(f, "@{}", n)?;
                }
                write!(f, "{}", ear)?;

                Ok(())
            }
            Double(n) => write!(f, "{}", n),
            FieldAccess(term, field) => {
                write!(f, "{}.{}", Show(term.as_ref()), field)
            }
            Import {
                path,
                as_,
                guard,
                fall,
            } => {
                write!(f, "{}", path)?;
                if let Some(guard) = guard {
                    write!(f, " {}", guard)?;
                }
                if let Some(as_) = as_ {
                    write!(f, " as {}", as_)?;
                }
                if let Some((fall, fall_as)) = fall {
                    write!(f, " ? {}", fall)?;
                    if let Some(as_) = fall_as {
                        write!(f, " as {}", as_)?;
                    }
                }
                Ok(())
            }
            Path(path) => print_list(f, SHOW_LIST_STYLE_PATH, path),
            Record(fields) => print_list(f, SHOW_LIST_STYLE_REC, fields),
            TypeRecord(fields) => print_list(f, SHOW_LIST_STYLE_TYPEREC, fields),
            TypeEnum(fields) => print_list(f, SHOW_LIST_STYLE_TYPEENUM, fields),
            List(fields) => print_list(f, SHOW_LIST_STYLE_LIST, fields),
            Expr(expr) => write!(f, "({})", Show(expr.as_ref())),
            Merge(term, val) => {
                write!(
                    f,
                    "(merge {} {})",
                    Show(ShowList(SHOW_LIST_STYLE_REC, term)),
                    Show(val.as_ref())
                )
            }
            Text(n, entries) => write!(f, "{}", Show(SText(*n, entries))),
            Project(style, term, names) => {
                let list_style = match style {
                    1 => SHOW_LIST_STYLE_PROJECTION,
                    _ => SHOW_LIST_STYLE_SELECTION,
                };
                write!(
                    f,
                    "{}.{}",
                    Show(term.as_ref()),
                    Show(ShowList(list_style, names))
                )
            }
        }
    }
}

fn print_list<'i, P>(f: &mut fmt::Formatter, style: ShowListStyle<'i>, list: P) -> fmt::Result
where
    P: IntoIterator,
    Show<ListEntry<'i, P::Item>>: fmt::Display,
{
    let ShowListStyle(open, close, assign, sep, empty, first_sep) = style;

    write!(f, "{} ", open)?;

    let mut first = true;

    for entry in list {
        if !first || first_sep {
            write!(f, "{} ", sep)?;
        }
        first = false;
        write!(f, "{} ", Show(ListEntry(assign, entry)))?;
    }

    if first {
        write!(f, "{} ", empty)?;
    }

    write!(f, "{}", close)?;
    Ok(())
}

#[derive(Copy, Clone)]
struct ShowListStyle<'s>(&'s str, &'s str, &'s str, &'s str, &'s str, bool);
const SHOW_LIST_STYLE_PATH: ShowListStyle = ShowListStyle("", "", "", ".", "", false);
const SHOW_LIST_STYLE_REC: ShowListStyle = ShowListStyle("{", "}", "=", ",", "=", true);
const SHOW_LIST_STYLE_TYPEREC: ShowListStyle = ShowListStyle("{", "}", ":", ",", "", true);
const SHOW_LIST_STYLE_TYPEENUM: ShowListStyle = ShowListStyle("<", ">", ":", "|", "", true);
const SHOW_LIST_STYLE_LIST: ShowListStyle = ShowListStyle("[", "]", "", ",", "", true);
const SHOW_LIST_STYLE_PROJECTION: ShowListStyle = ShowListStyle("{", "}", "", ",", "", true);
const SHOW_LIST_STYLE_SELECTION: ShowListStyle = ShowListStyle("(", ")", "", ",", "", false);

struct ShowList<'i, P>(ShowListStyle<'i>, &'i P);
struct ListEntry<'i, E>(&'i str, E);
struct Path<T>(T);
struct SText<'i>(u8, &'i ast::Deq<ast::TextEntry<'i>>);

impl<'i> fmt::Display for Show<ListEntry<'i, &'i (ast::Path<'i>, Box<ast::Expr<'i>>)>> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ListEntry(assign, (name, expr)) = self.0;
        write!(f, "{} {} {}", Show(Path(name)), assign, Show(expr.as_ref()))
    }
}

impl<'i> fmt::Display for Show<ListEntry<'i, (&'i ast::Path<'i>, &'i Box<ast::Term1<'i>>)>> {
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

impl<'i, T> fmt::Display for Show<ListEntry<'i, &'i Box<T>>>
where
    Show<&'i T>: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ListEntry(_, val) = self.0;
        write!(f, "{}", Show(val.as_ref()))
    }
}

impl<'i> fmt::Display for Show<ListEntry<'i, &'i (&'i ast::Path<'i>, &'i ast::Val<'i>)>> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ListEntry(assign, &(name, expr)) = self.0;
        let expr = expr.as_ref();
        write!(f, "{} {} {}", Show(Path(name)), assign, Show(expr))
    }
}

impl<'i> fmt::Display for Show<ListEntry<'i, &'i ast::Ident<'i>>> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ListEntry(_, name) = self.0;
        write!(f, "{}", name)
    }
}

impl<'i> fmt::Display for Show<ListEntry<'i, &'i ast::Term1<'i>>> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ListEntry(_, name) = self.0;
        write!(f, "{}", Show(name))
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
        let SText(style, entries) = self.0;
        let mark = match style {
            1 => "\"",
            _ => "''",
        };

        write!(f, "{}", mark)?;

        for (text, imbue) in entries {
            if !text.is_empty() {
                write!(f, "{}", text)?;
            }
            if let Some(val) = imbue {
                write!(f, "${{ {} }}", Show(val.as_ref()))?;
            }
        }

        write!(f, "{}", mark)?;

        Ok(())
    }
}

impl<'i, P, I> fmt::Display for Show<ShowList<'i, P>>
where
    &'i P: IntoIterator<Item = I>,
    Show<ListEntry<'i, I>>: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let &ShowList(style, list) = &self.0;
        print_list(f, style, list)
    }
}
