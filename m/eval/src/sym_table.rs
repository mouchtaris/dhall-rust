use super::{Deq, Map, Result};
use ast::Expr;

pub type Value<'i> = Option<Expr<'i>>;

#[derive(Default, Debug)]
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
        let scope_id = self.scope.len() - 1;
        let this_scope = self.scope.front_mut().unwrap();
        let nfo_id = this_scope.name_info.len();
        let info = Info { value: val, typ };
        log::debug!(
            "{:4} Define {}.{}.{}: {:?}",
            line!(),
            name,
            scope_id,
            nfo_id,
            info
        );

        if let Some(prev_info) = this_scope.name_info.insert(name, info) {
            log::trace!(
                "{:4} Shadowing {}.{}.{}: {:?}",
                line!(),
                name,
                scope_id,
                nfo_id,
                prev_info
            );
        }
    }

    pub fn add_thunk(&mut self, name: &'i str) {
        self.add(name, None, None)
    }

    pub fn lookup(&self, name: &str, nscope: u16) -> Result<&Info<'i>> {
        let mut depth = nscope;
        for scope in self.scope.iter().skip(nscope as usize) {
            if let Some(info) = scope.name_info.get(name) {
                log::debug!(
                    "{:4} Lookup {}.{}.{}: {:?}",
                    line!(),
                    name,
                    nscope,
                    depth,
                    info
                );
                return Ok(info);
            }
            depth += 1;
        }

        Err(format!("Not found: {}", name).into())
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
