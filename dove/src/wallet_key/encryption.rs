use std::fs;
use std::num::NonZeroU32;
use std::path::Path;

use anyhow::{anyhow, Result};
use aes::Aes256;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
use lockfile::Lockfile;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};

use crate::dot_move_folder;

/// The name of the file with salt for generating the key by password
const SALT_FILE_NAME: &str = "salt.aes";
/// The name of the file with the encryption key
const IV_FILE_NAME: &str = "iv.p7s";
const PADDING_SIZE: usize = 20;
type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub fn encrypt(data: &[u8], password: Option<&str>) -> Result<Vec<u8>> {
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

pub fn decrypt(data: &[u8], password: Option<&str>) -> Result<Vec<u8>> {
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
    let salt_path = dot_move_folder()?.join(SALT_FILE_NAME);

    let salt = if let Some(lock) = exist_or_lock(&salt_path)? {
        let salt = generate_key()?;
        fs::write(salt_path, salt)?;
        lock.release()?;
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
    let key_path = dot_move_folder()?.join(IV_FILE_NAME);
    let key = if let Some(lock) = exist_or_lock(&key_path)? {
        let salt = generate_key()?;
        fs::write(key_path, salt)?;
        lock.release()?;
        salt
    } else {
        let buf = fs::read(key_path)?;
        let mut key = [0; 16];
        key[..16].copy_from_slice(&buf);
        key
    };

    Ok(key)
}

#[inline]
fn generate_key<const N: usize>() -> Result<[u8; N]> {
    let mut random = [0; N];
    let rng = rand::SystemRandom::new();
    rng.fill(&mut random)
        .map_err(|_| anyhow!("Failed to generate a key"))?;
    Ok(random.to_owned())
}

/// Checking for the presence of a file, if there is no file, we get a lock. Return Ok(Some(Lockfile))
/// If the lock is on, then wait for the removal. Return: Ok(None)
/// If the file exists, then no action is required.. Return: Ok(None)
fn exist_or_lock(path_key: &Path) -> Result<Option<Lockfile>> {
    let lock_path = path_key.with_extension("lock");

    if path_key.exists() {
        if lock_path.exists() {
            let mut time_limit = 300;
            let sleep_time = std::time::Duration::from_secs(1);
            loop {
                std::thread::sleep(sleep_time);
                if !lock_path.exists() {
                    break;
                }
                time_limit -= 1;
                if time_limit == 0 {
                    bail!("Waiting time has expired");
                }
            }
        }
        return Ok(None);
    }

    match Lockfile::create(&lock_path) {
        Ok(lock) => Ok(Some(lock)),
        Err(err) => match err {
            lockfile::Error::LockTaken => exist_or_lock(path_key),
            _ => bail!("File Lock Error: {} {:?}", lock_path.display(), err),
        },
    }
}

#[cfg(test)]
mod test {
    use super::{aes_key, decrypt, encrypt, pkcs7_key};

    const TEXT: &str = "Lorem Ipsum - All the facts - Lipsum generator";
    const PASSWORD: &str = "demo123DD";

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
