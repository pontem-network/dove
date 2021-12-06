use std::str::FromStr;
use std::fs;
use std::path::PathBuf;
use anyhow::Error;
use structopt::StructOpt;
use move_cli::Move;
use crate::cmd::{Cmd, default_sourcemanifest};
use crate::context::Context;

/// Clean target directory command.
#[derive(StructOpt, Debug, Default)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
pub struct Clean {
    // Directories will be deleted
    // [state] Clear only the executor state:
    //      PROJECT_DIR/storage
    //      PROJECT_DIR/build/mv_interfaces
    //      PROJECT_DIR/build/package
    // [all] Clear all:
    //      PROJECT_DIR/storage
    //      PROJECT_DIR/build
    #[structopt(help = "Type of cleaning. [default=all]\n\
                        state - Clear only the executor state.\n\
                        all - Clear all.")]
    clear_type: Option<ClearType>,
    // deleting folders:
    //      PROJECT_DIR/storage
    //      PROJECT_DIR/build
    //      ~/.move/*
    #[structopt(help = "Clear target directory and global cache command", long)]
    global: bool,
}

impl Cmd for Clean {
    fn context(&mut self, project_dir: PathBuf, move_args: Move) -> anyhow::Result<Context> {
        Ok(Context {
            project_dir,
            move_args,
            manifest: default_sourcemanifest(),
            manifest_hash: 0,
        })
    }

    fn apply(&mut self, ctx: &mut Context) -> anyhow::Result<()>
    where
        Self: Sized,
    {
        let clear_type = self.clear_type.unwrap_or_default();

        let mut folders = match clear_type {
            // Clear only the executor state.
            ClearType::State => {
                vec![
                    ctx.project_dir.join("storage"),
                    ctx.project_dir.join("build").join("mv_interfaces"),
                    ctx.project_dir.join("build").join("package"),
                ]
            }
            // Clear all.
            ClearType::All => {
                vec![
                    ctx.project_dir.join("storage"),
                    ctx.project_dir.join("build"),
                ]
            }
        };

        // If global cleanup adds directories from ~/.move/*
        if self.global {
            folders.extend(move_cache_folders().unwrap_or_default().into_iter());
        }

        for path in folders {
            if !path.exists() {
                continue;
            }
            if let Err(err) = fs::remove_dir_all(&path) {
                println!(
                    "Warning: failed to delete directory {}\n{}",
                    path.display(),
                    err.to_string()
                );
            }
        }
        Ok(())
    }
}

/// The type of cleaning.
#[derive(StructOpt, Debug, Copy, Clone)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
pub enum ClearType {
    /// Clear only the executor state.
    #[structopt(help = "Clear only the executor state.")]
    State,
    /// Clear all.
    #[structopt(help = "Clear all.")]
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

/// Clean project.
pub fn run_internal_clean(ctx: &mut Context) -> anyhow::Result<()> {
    let mut cmd = Clean::default();
    cmd.apply(ctx)
}

/// adds directories from ~/.move/*
fn move_cache_folders() -> anyhow::Result<Vec<PathBuf>> {
    let move_home = std::env::var("MOVE_HOME").unwrap_or_else(|_| {
        format!(
            "{}/.move",
            std::env::var("HOME").expect("env var 'HOME' must be set")
        )
    });

    let path = PathBuf::from_str(&move_home)?;
    if !path.exists() {
        bail!("MOVE_HOME - path {:?} not found", path.display());
    }

    let paths = path
        .read_dir()?
        .filter_map(|dir| dir.ok())
        .map(|path| path.path())
        .collect::<Vec<PathBuf>>();
    Ok(paths)
}