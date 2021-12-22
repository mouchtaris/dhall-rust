pub const VERSION: &str = "0.0.1";

use std::io;

pub use parse_lalrpop::{dhall, Error, ParseError, ParseErrorE, Result, ResultT};

pub fn parse_str(inp: &str) -> Result<ast::Expr> {
    let mut lex = lex::Lex::new(inp);
    dhall::ExprParser::new().parse(&mut lex)
}

pub fn parse_read<'i, R>(inp: &mut R, buf: &'i mut String) -> ResultT<'i, ast::Expr<'i>, io::Error>
where
    R: io::Read,
{
    inp.read_to_string(buf)?;
    let ast = parse_str(buf)
        .map_err(|err| err.map_error(|_| io::Error::new(io::ErrorKind::Other, "oh no!")))?;
    Ok(ast)
}
