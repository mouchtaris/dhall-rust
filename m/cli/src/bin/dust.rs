use error::Result;

fn main() -> Result<()> {
    pretty_env_logger::init();

    let mut source = String::new();
    let mut inp = std::io::stdin();
    let mut prog = parse::parse_read(&mut inp, &mut source)?;
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
