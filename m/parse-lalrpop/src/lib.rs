pub const VERSION: &str = "0.0.1";

#[derive(Debug)]
pub struct Error;

pub type ParseErrorE<'i, E> = lalrpop_util::ParseError<usize, ast::Token<'i>, E>;
pub type ResultT<'i, T, E> = std::result::Result<T, ParseErrorE<'i, E>>;

pub type ParseError<'i> = ParseErrorE<'i, Error>;
pub type Result<'i, T> = ResultT<'i, T, Error>;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub dhall);
