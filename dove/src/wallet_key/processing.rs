use anyhow::{Result, bail};

/// Checking and processing the key name
pub fn key_name(value: &str) -> Result<String> {
    let alias = value.trim().to_lowercase();
    let rg = regex::Regex::new(r"^[a-z\d\-\\_]+$")?;
    if rg.is_match(&alias) {
        Ok(alias)
    } else {
        bail!(r#"An alias can consist of letters, numbers and symbols "-", "-""#)
    }
}

pub fn secret_phrase(value: &str) -> Result<String> {
    let key_phrase: Vec<&str> = value
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

pub fn string_to_private_key(key: &str) -> Result<String> {
    let key = if key.starts_with('[') {
        key.trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|v| v.trim())
            .filter_map(|v| v.parse::<u8>().ok())
            .collect::<Vec<u8>>()
    } else {
        hex::decode(key.trim_start_matches("0x"))?
    };
    if key.len() != 32 {
        bail!("The key length was expected to be 64 bytes");
    }
    Ok(hex::encode(key))
}

pub mod url {
    use std::str::FromStr;

    use url::Url;
    use anyhow::Result;

    pub fn for_aptos(url: &str) -> Result<Url> {
        if url.is_empty() {
            bail!("URL cannot be empty");
        }
        let url = Url::from_str(url)?;

        match &url.origin() {
            url::Origin::Tuple(protocol, _, _) => match protocol.as_str() {
                "http" | "https" => Ok(url),
                _ => bail!(r#""http" | "https" protocol was expected"#),
            },
            _ => bail!("Unknown protocol"),
        }
    }

    pub fn for_substrate(url: &str) -> Result<Url> {
        if url.is_empty() {
            bail!("URL cannot be empty");
        }
        let url = Url::from_str(url)?;

        match &url.origin() {
            url::Origin::Tuple(protocol, _, _) => match protocol.as_str() {
                "http" | "https" | "ws" => Ok(url),
                _ => bail!(r#""http" | "https" | "ws" protocol was expected"#),
            },
            _ => bail!("Unknown protocol"),
        }
    }

    #[cfg(test)]
    mod test {
        use crate::wallet_key::processing::url::*;

        #[test]
        fn test_from_aptos() {
            for str_url in [
                "http://127.0.0.1/",
                "http://demo.example/",
                "http://localhost/",
                "https://demo.example/",
                "https://demo.example:8080/transaction?get=data",
            ] {
                for_aptos(str_url).unwrap();
            }

            for str_url in ["127.0.0.1", "ftp://127.0.0.1/", "ws://demo.example/", "127"] {
                assert!(for_aptos(str_url).is_err());
            }
        }

        #[test]
        fn test_from_substrate() {
            for str_url in [
                "http://127.0.0.1/",
                "http://demo.example/",
                "http://localhost/",
                "https://demo.example/",
                "https://demo.example:8080/transaction?get=data",
                "ws://demo.example/",
            ] {
                for_substrate(str_url).unwrap();
            }

            for str_url in ["127.0.0.1", "ftp://127.0.0.1/", "127"] {
                assert!(for_substrate(str_url).is_err());
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::key_name;

    #[test]
    fn test_alias() {
        for alias_str in ["Demo", "demo_123", "123 ", "demo-demo"] {
            assert!(key_name(alias_str).is_ok());
        }

        for alias_str in ["Demo&", "demo 123", "* ", "(demo)"] {
            assert!(key_name(alias_str).is_err());
        }
    }
}
