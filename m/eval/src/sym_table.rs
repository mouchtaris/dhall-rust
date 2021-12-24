use super::{Deq, Intoz, Map, Result};

pub type Value<'i> = Option<ast::Expr<'i>>;

#[derive(Default)]
pub struct Info<'i> {
    pub value: Value<'i>,
}

#[derive(Default)]
pub struct Scope<'i> {
    name_info: Map<&'i str, Info<'i>>,
}

#[derive(Default)]
pub struct SymTable<'i> {
    scope: Deq<Scope<'i>>,
}

impl<'i> SymTable<'i> {
    pub fn enter_scope(&mut self) {
        log::warn!("todo::enter_scope {} {}", file!(), line!());
    }

    pub fn exit_scope(&mut self) {
        log::warn!("todo::exit_scope {} {}", file!(), line!());
    }

    pub fn add<E: Intoz<Value<'i>>>(&mut self, name: &'i str, val: E) {
        log::warn!("todo::add {} {}", file!(), line!());
    }
}
