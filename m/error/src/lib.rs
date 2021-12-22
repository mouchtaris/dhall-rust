pub const VERSION: &str = "0.0.1";

use std::{env, io, result};

#[derive(thiserror::Error, Debug)]
pub enum Source {
    #[error("io: {:?}", .0)]
    Io(#[from] io::Error),
    #[error("env var: {:?}", .0)]
    Var(#[from] env::VarError),
    #[error("parse: {:?}", .0)]
    Lalrpop(parse::ParseError<'static>),
    #[error("{}", .0)]
    Any(String),
}

#[derive(Debug)]
pub struct Error {
    source: Source,
}

pub type Result<T> = result::Result<T, Error>;

#[macro_export]
macro_rules! bail {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {
        return Err(Error::any(format!($fmt $(, $arg)*)))
    }
}

impl Error {
    pub fn new(source: Source) -> Self {
        Self { source }
    }

    pub fn any<M: Into<String>>(msg: M) -> Self {
        Self::new(Source::Any(msg.into()))
    }
}

impl<T> From<T> for Error
where
    Source: From<T>,
{
    fn from(t: T) -> Self {
        Self::new(t.into())
    }
}

impl<'i, E> From<parse::ParseErrorE<'i, E>> for Error {
    fn from(e: parse::ParseErrorE<'i, E>) -> Self {
        let e = e.map_token(|t| t.set_val("")).map_error(|_| parse::Error);
        Self::new(Source::Lalrpop(e))
    }
}

impl From<String> for Error {
    fn from(e: String) -> Self {
        Self::new(Source::Any(e))
    }
}
