pub const VERSION: &str = "0.0.1";

use error::Result;
mod resolve;

use resolve::Resolve;

#[derive(Default)]
pub struct Reservoir {
    pub uris: ast::Deq<String>,
    pub path: ast::Deq<String>,
    pub output_dir: String,
}

impl Reservoir {
    pub fn new(output_dir: String) -> Self {
        Self {
            uris: <_>::default(),
            path: <_>::default(),
            output_dir,
        }
    }

    pub fn resolve_from<P>(&mut self, path: P, mut tree: ast::Expr) -> Result<()>
    where
        P: Into<String>,
    {
        self.path.push_back(path.into());
        tree.resolve(self)?;
        self.fetch_http()?;
        Ok(())
    }

    pub fn register<P: Into<String>>(&mut self, path: P) {
        self.uris.push_back(path.into());
    }

    fn fetch_http(&mut self) -> Result<()> {
        use std::process::{Command, Stdio};

        let mut cmd = Command::new("curl");

        cmd.stdin(Stdio::piped());

        let opts = [
            "--no-progress-meter",
            "--create-dirs",
            "--output-dir",
            &self.output_dir,
        ];
        let args = || {
            opts.iter().cloned().map(String::from).chain(
                self.uris
                    .iter()
                    .filter_map(|s| {
                        if s.starts_with("http") {
                            Some(vec![s.clone(), format!("--output"), s.clone()])
                        } else {
                            None
                        }
                    })
                    .flatten(),
            )
        };

        log::debug!("Curling: {:?}", args().collect::<Vec<_>>());
        cmd.args(args());

        let mut proc = cmd.spawn()?;
        let status = proc.wait()?;

        if !status.success() {
            return Err(format!("curl failed: {:?}", status).into());
        }

        log::info!("Curled {} files into {}", (args().count() - 4) / 3, &self.output_dir);

        Ok(())
    }
}
