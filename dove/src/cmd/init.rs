use anyhow::Error;
use http::Uri;
use crate::manifest::MANIFEST;
use std::fs;
use crate::cmd::Cmd;
use crate::context::{Context, create_context};
use structopt::StructOpt;
use std::fs::OpenOptions;
use std::io::Write;

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
        name = "Dialect",
        long = "dialect",
        short = "d"
    )]
    dialect: Option<String>,
}

impl Init {
    /// Creates a new Init command.
    pub fn new(
        repository: Option<Uri>,
        address: Option<String>,
        dialect: Option<String>,
    ) -> Init {
        Init {
            repository,
            address,
            dialect,
        }
    }
}

impl Cmd for Init {
    fn context(&self) -> Result<Context, Error> {
        create_context()
    }

    fn apply(self, ctx: Context) -> Result<(), Error> {
        let manifest = ctx.path_for(MANIFEST);
        if manifest.exists() {
            return Err(anyhow!("init cannot be run on existing project."));
        }

        let name = ctx
            .project_dir
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| anyhow!("Failed to extract directory name."))?;
        fs::create_dir_all(ctx.path_for(&ctx.manifest.layout.module_dir))?;
        fs::create_dir_all(ctx.path_for(&ctx.manifest.layout.script_dir))?;
        fs::create_dir_all(ctx.path_for(&ctx.manifest.layout.tests_dir))?;

        let mut f = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(manifest)?;

        writeln!(&mut f, "[package]")?;
        writeln!(&mut f, "name = \"{}\"", name)?;

        if let Some(adr) = &self.address {
            writeln!(&mut f, "account_address = \"{}\"", adr)?;
        }

        if let Some(url) = &self.repository {
            writeln!(&mut f, "blockchain_api = \"{}\"", url)?;
        }

        if let Some(dialect) = &self.dialect {
            writeln!(&mut f, "dialect = \"{}\"", dialect)?;
        }

        write!(
            &mut f,
            "\
dependencies = [
    {{ git = \"https://github.com/pontem-network/move-stdlib\" }}
]
"
        )?;

        Ok(())
    }
}
