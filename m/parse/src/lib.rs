pub const VERSION: &str = "0.0.1";

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub dhall);

#[derive(Debug)]
pub struct Error;
