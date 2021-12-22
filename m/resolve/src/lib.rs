pub const VERSION: &str = "0.0.1";

use error::{bail, Error, Result};

#[derive(Default)]
pub struct Reservoir {}

pub trait Resolve {
    fn resolve(&mut self, reservoir: &mut Reservoir) -> Result<()>;
}

impl<'i> Resolve for ast::Expr<'i> {
    fn resolve(&mut self, r: &mut Reservoir) -> Result<()> {
        use ast::Expr::*;
        match self {
            Term1(t1) => t1.resolve(r),
            o => bail!("How to resolve expr {:?}", o),
        }
    }
}

impl<'i> Resolve for ast::Term1<'i> {
    fn resolve(&mut self, reservoir: &mut Reservoir) -> Result<()> {
        use ast::Term1::*;
        match self {
            o => bail!("How to resolve term1 {:?}", o),
        }
    }
}

impl<'i> Resolve for ast::Term<'i> {
    fn resolve(&mut self, reservoir: &mut Reservoir) -> Result<()> {
        use ast::Term::*;
        match self {
            o => bail!("How to resolve term {:?}", o),
        }
    }
}
