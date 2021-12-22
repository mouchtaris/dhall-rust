pub const VERSION: &str = "0.0.1";

use {
    error::Result,
    std::{
        collections::{hash_map::HashMap as Map, hash_set::HashSet as Set},
        fs,
    },
};

mod resolve;
use resolve::Resolve;

#[derive(Default)]
pub struct Reservoir {
    // across-state
    fetched_uris: Set<String>,
    output_dir: String,
    files: Map<String, String>,
    // iteration-state
    uris: Set<String>,
    path: String,
}

impl Reservoir {
    pub fn new(output_dir: String) -> Self {
        Self {
            uris: <_>::default(),
            fetched_uris: <_>::default(),
            path: <_>::default(),
            files: <_>::default(),
            output_dir,
        }
    }

    pub fn import_file<P: AsRef<str>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref();
        log::debug!("importing {}", path);

        if self.files.contains_key(path) {
            return Ok(());
        }

        let prefix = if is_http(path) { &self.output_dir } else { "" };
        let real_path = format!("{}{}", prefix, path);

        log::debug!("access: {}", real_path);
        let mut file = fs::File::open(&real_path)?;

        let mut read_buffer = String::new();
        read_buffer.clear();
        let mut ast = parse::parse_read(&mut file, &mut read_buffer)?;

        self.uris.clear();
        let base_path = path;
        let mut path = String::new();

        ast.visit_register(|p| {
            path.push_str(p);
            path_resolve(&base_path, &mut path);
            log::trace!("[register] resolved as {}", path);

            if is_http(&path) {
                log::trace!("register url [{:02}]", self.uris.len());
                self.uris.insert(path.clone());
            }

            path.clear();
            Ok(())
        })?;

        self.fetch_http()?;

        ast.visit_import(|p, _| {
            path.push_str(p);
            path_resolve(&base_path, &mut path);
            log::trace!("[import] resolved as {}", path);

            self.import_file(path.clone())?;

            path.clear();
            Ok(())
        })?;

        self.files.insert(base_path.to_owned(), read_buffer);
        Ok(())
    }

    fn fetch_http(&mut self) -> Result<()> {
        use std::process::{Command, Stdio};

        let mut cmd = Command::new("curl");

        cmd.stdin(Stdio::piped());

        let mut fetch_count = self.fetched_uris.len();
        let opts = [
            "--no-progress-meter",
            "--create-dirs",
            "--output-dir",
            &self.output_dir,
        ];
        let mut args: Vec<String> = opts.iter().map(|&s| s.to_owned()).collect();
        for uri in &self.uris {
            if is_http(uri) && !self.fetched_uris.contains(uri) {
                args.push(uri.clone());
                args.push(format!("--output"));
                args.push(uri.clone());
                self.fetched_uris.insert(uri.clone());
            }
        }
        fetch_count = self.fetched_uris.len() - fetch_count;
        cmd.args(args);

        log::debug!(
            "Curling: {:?} + {} uris into {}",
            opts,
            fetch_count,
            &self.output_dir
        );

        let mut proc = cmd.spawn()?;
        let status = proc.wait()?;

        if !status.success() {
            return Err(format!("curl failed: {:?}", status).into());
        }

        log::info!("Curled: {} uris into {}", fetch_count, &self.output_dir);

        Ok(())
    }
}

const HTTP: &str = "http://";
const HTTPS: &str = "https://";

fn is_http(path: &str) -> bool {
    path.starts_with(HTTP) || path.starts_with(HTTPS)
}

fn is_absolute(path: &str) -> bool {
    is_http(path) || path.starts_with("/")
}

fn dir_base(path: &str) -> (&str, &str) {
    path.split_at(path.rfind('/').map(|x| x + 1).unwrap_or(0))
}

fn path_resolve(base: &str, path: &mut String) {
    if !is_absolute(path) {
        let (dir, _) = dir_base(base);
        path.insert_str(0, dir);
    }
}
