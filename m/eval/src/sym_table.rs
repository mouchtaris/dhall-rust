use super::{Deq, Map};
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

    pub fn lookup(&self, name: &str, mut scope: u8) -> Option<&Info<'i>> {
        for Scope { name_info } in &self.scope {
            if scope > 0 {
                scope -= 1;
                continue;
            }

            let cell = name_info.get(name);

            if cell.is_some() {
                return cell;
            }
        }
        None
    }
}
