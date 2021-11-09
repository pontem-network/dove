// use std::fs;
// use std::fs::OpenOptions;
// use std::io::Write;
// use std::path::PathBuf;
// use std::str::FromStr;
//
// use anyhow::Error;
// use http::Uri;
use structopt::StructOpt;
//
// use lang::compiler::dialects::DialectName;
//
// use crate::cmd::Cmd;
// use crate::context::{Context, get_context};
// use crate::manifest::{DoveToml, MANIFEST};
// use lazy_static::lazy_static;
// use regex::Regex;
//
// use crate::{stdoutln, PONT_STDLIB_URL, PONT_STDLIB_VERSION};
// use crate::stdout::colorize::good;

use crate::cmd::Cmd;
use crate::context::Context;

/// Init project command.
#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
pub struct Init {
    #[structopt(
        help = "Creates only Dove.toml.",
        name = "minimal",
        long = "minimal",
        short = "m"
    )]
    minimal: bool,
    #[structopt(long, hidden = true)]
    color: Option<String>,
}

impl Init {
    /// Creates a new Init command.
    pub fn new(
        minimal: bool,
    ) -> Init {
        Init {
            minimal,
            color: None,
        }
    }
}

impl Cmd for Init {
    fn apply(&self, ctx: Context) -> anyhow::Result<()> where Self: Sized {
        todo!()
    }
//     fn context(&self, project_dir: PathBuf) -> Result<Context, Error> {
//         let manifest = DoveToml::default();
//         get_context(project_dir, manifest)
//     }
//
//     fn apply(self, ctx: Context) -> Result<(), Error> {
//         let manifest = ctx.path_for(MANIFEST);
//         if manifest.exists() {
//             return Err(anyhow!("init cannot be run on existing project."));
//         }
//         let dialect = DialectName::from_str(&self.dialect)?.get_dialect();
//
//         let name = ctx
//             .project_dir
//             .file_name()
//             .and_then(|name| name.to_str())
//             .ok_or_else(|| anyhow!("Failed to extract directory name."))?;
//
//         if !is_valid_name(name) {
//             return Err(anyhow!(
//                 "Invalid project name. Allowed symbols a-z, A-Z, 0-9,_,-"
//             ));
//         }
//
//         if !self.minimal {
//             stdoutln!(
//                 "Creating default directories(to omit those, use --minimal): \n\
//                 \t./modules\n\
//                 \t./scripts\n\
//                 \t./tests"
//             );
//             fs::create_dir_all(ctx.path_for(&ctx.manifest.layout.modules_dir))?;
//             fs::create_dir_all(ctx.path_for(&ctx.manifest.layout.scripts_dir))?;
//             fs::create_dir_all(ctx.path_for(&ctx.manifest.layout.tests_dir))?;
//         }
//
//         stdoutln!("Generating default Dove.toml file...");
//         let mut f = OpenOptions::new()
//             .create(true)
//             .read(true)
//             .write(true)
//             .open(manifest)?;
//
//         writeln!(&mut f, "[package]")?;
//         writeln!(&mut f, "name = \"{}\"", name)?;
//
//         if let Some(adr) = &self.address {
//             dialect.parse_address(adr)?;
//             writeln!(&mut f, "account_address = \"{}\"", adr)?;
//         }
//
//         if let Some(url) = &self.repository {
//             if url.scheme_str() != Some("https") && url.scheme_str() != Some("http") {
//                 return Err(anyhow!("url must start with http|https"));
//             }
//             writeln!(&mut f, "blockchain_api = \"{}\"", url)?;
//         }
//
//         writeln!(&mut f, "dialect = \"{}\"", self.dialect)?;
//
//         if !self.minimal && dialect.name() == DialectName::Pont {
//             write!(
//                 &mut f,
//                 r#"
// dependencies = [
//     {}
// ]
// "#,
//                 format!(
//                     r#"{{ git = "{}", tag = "{}"}}"#,
//                     PONT_STDLIB_URL, PONT_STDLIB_VERSION
//                 )
//             )?;
//         }
//         stdoutln!(
//             "Project {} initialized in {}",
//             good("successfully"),
//             ctx.project_dir.display()
//         );
//
//         Ok(())
//     }
}

// fn is_valid_name(text: &str) -> bool {
//     lazy_static! {
//         static ref RE: Regex = Regex::new(r"^[\w\-_]{1,64}$").unwrap();
//     }
//     RE.is_match(text)
// }
