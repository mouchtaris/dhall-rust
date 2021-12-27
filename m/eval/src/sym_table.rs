use super::{Deq, Map, Result};
use ast::Expr;

pub type Value<'i> = Option<Expr<'i>>;

#[derive(Default, Debug)]
pub struct Info<'i> {
    pub value: Value<'i>,
    pub typ: Value<'i>,
    pub scope_id: usize,
}

#[derive(Debug)]
pub struct Scope<'i> {
    name_info: Map<&'i str, Info<'i>>,
}

#[derive(Default)]
pub struct SymTable<'i> {
    scope: Deq<Scope<'i>>,
    mark: usize,
}

impl<'i> SymTable<'i> {
    pub const NONE: &'i Option<ast::Expr<'i>> = &None;

    pub fn next_scope_id(&self) -> usize {
        self.scope.len()
    }

    pub fn scope_id(&self) -> usize {
        self.next_scope_id() - 1
    }

    pub fn scope_offset(&self, sid: usize) -> usize {
        self.scope_id() - sid
    }

    pub fn mark_scope(&mut self) {
        self.mark = self.scope.len();
        log::trace!("mark at: {}", self.scope_id());
    }

    pub fn return_to_marked_scope(&mut self) {
        self.scope.shrink_to(self.mark);
        log::trace!("return to mark: {}", self.scope_id());
    }

    pub fn enter_scope1(&mut self, is_shadow: bool) {
        let scope = Scope {
            name_info: <_>::default(),
        };

        log::trace!(
            "enter {}{}",
            self.next_scope_id(),
            if is_shadow { " (shadow)" } else { "" }
        );
        self.scope.push_front(scope);
    }

    pub fn enter_scope(&mut self) {
        self.enter_scope1(false)
    }

    pub fn exit_scope(&mut self) {
        self.scope.pop_front();
        log::trace!("exit to {}", self.scope_id());
    }

    pub fn add(&mut self, name: &'i str, typ: Value<'i>, val: Value<'i>) {
        let scope_id = self.scope_id();
        let this_scope = self.scope.front_mut().unwrap();
        let nfo_id = this_scope.name_info.len();
        let info = Info {
            value: val,
            typ,
            scope_id,
        };
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

    fn lookup_from(&self, scope_offset: usize, name: &str, mut nscope: u16) -> Result<&Info<'i>> {
        log::trace!(
            "{:4} ({}) Lookup {} >={} @{}",
            line!(),
            self.scope_id(),
            name,
            scope_offset,
            nscope,
        );

        let mut in_scope_id = self.scope.len();
        for scope in self.scope.iter().skip(scope_offset) {
            if let Some(info) = scope.name_info.get(name) {
                if nscope == 0 {
                    log::debug!(
                        "{:4} Lookup {} >={} @{}  ->{} {:?}",
                        line!(),
                        name,
                        scope_offset,
                        nscope,
                        in_scope_id - 1,
                        info
                    );
                    return Ok(info);
                }
                nscope -= 1;
            }
            in_scope_id -= 1;
        }

        panic!("[ERROR] Not found: {}", name)
    }

    pub fn lookup_from_offset(&self, scope_offset: usize, name: &str) -> Result<&Info<'i>> {
        self.lookup_from(scope_offset, name, 0)
    }

    pub fn lookup(&self, name: &str, nscope: u16) -> Result<&Info<'i>> {
        self.lookup_from(0, name, nscope)
    }

    pub fn is_thunk1(&self, starting_scope_id: usize, name: &str, nscope: u16) -> Result<bool> {
        Ok(self
            .lookup_from(starting_scope_id, name, nscope)?
            .value
            .is_none())
    }

    pub fn is_thunk(&self, name: &str, nscope: u16) -> Result<bool> {
        Ok(self.lookup(name, nscope)?.value.is_none())
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
