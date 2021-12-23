pub const VERSION: &str = "0.0.1";

use ast::{Deq, Ident, RecordData, Term, Term1, TextEntry, TypeEnumData, Val};

pub mod new {
    use super::*;

    macro_rules! impl_report {
        ($name:ident -> $t:ident $l:lifetime : $treq:ty = $m:expr) => {
            pub fn $name<$l, T: Into<$treq>>(inp: T) -> $t<$l> {
                let t = $m(inp.into());
                log::trace!("Reduce {:?}", t);
                t
            }
        };
    }

    pub type BTerm<'i> = Box<Term<'i>>;

    pub mod term {
        use super::*;
        impl_report! {
        var -> Term 's
            : (&'s str, &'s str)
            = |(name, scope)| Term::Var(name, scope) }

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
            : (bool, &'s str)
            = |(is_neg, val)| Term::Integer(is_neg, val) }

        impl_report! {
        double -> Term 's
            : &'s str
            = |s: &'s str| Term::Double(s) }

        impl_report! {
        project -> Term 's
            : (Box<Term<'s>>, Deq<Term1<'s>>)
            = |(term, names)| Term::Project(1, term, names) }

        impl_report! {
        select -> Term 's
            : (Box<Term<'s>>, Deq<Term1<'s>>)
            = |(term, names)| Term::Project(2, term, names) }

        impl_report! {
        import -> Term 's
            : (&'s str, Option<&'s str>, Option<&'s str>, Option<(&'s str, Option<&'s str>)>)
            = |(path, guard, as_, fall)| Term::Import { path, guard, as_, fall } }

        // fn select2<T: Analogous<(Term<'i>, Deq<Term1<'i>>)>>(t: T) -> Term<'i> {
        // }
    }
}
