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
            "--dbg_show_ast" => {
                opt_show = false;
                opt_dbg_show_ast = true;
            }
            "--dbg_list_files" => {
                opt_show = false;
                opt_dbg_list_files = true;
            }
            "--resolve" => r.enable_resolve = true,
            "--fetch" => r.enable_fetch = true,
            "--no_resolve" => r.enable_resolve = false,
            "--help" => opt_help = true,
            _ => opt_input_file_path = arg,
        }
    }

    if opt_help {
        eprintln!(
            r#"
            "--show" => opt_show = true,
            "--no_show" => opt_show = false,
            "--dbg_show_ast" => opt_dbg_show_ast = true,
            "--dbg_ast" => opt_dbg_list_files = true,
            "--resolve" => r.enable_resolve = true,
            "--no_resolve" => r.enable_resolve = false,
            "--help" => opt_help = true,
        "#
        );
        return Ok(());
    }

    r.import_file(&opt_input_file_path)?;

    if opt_dbg_list_files {
        eprintln!("Files:");
        for (f, _) in &r.files {
            eprintln!("- {}", f);
        }
    }

    let source = if opt_show || opt_dbg_show_ast {
        r.files.get(&opt_input_file_path)
    } else {
        None
    };

    if let Some(source) = source {
        let ast = parse::parse_str(source)?;

        if opt_show {
            println!("{}", show::Show(&ast));
        }
        if opt_dbg_show_ast {
            eprintln!("{:#?}", &ast);
        }
    }

    Ok(())
}
