pub const VERSION: &str = "0.0.1";
mod eval;
mod intoz;
mod sym_table;
use {
    error::{bail, Error, Result},
    eval::Eval,
    intoz::{Intoz, Stealr},
    std::collections::{HashMap as Map, VecDeque as Deq},
    sym_table::{Info, SymTable, Value},
};

/// [Facade]
///
/// Eval the given item.
///
pub fn eval<'i, E: Eval<'i> + Intoz<ast::Expr<'i>>>(e: E) -> Result<ast::Expr<'i>> {
    let mut ctx = Evaluator::new();
    e.eval(&mut ctx)
}

/// The context of an evaluation.
pub struct Evaluator<'i> {
    sym_table: SymTable<'i>,
}

impl<'i> Evaluator<'i> {
    /// Create an empty evaluation context.
    pub fn new() -> Self {
        let mut ctx = Self {
            sym_table: <_>::default(),
        };
        ctx.init();
        ctx
    }

    fn init(&mut self) {
        let Self { sym_table } = self;
        sym_table.enter_scope();

        // Install core
        sym_table.add("Type", SymTable::NONE, SymTable::NONE);
    }

    pub fn enter_scope(&mut self) {
        self.sym_table.enter_scope()
    }
    pub fn exit_scope(&mut self) {
        self.sym_table.exit_scope()
    }
    pub fn assign<E, T>(&mut self, name: &'i str, typ: T, val: E)
    where
        E: Stealr<Value<'i>>,
        T: Stealr<Value<'i>>,
    {
        self.sym_table.add(name, typ, val)
    }
    pub fn lookup(&self, name: &str, scope: u8) -> Option<&Info<'i>> {
        self.sym_table.lookup(name, scope)
    }
}
