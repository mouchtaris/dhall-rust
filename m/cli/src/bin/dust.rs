use error::Result;

fn main() -> Result<()> {
    pretty_env_logger::init();

    let mut home = std::env::var("HOME")?;
    if !home.ends_with("/") {
        home.push('/');
    }
    home.push_str(".cache/dust/");

    let mut r = resolve::Reservoir::new(home);

    let mut opt_show = true;
    let mut opt_dbg_list_files = false;
    let mut opt_dbg_show_ast = false;
    let mut opt_help = false;
    let mut opt_input_file_path = format!("/dev/stdin");
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--show" => opt_show = true,
            "--no_show" => opt_show = false,
            "--ast" => {
                opt_show = false;
                opt_dbg_show_ast = true;
            }
            "--files" => {
                opt_show = false;
                opt_dbg_list_files = true;
            }
            "--resolve" => r.enable_resolve = true,
            "--no_resolve" => r.enable_resolve = false,
            "--fetch" => r.enable_fetch = true,
            "--no_fetch" => r.enable_fetch = false,
            "--help" => opt_help = true,
            _ => opt_input_file_path = arg,
        }
    }

    if opt_help {
        let helps = &[
            "| --- Command --- | --- Default ---        |",
            "  --[no_]resolve     true                    Resolve imports",
            "  --[no_]fetch       false                   Download http imports to ~/.cache/dust (requires resolve)",
            "  --[no_]show        !(resolve || fetch)     Show dhall.",
            "  --help             false                   Show help and exit.",
            "  --ast              false                   Show the dhall AST. Turns show off.",
            "  --files            false                   Show the loaded files. Turns show off.",
        ];
        helps.iter().for_each(|s| eprintln!("{}", s));
        return Ok(());
    }

    r.import_file(&opt_input_file_path)?;

    if opt_dbg_list_files {
        eprintln!("Files:");
        for (f, (o, _)) in r.files() {
            eprintln!("- {} {}", o, f);
        }
    }

    if opt_show || opt_dbg_show_ast {
        if r.enable_resolve {
            println!("{}", resolve::Importer(&mut r));
            println!("in `{}`", &opt_input_file_path);
        } else if let Some(source) = r.file(&opt_input_file_path) {
            let ast = parse::parse_str(source)?;
            if opt_show {
                println!("{}", show::Show(&ast));
            }
            if opt_dbg_show_ast {
                eprintln!("{:#?}", &ast);
            }
        }
    }

    Ok(())
}
