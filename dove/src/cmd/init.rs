use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Error;
use http::Uri;
use structopt::StructOpt;

use lang::compiler::dialects::DialectName;

use crate::cmd::Cmd;
use crate::context::{Context, get_context};
use crate::manifest::{DoveToml, MANIFEST};

const PONT_STDLIB: &str =
    r#"{ git = "https://github.com/pontem-network/move-stdlib", tag = "v0.1.2" }"#;

/// Init project command.
#[derive(StructOpt, Debug)]
pub struct Init {
    #[structopt(
        help = "Basic uri to blockchain api.",
        name = "Blockchain API",
        long = "repo",
        short = "r"
    )]
    repository: Option<Uri>,
    #[structopt(
        help = "Account address.",
        name = "address",
        long = "address",
        short = "a"
    )]
    address: Option<String>,
    #[structopt(
        help = "Compiler dialect",
        default_value = "pont",
        name = "Dialect",
        long = "dialect",
        short = "d"
    )]
    dialect: String,

    #[structopt(
        help = "Creates only Dove.toml.",
        name = "minimal",
        long = "minimal",
        short = "m"
    )]
    minimal: bool,
}

impl Init {
    /// Creates a new Init command.
    pub fn new(
        repository: Option<Uri>,
        address: Option<String>,
        dialect: String,
        minimal: bool,
    ) -> Init {
        Init {
            repository,
            address,
            dialect,
            minimal,
        }
    }
}

impl Cmd for Init {
    fn context(&self, project_dir: PathBuf) -> Result<Context, Error> {
        let manifest = DoveToml::default();
        get_context(project_dir, manifest)
    }

    fn apply(self, ctx: Context) -> Result<(), Error> {
        let manifest = ctx.path_for(MANIFEST);
        if manifest.exists() {
            return Err(anyhow!("init cannot be run on existing project."));
        }
        let dialect = DialectName::from_str(&self.dialect)?.get_dialect();

        let name = ctx
            .project_dir
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| anyhow!("Failed to extract directory name."))?;

        if !self.minimal {
            fs::create_dir_all(ctx.path_for(&ctx.manifest.layout.modules_dir))?;
            fs::create_dir_all(ctx.path_for(&ctx.manifest.layout.scripts_dir))?;
            fs::create_dir_all(ctx.path_for(&ctx.manifest.layout.tests_dir))?;
        }

        let mut f = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(manifest)?;

        writeln!(&mut f, "[package]")?;
        writeln!(&mut f, "name = \"{}\"", name)?;

        if let Some(adr) = &self.address {
            dialect.parse_address(adr)?;
            writeln!(&mut f, "account_address = \"{}\"", adr)?;
        }

        if let Some(url) = &self.repository {
            if url.scheme_str() != Some("https") && url.scheme_str() != Some("http") {
                return Err(anyhow!("url must start with http|https"));
            }
            writeln!(&mut f, "blockchain_api = \"{}\"", url)?;
        }

        writeln!(&mut f, "dialect = \"{}\"", self.dialect)?;

        if !self.minimal && dialect.name() == DialectName::Pont {
            write!(
                &mut f,
                r#"
dependencies = [
    {}
]
"#,
                PONT_STDLIB
            )?;
        }
        Ok(())
    }
}
