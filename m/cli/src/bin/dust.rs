use error::Result;

fn main() -> Result<()> {
    pretty_env_logger::init();

    let mut home = std::env::var("HOME")?;
    if !home.ends_with("/") {
        home.push('/');
    }
    home.push_str(".cache/dust/");

    let mut r = resolve::Reservoir::new(home);

    for arg in std::env::args().skip(1) {
        r.import_file(&arg)?;
    }

    Ok(())
}
