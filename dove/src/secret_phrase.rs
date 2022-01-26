use std::fs;
use std::num::NonZeroU32;
use std::path::PathBuf;

use anyhow::{anyhow, Result};
use aes::Aes256;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use crate::move_folder;

/// The name of the file with salt for generating the key by password
const SALT_FILE_NAME: &str = "salt.aes";
/// The name of the file with the encryption key
const IV_FILE_NAME: &str = "iv.p7s";
const PADDING_SIZE: usize = 20;
type Aes256Cbc = Cbc<Aes256, Pkcs7>;

/// Saving a secret phrase
/// Secret phrase will be stored encrypted in the directory "~/.move/" with the alias name and the extension "*.key".
/// ~/.move/<ALIAS>.key
pub fn save(secret_phrase: &str, alias: &str, password: Option<&str>) -> Result<()> {
    let path = path(alias)?;
    if path.exists() {
        bail!(r#"A key with name "{}" already exists"#, alias);
    }

    let data = encrypt(secret_phrase.trim().as_bytes(), password)?;
    fs::write(&path, data)?;
    Ok(())
}

/// Get saved secret phrase
/// Decrypted from ~/.move/<ALIAS>.key
pub fn get(alias: &str, password: Option<&str>) -> Result<String> {
    let path = path(alias)?;
    if !path.exists() {
        bail!(r#"A key with name "{}" not exists"#, alias);
    }

    let file_contents = fs::read(&path)?;
    let dec = decrypt(file_contents.as_slice(), password)?;
    let secret_phrase = String::from_utf8(dec)?;

    Ok(secret_phrase)
}

/// Check if there is a secret phrase with this alias
/// ~/.move/<ALIAS>.key
#[inline]
pub fn isset(alias: &str) -> bool {
    path(alias).map_or(false, |path| path.exists())
}

/// List of saved secret phrase.
/// Returns names of files with the extension "*.key" from directory "~/.move/"
/// ~/.move/*.key
pub fn list() -> Result<Vec<String>> {
    let list = move_folder()?
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
/// ~/.move/<ALIAS>.key
fn path(alias: &str) -> Result<PathBuf> {
    let alias = valid_alias(alias)?;
    move_folder().map(|path| path.join(&alias).with_extension("key"))
}

/// Checking and processing the key name
pub fn valid_alias(alias: &str) -> Result<String> {
    let alias = alias.trim().to_lowercase();
    let rg = regex::Regex::new(r"^[a-z\d\-\\_]+$")?;
    if rg.is_match(&alias) {
        Ok(alias)
    } else {
        bail!(r#"An alias can consist of letters, numbers and symbols "-", "-""#)
    }
}

/// Delete a secret phrase by alias
pub fn delete_by_alias(alias: &str) -> Result<()> {
    let path = path(alias)?;
    if !path.exists() {
        bail!(r#"A key with name "{}" not exists"#, alias);
    }
    fs::remove_file(&path)?;

    Ok(())
}

/// Delete all saved secret phrases
pub fn delete_all() -> Result<()> {
    list()?
        .iter()
        .try_for_each(|alias| delete_by_alias(alias))?;
    Ok(())
}

/// Encrypt data
fn encrypt(data: &[u8], password: Option<&str>) -> Result<Vec<u8>> {
    let key = aes_key(password)?;
    let iv = pkcs7_key()?;

    let cipher = Aes256Cbc::new_from_slices(&key, &iv)?;
    let mut buffer = vec![0; data.len() + PADDING_SIZE];
    let pos = data.len();
    buffer[..pos].copy_from_slice(data);

    let result = cipher
        .encrypt(&mut buffer, pos)
        .map(|result| result.to_vec())?;
    Ok(result)
}

/// Decrypt data
fn decrypt(data: &[u8], password: Option<&str>) -> Result<Vec<u8>> {
    let key = aes_key(password)?;
    let iv = pkcs7_key()?;

    let cipher = Aes256Cbc::new_from_slices(&key, &iv)?;
    let mut buffer = data.to_vec();

    let result = cipher.decrypt(&mut buffer).map(|result| result.to_vec())?;
    Ok(result)
}

/// Get the aes key value.
/// It is created based on the password + "salt.aes"
/// If "~/.move/salt.aes" does not exist, then it will be created
fn aes_key(password: Option<&str>) -> Result<[u8; 32]> {
    let salt_path = move_folder()?.join(SALT_FILE_NAME);
    let salt = if !salt_path.exists() {
        let salt = gen_key()?;
        fs::write(salt_path, salt)?;
        salt
    } else {
        let key = fs::read(salt_path)?;
        let mut buf = [0; 32];
        buf[..32].copy_from_slice(&key);
        buf
    };

    let key = match password {
        Some(word) => {
            let mut pbkdf2_hash = [0u8; digest::SHA256_OUTPUT_LEN];
            pbkdf2::derive(
                pbkdf2::PBKDF2_HMAC_SHA256,
                NonZeroU32::new(14_623).unwrap(),
                &salt,
                word.as_bytes(),
                &mut pbkdf2_hash,
            );
            pbkdf2_hash
        }
        None => salt,
    };
    Ok(key)
}

/// Get the pkcs7 key value
/// If the key is not in "~/.move/iv.p7s", it will be created
fn pkcs7_key() -> Result<[u8; 16]> {
    let key_path = move_folder()?.join(IV_FILE_NAME);
    let key = if !key_path.exists() {
        let salt = gen_key()?;
        fs::write(key_path, salt)?;
        salt
    } else {
        let buf = fs::read(key_path)?;
        let mut key = [0; 16];
        key[..16].copy_from_slice(&buf);
        key
    };

    Ok(key)
}

/// Generate a key
#[inline]
fn gen_key<const N: usize>() -> Result<[u8; N]> {
    let mut random = [0; N];
    let rng = rand::SystemRandom::new();
    rng.fill(&mut random)
        .map_err(|_| anyhow!("Failed to generate a key"))?;
    Ok(random.to_owned())
}

#[cfg(test)]
mod test {
    use super::{aes_key, decrypt, encrypt, pkcs7_key, valid_alias};

    const TEXT: &str = "Lorem Ipsum - All the facts - Lipsum generator";
    const PASSWORD: &str = "demo123DD";

    #[test]
    fn test_valid_alias() {
        for alias in ["Demo", "demo_123", "123 ", "demo-demo"] {
            assert!(valid_alias(alias).is_ok());
        }

        for alias in ["Demo&", "demo 123", "* ", "(demo)"] {
            assert!(valid_alias(alias).is_err());
        }
    }

    #[test]
    fn test_key() {
        aes_key(None).unwrap();
        aes_key(Some(PASSWORD)).unwrap();
        pkcs7_key().unwrap();
    }

    #[test]
    fn test_encrypt_without_password() {
        let enc = encrypt(TEXT.as_bytes(), None).unwrap();
        let dec = decrypt(enc.as_slice(), None).unwrap();

        assert_eq!(TEXT.as_bytes(), dec.as_slice());
    }

    #[test]
    fn encrypt_with_password() {
        let enc = encrypt(TEXT.as_bytes(), Some(PASSWORD)).unwrap();
        let dec = decrypt(enc.as_slice(), Some(PASSWORD)).unwrap();

        assert_eq!(TEXT.as_bytes(), dec.as_slice());
    }
}
