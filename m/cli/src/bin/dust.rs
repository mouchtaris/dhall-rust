fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    let mut source = String::new();
    std::io::Read::read_to_string(&mut std::io::stdin(), &mut source).unwrap();
    let mut lex = lex::Lex::new(&source);
    let _prog = parse::dhall::ExprParser::new().parse(&mut lex).unwrap();
    Ok(())
}
