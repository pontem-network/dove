use std::str::FromStr;
use anyhow::Result;
use structopt::StructOpt;
use url::Url;
use crate::access;

const DEFAULT_NODE_ADDRESS: &str = "ws://127.0.0.1:9944";

/// Secret Key Management
#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
pub enum Key {
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

impl Key {
    pub fn apply(&mut self) -> Result<()>
    where
        Self: Sized,
    {
        match &self {
            // Save the secret key for access under a alias
            Key::Add {
                alias,
                without_password,
            } => add(alias, *without_password),

            // Displaying a list of saved secret keys
            Key::List {} => list(),

            // Deleting secret keys
            Key::Delete { alias, all } => {
                if *all {
                    access::delete_all()?;
                    println!("All stored secret keys have been successfully deleted");
                } else if let Some(alias) = alias {
                    access::delete_by_alias(alias)?;
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
    let alias = access::valid_alias(alias)?;

    if access::isset(&alias) {
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

    let secret_phrase = cli_entering_a_secret_phrase()?;
    let node_url = cli_read_node_address()?;
    let key = access::Key::from((node_url, secret_phrase));

    access::save(&alias, password, key)?;

    Ok(())
}

/// Displaying a list of saved secret keys
fn list() -> Result<()> {
    println!("List of saved secret keys:");
    let list = access::list()?;
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

fn cli_entering_a_secret_phrase() -> Result<String> {
    println!("Please enter secret phrase:");

    let key_phrase = cli_read_line()?;
    let key_phrase: Vec<&str> = key_phrase
        .trim()
        .split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    if key_phrase.is_empty() {
        bail!("Secret phrase cannot be empty");
    }

    if ![12, 18, 24].into_iter().any(|num| num == key_phrase.len()) {
        bail!("Wrong number of words");
    }

    Ok(key_phrase.join(" "))
}

fn cli_read_node_address() -> Result<Url> {
    println!(
        "Please enter url of node [DEFAULT: {}]:",
        DEFAULT_NODE_ADDRESS
    );

    let mut url = cli_read_line()?;
    if url.is_empty() {
        url = DEFAULT_NODE_ADDRESS.to_string();
    }
    let url = Url::from_str(&url)?;

    match &url.origin() {
        url::Origin::Tuple(protocol, _, _) => match protocol.as_str() {
            "http" | "https" | "ws" => (),
            _ => bail!(r#""http" | "https" | "ws" protocol was expected"#),
        },
        _ => bail!("Unknown protocol"),
    };

    Ok(url)
}

fn cli_read_line() -> Result<String> {
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}
