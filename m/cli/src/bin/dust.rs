use error::Result;

fn main() -> Result<()> {
    pretty_env_logger::init();

    let mut buffer = String::new();
    let mut inp = std::io::stdin();
    let prog = parse::parse_read(&mut inp, &mut buffer)?;

    {
        let mut home = std::env::var("HOME")?;
        home.push_str("/.cache/dust");

        let mut r = resolve::Reservoir::new(home);
        r.resolve_from("./", prog)?;

        log::debug!("Discovered Uris:\n{:#?}", r.uris);
    }

    Ok(())
}
