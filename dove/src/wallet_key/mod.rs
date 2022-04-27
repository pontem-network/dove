use std::fs;
use std::path::PathBuf;

use serde::{Serialize, Deserialize};
use anyhow::Result;

use url::Url;

pub mod encryption;
pub mod processing;

use crate::dot_move_folder;

#[derive(Serialize, Deserialize, Debug)]
pub enum WalletKey {
    Substrate {
        node: Url,
        key: Vec<u8>,
        with_password: bool,
        test: bool,
    },
    Aptos {
        node: Url,
        faucet: Url,
        key: Vec<u8>,
        with_password: bool,
        test: bool,
    },
}

impl WalletKey {
    pub fn node_url(&self) -> &Url {
        match self {
            WalletKey::Substrate { node, .. } => node,
            WalletKey::Aptos { node, .. } => node,
        }
    }

    /// hex or secret phrase
    pub fn private_key(&self, password: Option<&str>) -> Result<String> {
        if self.is_with_password() && password.is_none() {
            bail!("Password required");
        }
        let enc_key = match self {
            WalletKey::Substrate { key, .. } => key,
            WalletKey::Aptos { key, .. } => key,
        };
        let key = String::from_utf8(encryption::decrypt(enc_key, password)?)?;
        Ok(key)
    }

    pub fn is_test_account(&self) -> bool {
        *match self {
            WalletKey::Substrate { test, .. } => test,
            WalletKey::Aptos { test, .. } => test,
        }
    }

    pub fn is_with_password(&self) -> bool {
        *match self {
            WalletKey::Substrate {
                with_password: enc, ..
            } => enc,
            WalletKey::Aptos {
                with_password: enc, ..
            } => enc,
        }
    }

    pub fn is_aptos(&self) -> bool {
        matches!(self, WalletKey::Aptos { .. })
    }

    /// Saving a "private key" + DATA KEY
    /// "private key" + URL will be stored encrypted in the directory "~/.move/" with the key name and the extension "*.key".
    /// ~/.move/<KEY_NAME>.key
    pub fn save(&self, key_name: &str) -> Result<()> {
        let path = WalletKey::path(key_name)?;
        if path.exists() {
            bail!(r#"A key with name "{}" already exists"#, key_name);
        }

        let data = bcs::to_bytes(&self)?;
        fs::write(&path, data)?;
        Ok(())
    }

    /// Aptos|Substrate
    pub fn node_name(&self) -> String {
        match self {
            WalletKey::Substrate { .. } => "Substrate",
            WalletKey::Aptos { .. } => "Aptos",
        }
        .to_string()
    }
}

impl WalletKey {
    pub fn subsrate_from(
        node: Url,
        private_key: String,
        password: Option<&str>,
        test: bool,
    ) -> Result<WalletKey> {
        let key = encryption::encrypt(private_key.as_bytes(), password)?;

        Ok(WalletKey::Substrate {
            node,
            key,
            with_password: password.is_some(),
            test,
        })
    }

    pub fn aptos_from(
        node: Url,
        faucet: Url,
        key: String,
        password: Option<&str>,
        test: bool,
    ) -> Result<WalletKey> {
        let key = encryption::encrypt(key.as_bytes(), password)?;

        Ok(WalletKey::Aptos {
            node,
            faucet,
            key,
            with_password: password.is_some(),
            test,
        })
    }

    /// Get saved "secret phrase" + URL
    /// Decrypted from ~/.move/<ALIAS>.key
    pub fn load(alias: &str) -> Result<WalletKey> {
        let path = WalletKey::path(alias)?;
        if !path.exists() {
            bail!(r#"A key with name "{}" not exists"#, alias);
        }

        let file_contents = fs::read(&path)?;
        Ok(bcs::from_bytes(&file_contents)?)
    }

    /// Check if there is a secret phrase with this alias
    /// ~/.move/<ALIAS>.key
    #[inline]
    pub fn existence(alias: &str) -> bool {
        WalletKey::path(alias).map_or(false, |path| path.exists())
    }

    /// List of saved secret phrase.
    /// Returns names of files with the extension "*.key" from directory "~/.move/"
    /// ~/.move/*.key
    pub fn list() -> Result<Vec<String>> {
        let list = dot_move_folder()?
            .read_dir()?
            .filter_map(|dir| dir.ok())
            .map(|path| path.path())
            .filter(|path| {
                path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("key")
            })
            .filter_map(|path| {
                path.file_stem()
                    .map(|name| name.to_string_lossy().to_string())
            })
            .collect();
        Ok(list)
    }

    /// Path to the secret phrase
    /// ~/.move/<KEY_NAME>.key
    fn path(key_name: &str) -> Result<PathBuf> {
        let key_name = processing::key_name(key_name)?;
        dot_move_folder().map(|path| path.join(&key_name).with_extension("key"))
    }

    /// Delete a secret phrase by key name
    pub fn delete_by_key_name(key_name: &str) -> Result<()> {
        let path = WalletKey::path(key_name)?;
        if !path.exists() {
            bail!(r#"A key with name "{}" not exists"#, key_name);
        }
        fs::remove_file(&path)?;

        Ok(())
    }

    /// Delete all saved secret phrases
    pub fn delete_all() -> Result<()> {
        WalletKey::list()?
            .iter()
            .try_for_each(|key_name| WalletKey::delete_by_key_name(key_name))?;
        Ok(())
    }
}
