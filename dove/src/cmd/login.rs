use std::path::PathBuf;
use anyhow::Result;
use structopt::StructOpt;
use move_cli::Move;
use crate::cmd::{Cmd, default_sourcemanifest};
use crate::cmd::publish::cli_entering_a_secret_phrase;
use crate::context::Context;
use crate::secret_phrase;

/// Secret Key Management
#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
pub enum Login {
    /// Save the secret key for access under a alias
    #[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
    #[structopt(name = "add")]
    Add {
        /// Alias to access the key. Case-insensitive
        #[structopt(long)]
        alias: String,

        /// Access to the key without a password. We do not recommend using this parameter.
        #[structopt(long = "nopassword")]
        without_password: bool,
    },

    /// List of saved keys
    #[structopt(name = "list")]
    #[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
    List {},

    /// Delete a key
    #[structopt(name = "delete")]
    #[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
    Delete {
        /// Delete a key by alias. Case-insensitive
        #[structopt(long)]
        alias: Option<String>,

        /// Delete all keys
        #[structopt(long)]
        all: bool,
    },
}

impl Cmd for Login {
    fn context(&mut self, project_dir: PathBuf, move_args: Move) -> Result<Context> {
        Ok(Context {
            project_dir,
            move_args,
            manifest: default_sourcemanifest(),
            manifest_hash: 0,
        })
    }

    fn apply(&mut self, _ctx: &mut Context) -> Result<()>
    where
        Self: Sized,
    {
        match &self {
            // Save the secret key for access under a alias
            Login::Add {
                alias,
                without_password,
            } => add(alias, *without_password),

            // Displaying a list of saved secret keys
            Login::List {} => list(),

            // Deleting secret keys
            Login::Delete { alias, all } => {
                if *all {
                    secret_phrase::delete_all()?;
                    println!("All stored secret keys have been successfully deleted");
                } else if let Some(alias) = alias {
                    secret_phrase::delete_by_alias(alias)?;
                } else {
                    bail!("Specify which secret key you want to delete");
                };
                Ok(())
            }
        }
    }
}

/// Save the secret key for access under a alias
fn add(alias: &str, without_password: bool) -> Result<()> {
    let alias = secret_phrase::valid_alias(alias)?;

    if secret_phrase::isset(&alias) {
        bail!(r#"A key with name "{}" already exists"#, alias);
    }

    let enter_password: String;
    let password = if without_password {
        None
    } else {
        println!("Please enter password for key:");
        enter_password = read_password()?;

        println!("Confirm password:");
        let confirm = read_password()?;

        if enter_password != confirm {
            bail!("Passwords don't match");
        }

        Some(enter_password.as_str())
    };
    let phrase = cli_entering_a_secret_phrase()?;

    secret_phrase::save(&phrase, &alias, password)?;

    Ok(())
}

/// Displaying a list of saved secret keys
fn list() -> Result<()> {
    println!("List of saved secret keys:");
    let list = secret_phrase::list()?;
    if list.is_empty() {
        println!("- EMPTY -");
    } else {
        list.iter()
            .enumerate()
            .for_each(|(num, name)| println!(" {}. {}", num + 1, name));
    }

    Ok(())
}

fn read_password() -> Result<String> {
    let password = rpassword::read_password()?.trim().to_string();
    Ok(password)
}

#[cfg(test)]
mod test {}
