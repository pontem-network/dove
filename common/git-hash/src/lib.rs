/*!
    Utility proc-macro crate for get current git HEAD hash as constant `Option<&'static str>`.

    ## Usage example:

    ```rust
    // main.rs
    extern crate git_hash;

    fn main() {
        println!("{:?}", git_hash::git_hash!());
        println!("{:?}", git_hash::git_hash_short!());
    }
    ```
*/

extern crate proc_macro;
use proc_macro::{TokenStream, LexError};
mod cmd;

#[proc_macro]
pub fn git_hash(_: TokenStream) -> TokenStream {
    let git_hash = cmd::git_hash();
    opt_tokenize(git_hash).unwrap()
}

#[proc_macro]
pub fn git_hash_short(_: TokenStream) -> TokenStream {
    let git_hash = cmd::git_hash_short();
    opt_tokenize(git_hash).unwrap()
}
#[proc_macro]
pub fn git_hash_short_as_str(_: TokenStream) -> TokenStream {
    format!(r#""{}""#, cmd::git_hash_short().unwrap_or_default())
        .parse()
        .unwrap()
}
/// Package version
#[proc_macro]
pub fn crate_version(_: TokenStream) -> TokenStream {
    "env!(\"CARGO_PKG_VERSION\")".parse().unwrap_or_default()
}

#[proc_macro]
pub fn crate_version_with_git_hash(_: TokenStream) -> TokenStream {
    crate_version_with(cmd::git_hash()).unwrap()
}

#[proc_macro]
pub fn crate_version_with_git_hash_short(_: TokenStream) -> TokenStream {
    crate_version_with(cmd::git_hash_short()).unwrap()
}

/// Package version|branch
#[proc_macro]
pub fn dependency_branch_from_cargo_lock(name: TokenStream) -> TokenStream {
    let package = name.to_string();

    format!(
        r#""{}""#,
        DependencyInfo::from_cargo_lock(&package)
            .branch
            .unwrap_or_default()
    )
    .parse()
    .unwrap()
}

/// Package short hash of the commit
#[proc_macro]
pub fn dependency_git_short_hash_from_cargo_lock(name: TokenStream) -> TokenStream {
    let package = name.to_string();

    format!(
        r#""{}""#,
        DependencyInfo::from_cargo_lock(&package)
            .hash
            .unwrap_or_default()
    )
    .parse()
    .unwrap()
}

fn crate_version_with(s: Option<String>) -> Result<TokenStream, LexError> {
    let res = s
        .map(|rev| format!("-{}", rev))
        .unwrap_or_else(Default::default);

    format!("concat!(env!(\"CARGO_PKG_VERSION\"), \"{}\")", res).parse()
}

fn opt_tokenize(s: Option<String>) -> Result<TokenStream, LexError> {
    s.map(|s| format!("Some(\"{}\")", s))
        .unwrap_or_else(|| "None".to_owned())
        .parse()
}

#[derive(Default)]
struct DependencyInfo {
    branch: Option<String>,
    hash: Option<String>,
}
impl DependencyInfo {
    /// Get package information from Cargo.lock
    pub fn from_cargo_lock(name: &str) -> DependencyInfo {
        /// return (Branch, Hash)
        fn parse_source(source: &str) -> (Option<String>, Option<String>) {
            let source = source[source.find('"').unwrap_or(9)..].trim_matches('"');
            let branch = source.find("?branch=").map(|position| {
                let mut branch = &source[position + 8..];
                if let Some(position) = branch.find('&') {
                    branch = &branch[..position]
                } else if let Some(position) = branch.find('#') {
                    branch = &branch[..position]
                }
                branch.to_string()
            });
            let hash = source.find('#').map(|position| {
                let hash = &source[position + 1..];
                { &hash[..7] }.to_string()
            });
            (branch, hash)
        }

        let toml_string = std::fs::read_to_string("Cargo.lock").unwrap_or_default();
        toml_string
            .split("[[package]]")
            .find(|package| package.contains(&format!(r#"name = "{}""#, name)))
            .map(|package| {
                let mut lines = package
                    .lines()
                    .map(|line| line.trim())
                    .filter(|line| !line.is_empty());

                let (branch, hash) = lines
                    .find(|line| line.starts_with("source = "))
                    .map(parse_source)
                    .unwrap_or((None, None));

                DependencyInfo { branch, hash }
            })
            .unwrap_or_default()
    }
}
