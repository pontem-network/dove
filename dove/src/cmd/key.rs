use std::str::FromStr;
use anyhow::Result;
use clap::Parser;
use url::Url;
use crate::wallet_key;

const DEFAULT_NODE_ADDRESS: &str = "ws://127.0.0.1:9944";

/// Manage wallet keys
#[derive(Debug, Parser)]
pub enum KeyCommand {
    /// Save the secret key for access under a alias
    #[clap(name = "add")]
    Add {
        /// Alias to access the key. Case-insensitive
        #[clap(long)]
        alias: String,

        /// Access to the key without a password. We do not recommend using this parameter.
        #[clap(long = "nopassword")]
        without_password: bool,
    },

    /// List of saved keys
    #[clap(name = "list")]
    List {},

    /// Delete a key
    #[clap(name = "delete")]
    Delete {
        /// Delete a key by alias. Case-insensitive
        #[clap(long)]
        alias: Option<String>,

        /// Delete all keys
        #[clap(long)]
        all: bool,
    },
}

impl KeyCommand {
    pub fn apply(&mut self) -> Result<()> {
        match &self {
            // Save the secret key for access under a alias
            KeyCommand::Add {
                alias,
                without_password,
            } => add(alias, *without_password),

            // Displaying a list of saved secret keys
            KeyCommand::List {} => list(),

            // Deleting secret keys
            KeyCommand::Delete { alias, all } => {
                if *all {
                    wallet_key::delete_all()?;
                    println!("All stored secret keys have been successfully deleted");
                } else if let Some(alias) = alias {
                    wallet_key::delete_by_alias(alias)?;
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
    let alias = wallet_key::valid_alias(alias)?;

    if wallet_key::existence(&alias) {
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
    let key = wallet_key::WalletKey::from((node_url, secret_phrase));

    wallet_key::save(&alias, password, key)?;

    Ok(())
}

/// Displaying a list of saved secret keys
fn list() -> Result<()> {
    println!("List of saved secret keys:");
    let list = wallet_key::list()?;
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

pub fn cli_entering_a_secret_phrase() -> Result<String> {
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
