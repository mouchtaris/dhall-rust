use error::Result;

fn main() -> Result<()> {
    pretty_env_logger::init();
    let mut source = String::new();
    std::io::Read::read_to_string(&mut std::io::stdin(), &mut source)?;
    let mut lex = lex::Lex::new(&source);
    let mut prog = parse::dhall::ExprParser::new().parse(&mut lex)?;
    println!("{}", show::Show(&prog));
    // log::debug!("{:#?}", prog);

    {
        use resolve::Resolve;

        let mut r = resolve::Reservoir::default();
        prog.resolve(&mut r)?;

        log::debug!("Discovered Uris:\n{:#?}", r.uris);
    }

    Ok(())
}
