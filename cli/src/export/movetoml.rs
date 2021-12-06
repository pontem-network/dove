use serde::{Serialize, Serializer};
use std::collections::BTreeMap;
use std::path::PathBuf;
use move_core_types::account_address::AccountAddress;
use dialect::Dialect;

type NamedAddress = String;
type PackageName = String;

/// Declared address
pub type AddressDeclarations = BTreeMap<NamedAddress, Option<AccountAddress>>;
/// Project version
type Version = (u64, u64, u64);
/// Project dependencies.
pub type Dependencies = BTreeMap<PackageName, Dependency>;

/// Manifest move.toml
#[derive(Debug, Serialize)]
pub struct MoveToml {
    /// Package Information.
    pub package: PackageInfo,
    /// Project addresses: Packages, accounts
    #[serde(serialize_with = "serialize_for_addresses")]
    pub addresses: Option<AddressDeclarations>,
    /// Project dependencies
    pub dependencies: Option<Dependencies>,
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

/// Package Information
#[derive(Debug, Serialize)]
pub struct PackageInfo {
    /// Project Name
    pub name: PackageName,
    /// Project version. Example: (0,0,1) => 0.0.1
    #[serde(serialize_with = "serialize_for_version")]
    pub version: Version,
    /// List of authors of the project
    pub authors: Vec<String>,
    /// project license
    pub license: Option<String>,
    /// Dialect of the project. Pont, Diem, DFinance
    #[serde(serialize_with = "serialize_dialect")]
    pub dialect: Option<Dialect>,
    /// The minimum version of "Dove"
    pub dove_version: Option<String>,
}

fn serialize_for_version<S>(version: &Version, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{}.{}.{}", version.0, version.1, version.2);
    serializer.serialize_str(&s)
}

fn serialize_dialect<S>(dialect: &Option<Dialect>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let data = dialect.map(|d| match d {
        Dialect::Diem => "Diem",
        Dialect::Pont => "Pont",
        Dialect::DFinance => "DFinance",
    });
    match data {
        Some(list) => serializer.serialize_some(&list),
        None => serializer.serialize_none(),
    }
}

/// Dependency type: local and git
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Dependency {
    /// Local path to the project
    Local(DependencyLocal),
    /// Project address on GitHub
    Git(DependencyGit),
}

/// Local dependency. The path to the project
#[derive(Debug, Serialize)]
pub struct DependencyLocal {
    /// Local path to the project
    pub local: String,
}

/// Git dependency. Address to the project's github, commit/branch/tag
#[derive(Debug, Serialize)]
pub struct DependencyGit {
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
