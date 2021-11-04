use std::env::temp_dir;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{collections::BTreeMap};
use anyhow::{Error, Result};
use structopt::StructOpt;
use serde::{Serialize, Serializer};
use twox_hash::xxh3::hash128;

use move_core_types::account_address::AccountAddress;
use lang::compiler::dialects::DialectName;

use crate::context::Context;
use crate::cmd::Cmd;
use crate::manifest::{read_manifest, Dependence as DoveDependence, Git, DepPath};

type NamedAddress = String;
type PackageName = String;

type AddressDeclarations = BTreeMap<NamedAddress, Option<AccountAddress>>;
type DevAddressDeclarations = BTreeMap<NamedAddress, AccountAddress>;
type Version = (u64, u64, u64);
type Dependencies = BTreeMap<PackageName, Dependency>;

/// Export Dove.toml => Move.toml
#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
pub struct Export {
    #[structopt(long, hidden = true)]
    color: Option<String>,
}

impl Cmd for Export {
    fn apply(self, ctx: Context) -> Result<(), Error> {
        self.export(&ctx.project_dir)
    }
}

impl Export {
    fn export(&self, project_dir: &Path) -> Result<(), Error> {
        let dove_toml_path = project_dir.join("Dove.toml");
        if !dove_toml_path.exists() {
            anyhow::bail!("file Dove.toml was not found");
        }
        let dove_toml = read_manifest(&dove_toml_path)?;
        let dialect_name = DialectName::from_str(&dove_toml.package.dialect.unwrap_or_default())?;
        // Project directories
        create_project_directories(project_dir)?;

        // delete artifacts folder
        let artifacts_path = project_dir.join("artifacts");
        if artifacts_path.exists() {
            std::fs::remove_dir_all(artifacts_path)?;
        }

        // Move modules
        let modules_path = project_dir.join("modules");
        if modules_path.exists() {
            let source_path = project_dir.join("sources");
            modules_path
                .read_dir()?
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter_map(|path| {
                    path.file_name()
                        .and_then(|name| name.to_str())
                        .map(|name| name.to_string())
                        .map(|name| (path, name))
                })
                .for_each(
                    |(path, name)| {
                        if std::fs::rename(path, source_path.join(name)).is_err() {}
                    },
                );
            std::fs::remove_dir_all(modules_path)?;
        }

        // doc.toml
        save_as_toml(&project_dir.join("doc.toml"), &dove_toml.doc)?;
        // boogie_options.toml
        if let Some(boogie) = &dove_toml.boogie_options {
            save_as_toml(&project_dir.join("boogie_options.toml"), &boogie)?;
        }

        // account_address
        let mut addresses = AddressDeclarations::new();
        addresses.insert(
            "Account".to_string(),
            Some(
                dialect_name
                    .get_dialect()
                    .parse_address(&dove_toml.package.account_address)?,
            ),
        );

        // dependencies
        let dependencies: Dependencies = if let Some(deps) = dove_toml.package.dependencies {
            deps.deps
                .iter()
                .filter_map(|dep| match dependency_create_from(dep) {
                    Ok(dep) => dep.map(Ok),
                    Err(err) => Some(Err(err)),
                })
                .collect::<Result<Dependencies, Error>>()?
        } else {
            Dependencies::new()
        };

        let move_toml = MoveToml {
            package: PackageInfo {
                name: dove_toml.package.name.unwrap_or_else(|| {
                    project_dir
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string()
                }),
                authors: Vec::new(),
                license: None,
                version: (0, 0, 1),
                dialect: Some(dialect_name),
                dove_version: dove_toml.package.dove_version,
            },
            addresses: Some(addresses),
            dev_address_assignments: None,
            dependencies: if dependencies.is_empty() {
                None
            } else {
                Some(dependencies)
            },
            dev_dependencies: None,
            build: None,
        };

        std::fs::write(&project_dir.join("Move.toml"), toml::to_string(&move_toml)?)
            .map_err(|err| anyhow!(err.to_string()))
    }
}

#[derive(Debug, Serialize)]
struct MoveToml {
    pub package: PackageInfo,
    #[serde(serialize_with = "serialize_for_addresses")]
    pub addresses: Option<AddressDeclarations>,
    pub dev_address_assignments: Option<DevAddressDeclarations>,
    pub build: Option<BuildInfo>,
    pub dependencies: Option<Dependencies>,
    pub dev_dependencies: Option<Dependencies>,
}

fn serialize_for_addresses<S>(
    addresses: &Option<AddressDeclarations>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let data = addresses.as_ref().map(|list| {
        list.iter()
            .map(|(key, address)| {
                (
                    key.clone(),
                    address.map(|add| format!("0x{}", add.to_hex())),
                )
            })
            .collect::<BTreeMap<NamedAddress, Option<String>>>()
    });
    match data {
        Some(list) => serializer.serialize_some(&list),
        None => serializer.serialize_none(),
    }
}

