pub const VERSION: &str = "0.0.1";

use ast::{Deq, Ident, RecordData, Term, TextEntry, TypeEnumData, Val};

pub mod new {
    use super::*;

    macro_rules! impl_report {
        ($name:ident -> $t:ident $l:lifetime : $treq:ty = $m:expr) => {
            pub fn $name<$l, T: Into<$treq>>(inp: T) -> $t<$l> {
                let t = $m(inp.into());
                log::trace!("{:?}", t);
                t
            }
        };
    }

    pub type BTerm<'i> = Box<Term<'i>>;

    pub mod term {
        use super::*;
        impl_report! {
        var -> Term 's
            : &'s str
            = |inp| Term::Var(inp) }

        impl_report! {
        field_access -> Term 's
            : (Box<Term<'s>>, Ident<'s>)
            = |(t, i)| Term::FieldAccess(t, i) }

        impl_report! {
        list -> Term 's
            : Deq<Val<'s>>
            = Term::List }

        impl_report! {
        record -> Term 's
            : RecordData<'s>
            = Term::Record }

        impl_report! {
        type_record -> Term 's
            : RecordData<'s>
            = Term::TypeRecord }

        impl_report! {
        type_enum -> Term 's
            : TypeEnumData<'s>
            = Term::TypeEnum }

        impl_report! {
        expr -> Term 's
            : Val<'s>
            = Term::Expr }

        impl_report! {
        text -> Term 's
            : (u8, Deq<TextEntry<'s>>)
            = |(s, t)| Term::Text(s, t) }

        impl_report! {
            integer -> Term 's
                : &'s str
                = |s: &'s str| {
                        let is_neg = s.starts_with("-");
                        let rest_n = if is_neg { 1 } else { 0 };
                        let n = &s[rest_n..];
                        Term::Integer(is_neg, n)
                }
        }

        impl_report! {
        project -> Term 's
            : (Box<Term<'s>>, Deq<Ident<'s>>)
            = |(term, names)| Term::Project(1, term, names) }

        impl_report! {
        select -> Term 's
            : (Box<Term<'s>>, Deq<Ident<'s>>)
            = |(term, names)| Term::Project(2, term, names) }
    }
}
