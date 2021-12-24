pub const VERSION: &str = "0.0.1";
mod eval;
mod intoz;
mod sym_table;
use {
    error::{bail, Result},
    eval::Eval,
    intoz::Intoz,
    std::collections::{HashMap as Map, VecDeque as Deq},
    sym_table::SymTable,
};

/// [Facade]
///
/// Eval the given item.
///
pub fn eval<'i, E: Eval<'i> + Intoz<ast::Expr<'i>>>(mut e: E) -> Result<ast::Expr<'i>> {
    let mut ctx = Evaluator::new();
    e.eval(&mut ctx)?;
    Ok(e.intoz())
}

/// The context of an evaluation.
pub struct Evaluator<'i> {
    sym_table: SymTable<'i>,
}

impl<'i> Evaluator<'i> {
    /// Create an empty evaluation context.
    pub fn new() -> Self {
        Self {
            sym_table: <_>::default(),
        }
    }
}
