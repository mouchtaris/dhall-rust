pub const VERSION: &str = "0.0.1";

use std::{io, result};

#[derive(thiserror::Error, Debug)]
pub enum Source {
    #[error("io: {:?}", .source)]
    Io {
        #[from]
        source: io::Error,
    },
    #[error("parse: {:?}", .0)]
    Lalrpop(lalrpop_util::ParseError<usize, ast::Token<'static>, parse::Error>),
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

impl<'i> From<lalrpop_util::ParseError<usize, ast::Token<'i>, parse::Error>> for Error {
    fn from(e: lalrpop_util::ParseError<usize, ast::Token<'i>, parse::Error>) -> Self {
        let e = e.map_token(|t| t.set_val(""));
        Self::new(Source::Lalrpop(e))
    }
}
