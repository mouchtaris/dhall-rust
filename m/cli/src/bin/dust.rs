use error::Result;
use std::collections::VecDeque as Deq;

fn main() -> Result<()> {
    pretty_env_logger::init();

    let mut home = std::env::var("HOME")?;
    if !home.ends_with("/") {
        home.push('/');
    }
    home.push_str(".cache/dust/");

    let mut r = resolve::Reservoir::new(home);

    let mut args: Deq<String> = std::env::args().skip(1).collect();
    for arg in std::env::args().skip(1) {
        r.import_file(&arg)?;
    }

    eprintln!("Files:");
    for (f, _) in r.files {
        eprintln!("- {}", f);
    }

    Ok(())
}
