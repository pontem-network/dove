use anyhow::Error;
use std::fs;
use crate::cmd::Cmd;
use crate::context::Context;
use structopt::StructOpt;
use std::str::FromStr;

/// Clean target directory command.
#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
pub struct Clean {
    #[structopt(about = "Type of cleaning.")]
    clear_type: Option<ClearType>,
    #[structopt(long, hidden = true)]
    color: Option<String>,
}

impl Cmd for Clean {
    fn apply(self, ctx: Context) -> Result<(), Error> {
        let clear_type = self.clear_type.unwrap_or_default();

        match clear_type {
            ClearType::State => {
                let path = ctx.path_for(&ctx.manifest.layout.storage_dir);
                if path.exists() {
                    fs::remove_dir_all(path)?;
                }
                Ok(())
            }
            ClearType::All => {
                let artifacts = ctx.path_for(&ctx.manifest.layout.artifacts);
                let index_path = ctx.path_for(&ctx.manifest.layout.index);

                if index_path.exists() {
                    fs::remove_file(index_path)?;
                }

                if artifacts.exists() {
                    fs::remove_dir_all(artifacts).map_err(Into::into)
                } else {
                    Ok(())
                }
            }
        }
    }
}

/// The type of cleaning.
#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
pub enum ClearType {
    /// Clear only the executor state.
    #[structopt(about = "Clear only the executor state.")]
    State,
    /// Clear all.
    #[structopt(about = "Clear all.")]
    All,
}

impl Default for ClearType {
    fn default() -> Self {
        ClearType::All
    }
}

impl FromStr for ClearType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "state" => ClearType::State,
            _ => ClearType::All,
        })
    }
}
