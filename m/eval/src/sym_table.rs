use super::{Deq, Map, Result};
use ast::Expr;

pub type Value<'i> = Option<Expr<'i>>;

#[derive(Default)]
pub struct Info<'i> {
    pub value: Value<'i>,
    pub typ: Value<'i>,
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
    pub const NONE: &'i Option<ast::Expr<'i>> = &None;

    pub fn enter_scope(&mut self) {
        self.scope.push_front(<_>::default())
    }

    pub fn exit_scope(&mut self) {
        self.scope.pop_front();
    }

    pub fn add(&mut self, name: &'i str, typ: Value<'i>, val: Value<'i>) {
        let v = self
            .scope
            .front_mut()
            .unwrap()
            .name_info
            .entry(name)
            .or_default();
        v.value = val;
        v.typ = typ;
    }

    pub fn lookup(&self, name: &str, scope: u16) -> Result<&Info<'i>> {
        for scope in self.scope.iter().skip(scope as usize) {
            if let Some(info) = scope.name_info.get(name) {
                return Ok(info);
            }
        }

        Err(format!("No found: {}", name).into())
    }

    pub fn is_thunk(&self, name: &str, scope: u16) -> Result<bool> {
        Ok(self.lookup(name, scope)?.value.is_none())
    }

    pub fn copy_value(&self, name: &str, scope: u16) -> Result<ast::Expr<'i>> {
        let info = self.lookup(name, scope)?;
        let val = info
            .value
            .as_ref()
            .map(<_>::to_owned)
            .ok_or_else(|| format!("Name is a thunk (has no value) {}@{}", name, scope))?;
        Ok(val)
    }
}
