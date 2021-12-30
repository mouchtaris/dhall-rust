pub const VERSION: &str = "0.0.1";
mod a_subst;
mod eval;
mod sym_table;
pub use {
    a_subst::ASubstitution,
    error::{bail, Error, Result},
    eval::{ctx, Context, Ctx, Eval},
    show::Show,
    std::collections::{
        hash_map::{Entry, HashMap as Map},
        hash_set::HashSet as Set,
        VecDeque as Deq,
    },
    sym_table::{Info, SymTable, Value},
};

pub fn eval<'i, E>(ctx: Ctx<'i>, expr: &mut E) -> Result<Ctx<'i>>
where
    E: Eval<'i>,
{
    expr.eval(ctx)
}

pub trait AsImm {
    fn as_imm(&mut self) -> &Self {
        self
    }
}
impl<T> AsImm for T {}
