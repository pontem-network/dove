use std::path::{PathBuf, Path};
use anyhow::Error;
use rand::Rng;
use std::env::temp_dir;
use twox_hash::xxh3::hash128;
use crate::export::DependenceExport;
use crate::export::dove_manifest::Git;
use crate::export::movetoml::{Dependency, DependencyGit};

/// Git dependency conversion: Dove.toml => Move.toml
pub fn convert_git_dependency(git: &Git) -> DependenceExport {
    let dep = Dependency::Git(DependencyGit {
        subdir: PathBuf::from(git.path.as_ref().unwrap_or(&"".to_string())),
        git: git.git.clone(),
        rev: git_rev(git)
            .cloned()
            .unwrap_or_else(|| "origin/master".to_string()),
    });
    let name = github_get_name_package(git);

    if let Ok(name) = name {
        DependenceExport {
            name,
            dep,
            error: None,
        }
    } else {
        DependenceExport {
            name: format!("NoName_{}", rand::thread_rng().gen_range(1, 9999999)),
            dep,
            error: name.err(),
        }
    }
}

fn git_rev(git: &Git) -> Option<&String> {
    git.rev
        .as_ref().or(git.tag.as_ref()).or(git.branch.as_ref())
}

/// Get the package name
fn github_get_name_package(git: &Git) -> Result<String, Error> {
    let tmp_directory = temp_dir();
    let request_github_url = github_url_for_movetoml_file(git)?;
    let tmp_file_path =
        tmp_directory.join(hash128(request_github_url.as_bytes()).to_string() + ".toml");
    let move_toml_text = github_file_download(&tmp_file_path, &request_github_url)
        .map_err(|err| anyhow!("{}\nGit: {}", err, &git.git))?;

    let move_toml = toml::from_str::<toml::Value>(&move_toml_text).map_err(|err| {
        anyhow!(
            "Error when parsing move.tool. \n{} \nGit: {}",
            err,
            &git.git
        )
    })?;
    move_toml
        .get("package")
        .and_then(|pack| pack.get("name"))
        .ok_or_else(|| anyhow!(r#"In Move.tool "name" not found"#))
        .map(|name| name.as_str().unwrap_or("").to_string())
}

fn github_url_for_movetoml_file(git: &Git) -> Result<String, Error> {
    let url = git.git.clone();
    let rev = git_rev(git);
    let mut request_url = url
        .find("github.com")
        .ok_or_else(|| anyhow!("Expected github.com\nGit {} ", &url))
        .map(|start| {
            url.rfind(".git")
                .map(|end| &url[start + 11..end])
                .unwrap_or_else(|| &url[start..])
        })?
        .to_string();

    request_url = format!(
        "https://api.github.com/repos/{}/contents{}Move.toml{}",
        request_url,
        git.path
            .as_ref()
            .map(|sub| format!("/{}/", sub.trim_matches('/')))
            .unwrap_or_else(|| "/".to_string()),
        rev.map(|rev| format!("?ref={}", rev)).unwrap_or_default()
    );
    Ok(request_url)
}

fn github_file_download(path: &Path, url: &str) -> Result<String, Error> {
    fn request(url: &str) -> Result<String, Error> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Accept",
            reqwest::header::HeaderValue::from_static("application/vnd.github.v3.raw"),
        );
        headers.insert(
            "User-Agent",
            reqwest::header::HeaderValue::from_static("curl/7.68.0"),
        );

        let response = reqwest::blocking::Client::new()
            .get(url)
            .headers(headers)
            .send()
            .map_err(|err| anyhow!("Couldn't get Move.toml\nRequest: {}\n{}", url, err))?;
        if response.status() != 200 {
            bail!(
                "Couldn't get Move.toml\nRequest: {}\nStatus: {}",
                url,
                response.status()
            );
        }
        response.text().map_err(|err| {
            anyhow!(
                "Couldn't get Move.toml.\nRequest: {}\n{}",
                url,
                err.to_string()
            )
        })
    }

    if path.exists() {
        let move_toml_content = std::fs::read_to_string(path)?;
        if move_toml_content.is_empty() {
            bail!("Move.toml not found\nRequest: {}", url);
        }
        return Ok(move_toml_content);
    }

    match request(url) {
        Ok(content) => {
            std::fs::write(path, &content)?;
            Ok(content)
        }
        Err(err) => {
            std::fs::write(path, "")?;
            anyhow::bail!(err)
        }
    }
}