#[derive(Debug, Serialize)]
struct PackageInfo {
    pub name: PackageName,
    #[serde(serialize_with = "serialize_for_version")]
    pub version: Version,
    pub authors: Vec<String>,
    pub license: Option<String>,
    pub dialect: Option<DialectName>,
    dove_version: Option<String>,
}
fn serialize_for_version<S>(version: &Version, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{}.{}.{}", version.0, version.1, version.2);
    serializer.serialize_str(&s)
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum Dependency {
    Local(DependencyLocal),
    Git(DependencyGit),
}

#[derive(Debug, Serialize)]
struct DependencyLocal {
    pub local: PathBuf,
}

#[derive(Debug, Serialize)]
struct DependencyGit {
    /// The git clone url to download from
    pub git: String,
    /// The git revision, AKA, a commit SHA
    ///     rev = "96d28e132dac33542bdafa4ac25324015d083bf1"
    ///     rev = "origin/main"
    pub rev: String,
    /// The path under this repo where the move package can be found -- e.g.,
    /// 'language/move-stdlib`
    pub subdir: PathBuf,
}

#[derive(Default, Debug, Serialize)]
struct BuildInfo {
    pub language_version: Option<Version>,
}

fn save_as_toml<S>(path: &Path, value: &S) -> Result<(), Error>
where
    S: Serialize,
{
    std::fs::write(path, toml::to_string(value)?).map_err(|err| anyhow!(err.to_string()))
}

fn dependency_create_from(dep: &DoveDependence) -> Result<Option<(String, Dependency)>, Error> {
    match dep {
        // Git dependency.
        DoveDependence::Git(git) => git_to_dependency(git).map(Some),
        // Local dependency.
        DoveDependence::Path(dep_path) => local_to_dependency(dep_path).map(Some),
        // Chain dependency.
        DoveDependence::Chain(_) => Ok(None),
    }
}

fn create_project_directories(project_dir: &Path) -> Result<(), Error> {
    for path in [
        project_dir.join("sources"),
        project_dir.join("examples"),
        project_dir.join("scripts"),
        project_dir.join("doc_templates"),
        project_dir.join("tests"),
    ]
    .iter()
    .filter(|path| !path.exists())
    {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

fn git_to_dependency(git: &Git) -> Result<(String, Dependency), Error> {
    let dep = Dependency::Git(DependencyGit {
        subdir: PathBuf::from(git.path.as_ref().unwrap_or(&"".to_string())),
        git: git.git.clone(),
        rev: git_rev(git)
            .cloned()
            .unwrap_or_else(|| "origin/master".to_string()),
    });

    Ok((github_get_name_package(git)?, dep))
}

fn local_to_dependency(local: &DepPath) -> Result<(String, Dependency), Error> {
    let local = PathBuf::from_str(&local.path)?;
    if !local.exists() {
        bail!("Project not found \nLocal: {}", &local.display());
    }

    let move_toml_path = local.join("Move.toml");
    if !move_toml_path.exists() {
        bail!("Move.toml not found \nLocal: {}", &local.display());
    }

    let dep = Dependency::Local(DependencyLocal {
        local: local.clone(),
    });

    let move_toml_content = std::fs::read_to_string(move_toml_path)
        .map_err(|err| anyhow!("Local: {}\n{}", &local.display(), err.to_string()))?;
    let move_toml = toml::from_str::<toml::Value>(&move_toml_content).map_err(|err| {
        anyhow!(
            "Error when parsing move.tool. \nLocal: {}\n{}",
            &local.display(),
            err
        )
    })?;

    let name = move_toml
        .get("package")
        .and_then(|pack| pack.get("name"))
        .ok_or_else(|| {
            anyhow!(
                "In Move.tool \"name\" not found \nLocal: {}",
                &local.display()
            )
        })
        .map(|name| name.as_str().unwrap_or("").to_string())?;
    Ok((name, dep))
}

fn git_rev(git: &Git) -> Option<&String> {
    git.rev
        .as_ref()
        .or_else(|| git.tag.as_ref())
        .or_else(|| git.branch.as_ref())
}

/// Get the package name
fn github_get_name_package(git: &Git) -> Result<String, Error> {
    let tmp_directory = temp_dir();
    let request_github_url = github_url_for_movetoml_file(git)?;
    let tmp_file_path =
        tmp_directory.join(hash128(request_github_url.as_bytes()).to_string() + ".toml");
    let move_toml_text = github_file_download(&tmp_file_path, &request_github_url)
        .map_err(|err| anyhow!("Git: {}\n{}", &git.git, err))?;

    let move_toml = toml::from_str::<toml::Value>(&move_toml_text).map_err(|err| {
        anyhow!(
            "Error when parsing move.tool. \nGit: {} \n{}",
            &git.git,
            err
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
        .ok_or_else(|| anyhow!("Dependency: Git {} expected github.com", &url))
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
