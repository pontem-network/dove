use std::str::FromStr;

use anyhow::Result;
use clap::Parser;

use crate::wallet_key;
use crate::wallet_key::WalletKey;

const DEFAULT_NODE_ADDRESS: &str = "ws://127.0.0.1:9944";
const APTOS_TESTNET_URL: &str = "https://fullnode.devnet.aptoslabs.com";
const APTOS_FAUCET_URL: &str = "https://faucet.devnet.aptoslabs.com";

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

        /// Node type
        #[clap(long = "type", default_value = "Substrate")]
        tp: NodeType,

        /// Key for test node
        #[clap(long = "test-account")]
        test_account: bool,

        /// [@todo][Only Aptos][Devnet] Generate a new private key
        #[clap(long = "generate")]
        generate_new_key: bool,

        /// [@todo][Only Aptos][Devnet] Create a test account in node and set the balance to 10_000 coins
        #[clap(long = "create")]
        create: bool,
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

    /// [@todo] For Aptos [DEVNET]
    #[clap(name = "account")]
    Account {
        /// [@todo] Alias to access the key. Case-insensitive
        #[clap(long)]
        alias: String,

        /// [@todo] Create an account in Aptos using the specified key [DEVNET]
        #[clap(long)]
        create: bool,

        /// [@todo] Set the balance at account in Aptos [DEVNET]
        #[clap(long)]
        set_balance: usize,
    },
}

impl KeyCommand {
    pub fn apply(&mut self) -> Result<()> {
        match &self {
            // Save the secret key for access under a alias
            KeyCommand::Add {
                alias,
                without_password,
                tp,
                test_account,
                generate_new_key,
                create,
            } => add(
                alias,
                *without_password,
                *tp,
                *test_account,
                *generate_new_key,
                *create,
            ),

            // Displaying a list of saved secret keys
            KeyCommand::List {} => list(),

            // Deleting secret keys
            KeyCommand::Delete { alias, all } => {
                if *all {
                    WalletKey::delete_all()?;
                    println!("All stored secret keys have been successfully deleted");
                } else if let Some(alias) = alias {
                    WalletKey::delete_by_key_name(alias)?;
                } else {
                    bail!("Specify which secret key you want to delete");
                };
                Ok(())
            }
            KeyCommand::Account {
                alias: _,
                create: _,
                set_balance: _,
            } => {
                todo!()
            }
        }
    }
}

/// Save the secret key for access under a alias
fn add(
    alias: &str,
    without_password: bool,
    tp: NodeType,
    test: bool,
    generate_new_key: bool,
    create: bool,
) -> Result<()> {
    let key_name = wallet_key::processing::key_name(alias)?;

    if WalletKey::existence(&key_name) {
        bail!(r#"A key with name "{}" already exists"#, key_name);
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

    let key = match tp {
        NodeType::Substrate => {
            if generate_new_key || create {
                bail!(r#""--generate" and "--create" can only be used with the Aptos node type"#)
            }

            key_for_substrate(password, test)
        }
        NodeType::Aptos => {
            if !test && (create || generate_new_key) {
                bail!(
                    r#"Key generation and account creation are only possible for the test node. Use the "--test-account" parameter when creating the key"#
                );
            }

            key_for_aptos(password, test, generate_new_key, create)
        }
    }?;

    key.save(&key_name)
}

fn key_for_substrate(password: Option<&str>, test: bool) -> Result<WalletKey> {
    println!("Please enter secret phrase:");
    let secret_phrase = wallet_key::processing::secret_phrase(&cli_read_line(None)?)?;

    println!(
        "Please enter url of node [DEFAULT: {}]:",
        DEFAULT_NODE_ADDRESS
    );
    let node_url =
        wallet_key::processing::url::for_substrate(&cli_read_line(Some(DEFAULT_NODE_ADDRESS))?)?;

    WalletKey::subsrate_from(node_url, secret_phrase, password, test)
}

/// @todo generate_new_key
/// @todo create
fn key_for_aptos(
    password: Option<&str>,
    test: bool,
    generate_new_key: bool,
    create: bool,
) -> Result<WalletKey> {
    let key = if generate_new_key {
        // [APTOS][DEVNET] Key generation
        todo!()
    } else {
        println!("Please enter the private key in hexadecimal format:");
        wallet_key::processing::string_to_private_key(&cli_read_line(None)?)?
    };

    println!("Please enter url [DEFAULT: {}]:", APTOS_TESTNET_URL);
    let node = wallet_key::processing::url::for_aptos(&cli_read_line(Some(APTOS_TESTNET_URL))?)?;

    println!("Please enter faucet url [DEFAULT: {}]:", APTOS_FAUCET_URL);
    let faucet = wallet_key::processing::url::for_aptos(&cli_read_line(Some(APTOS_FAUCET_URL))?)?;

    let key = WalletKey::aptos_from(node, faucet, key, password, test)?;

    // [APTOS][DEVNET] Create an account in Aptos
    if create {
        // create
        // set balance
        todo!();
    }

    Ok(key)
}

/// Displaying a list of saved secret keys
fn list() -> Result<()> {
    println!("List of saved secret keys:");

    let list = WalletKey::list()?;
    if list.is_empty() {
        println!("- EMPTY -");
    } else {
        let errors = list
            .iter()
            .enumerate()
            .filter_map(|(num, name)| match WalletKey::load(name) {
                Ok(key) => {
                    let node_name = key.node_name().to_uppercase();

                    println!(
                        "{num:3}. [{node:1}]{test:3}{pass:3} {url} - {name}",
                        num = num + 1,
                        node = &node_name[..1],
                        test = key.is_test_account().then(|| "[T]").unwrap_or_default(),
                        pass = key.is_with_password().then(|| "[P]").unwrap_or_default(),
                        url = key.node_url(),
                        name = name
                    );
                    None
                }
                Err(_) => Some(format!(
                    "{:3}. {} The key is corrupted or outdated",
                    num + 1,
                    name
                )),
            })
            .collect::<Vec<String>>()
            .join("\n");

        if !errors.is_empty() {
            println!("\n[ERROR]");
            println!("{}", errors);
        }
    }

    Ok(())
}

#[inline]
fn read_password() -> Result<String> {
    let password = rpassword::read_password()?.trim().to_string();
    Ok(password)
}

pub fn cli_read_line(default: Option<&str>) -> Result<String> {
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buffer)?;

    let mut value = buffer.trim();
    if value.is_empty() {
        if let Some(def) = default {
            value = def;
        }
    }
    Ok(value.to_string())
}

#[derive(Parser, Debug, Copy, Clone)]
pub enum NodeType {
    Aptos,
    Substrate,
}

impl FromStr for NodeType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let value = s.to_lowercase();
        Ok(match value.as_str() {
            "aptos" => NodeType::Aptos,
            "substrate" => NodeType::Substrate,
            _ => bail!("Unknown node type - {}", value),
        })
    }
}
