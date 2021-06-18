use std::path::{Path, PathBuf};
use std::str::FromStr;

use anyhow::{Error, Result};
use serde::Serialize;
use structopt::StructOpt;

use lang::compiler::dialects::*;
use crate::cmd::Cmd;
use crate::context::{Context, str_path, load_manifest, get_context};
use crate::index::resolver::git;
use crate::manifest::{Dependence, Git, Layout, MANIFEST, read_manifest, DoveToml};
use crate::stdoutln;

fn into_metadata(mut ctx: Context) -> Result<DoveMetadata, Error> {
    let layout = ctx.manifest.layout.to_absolute(&ctx)?;

    let mut local_deps = vec![];
    let mut git_deps = vec![];
    if let Some(dependencies) = ctx.manifest.package.dependencies.take() {
        for dep in dependencies.deps {
            match dep {
                Dependence::Git(git) => {
                    git_deps.push(GitMetadata::new(git, &ctx)?);
                }
                Dependence::Path(dep_path) => {
                    local_deps.push(ctx.str_path_for(&dep_path.path)?);
                }
            }
        }
    }

    let package_metadata = PackageMetadata {
        name: ctx.project_name(),
        account_address: ctx.manifest.package.account_address,
        blockchain_api: ctx.manifest.package.blockchain_api,
        git_dependencies: git_deps,
        local_dependencies: local_deps,
        dialect: ctx.dialect.name().to_string(),
    };
    Ok(DoveMetadata {
        package: package_metadata,
        layout,
    })
}

/// Metadata project command.
#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
pub struct Metadata {
    #[structopt(long, hidden = true)]
    color: Option<String>,
    /// Validate dove manifest
    #[structopt(long)]
    validate: bool,
}

impl Cmd for Metadata {
    fn apply(self, ctx: Context) -> Result<(), Error> {
        if self.validate {
            self.validate_manifest(ctx).or_else(|err| {
                err.print();
                Ok(())
            })
        } else {
            self.output_manifest(ctx)
        }
    }

    /// Returns project context.
    fn context(&self, project_dir: PathBuf) -> Result<Context> {
        if self.validate {
            get_context(project_dir, DoveToml::default())
        } else {
            let manifest = load_manifest(&project_dir)?;
            get_context(project_dir, manifest)
        }
    }
}

impl Metadata {
    /// output project metadata in json format
    fn output_manifest(&self, ctx: Context) -> Result<(), Error> {
        let metadata = into_metadata(ctx)?;
        stdoutln!(
            "{}",
            serde_json::to_string_pretty::<DoveMetadata>(&metadata)?
        );
        Ok(())
    }

    /// Validate manifest. Result output in json format
    fn validate_manifest(&self, mut ctx: Context) -> Result<(), ValidateJson> {
        let project_dir = ctx.project_dir;
        let manifest_path = project_dir.join(MANIFEST);
        if !manifest_path.exists() {
            return Err(ValidateJson::new_error(1, "Dove.toml not found", None));
        }
        let (dove_str, manifest) = self.get_manifest(&manifest_path)?;
        let dialect_name = self.get_dialect_name(&manifest, &dove_str)?;
        ctx = Context {
            project_dir,
            manifest,
            dialect: dialect_name.get_dialect(),
        };

        ctx.manifest.package.dependencies.map_or(Ok(()), |dp| {
            dp.deps
                .iter()
                .find_map(|dep| find_error_for_dep(dep, &dove_str))
                .map_or(Ok(()), Err)
        })?;

        ValidateJson::new_valid().print();
        Ok(())
    }

    fn get_manifest(&self, manifest_path: &Path) -> Result<(String, DoveToml), ValidateJson> {
        let dove_str = std::fs::read_to_string(manifest_path)
            .map_err(|err| ValidateJson::new_error(2, &err.to_string(), None))?;
        let dove_toml = toml::from_str::<DoveToml>(&dove_str).map_err(|err| {
            let offset = err
                .line_col()
                .map(|(line, col)| get_offset_by_line_and_col(&dove_str, line, col));
            ValidateJson::new_error(3, &err.to_string(), offset)
        })?;
        Ok((dove_str, dove_toml))
    }

    fn get_dialect_name(
        &self,
        dove_toml: &DoveToml,
        dove_toml_str: &str,
    ) -> Result<DialectName, ValidateJson> {
        match dove_toml.package.dialect.as_ref() {
            Some(dialect_string) => DialectName::from_str(dialect_string).map_err(|err| {
                let offset = dove_toml_str
                    .to_lowercase()
                    .find("dialect")
                    .map(|mut offset| {
                        offset = dove_toml_str[offset..].find('\"').unwrap_or(0) + offset;
                        get_line_and_col_by_offset(dove_toml_str, offset)
                    });
                ValidateJson::new_error(5, &err.to_string(), offset)
            }),
            None => Err(ValidateJson::new_error(4, "dialect not set", None)),
        }
    }
}

/// Move manifest.
#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct DoveMetadata {
    /// Project info.
    pub package: PackageMetadata,
    /// Project layout.
    #[serde(default)]
    pub layout: Layout,
}

/// Project info.
#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct PackageMetadata {
    /// Project name.
    pub name: String,
    /// Project AccountAddress.
    #[serde(default = "code_code_address")]
    pub account_address: String,
    /// dnode base url.
    pub blockchain_api: Option<String>,
    /// Git dependency list.
    pub git_dependencies: Vec<GitMetadata>,
    /// Local dependency list.
    pub local_dependencies: Vec<String>,
    /// Dialect used in the project.
    pub dialect: String,
}

/// Git dependency metadata.
#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct GitMetadata {
    /// Git url.
    pub git: String,
    /// Branch name.
    pub branch: Option<String>,
    /// Commit hash.
    pub rev: Option<String>,
    /// Tag.
    pub tag: Option<String>,
    /// Path.
    pub path: Option<String>,
    /// Local path.
    pub local_paths: Vec<String>,
}

impl GitMetadata {
    /// Create a new git metadata.
    pub fn new(git: Git, ctx: &Context) -> Result<GitMetadata, Error> {
        let path: &Path = ctx.manifest.layout.deps.as_ref();
        let pac_path = ctx.path_for(path.join(&git::make_local_name(&git)));
        let mut local_paths = Vec::new();

        if pac_path.exists() {
            let manifest_path = pac_path.join(MANIFEST);

            let manifest = if manifest_path.exists() {
                match read_manifest(&manifest_path) {
                    Ok(manifest) => Some(manifest),
                    Err(err) => {
                        log::error!("Failed to parse dove manifest:{:?}. Err:{}", pac_path, err);
                        None
                    }
                }
            } else {
                None
            };

            if let Some(manifest) = manifest {
                let modules_dir = pac_path.join(manifest.layout.modules_dir);
                if modules_dir.exists() {
                    local_paths.push(str_path(modules_dir)?);
                }
                if let Some(deps) = manifest.package.dependencies {
                    for dep in deps.deps.iter() {
                        if let Dependence::Path(path) = dep {
                            let local_dep_path = pac_path.join(path.as_ref()).canonicalize();
                            if let Ok(local_dep_path) = local_dep_path {
                                if local_dep_path.starts_with(&pac_path) {
                                    local_paths.push(str_path(local_dep_path)?);
                                    continue;
                                }
                            }

                            log::warn!(
                                "Package '{}' has invalid dependency:{:?}.",
                                git.git,
                                path
                            );
                        }
                    }
                }
            } else {
                local_paths.push(str_path(pac_path)?);
            }
        }

        Ok(GitMetadata {
            git: git.git,
            branch: git.branch,
            rev: git.rev,
            tag: git.tag,
            path: git.path,
            local_paths,
        })
    }
}

/// Answer for --validate
#[derive(Debug, Serialize)]
struct ValidateJson {
    code: u8,
    error: Option<ValidateJsonError>,
}
impl ValidateJson {
    pub fn new_error(
        code: u8,
        message: &str,
        offset: Option<(usize, usize, usize)>,
    ) -> ValidateJson {
        ValidateJson {
            code,
            error: Some(ValidateJsonError {
                message: message.to_string(),
                line: offset.map(|v| v.0),
                column: offset.map(|v| v.1),
                offset: offset.map(|v| v.2),
            }),
        }
    }

    pub fn new_valid() -> ValidateJson {
        ValidateJson {
            code: 0,
            error: None,
        }
    }

    pub fn to_string(&self) -> Result<String> {
        serde_json::to_string_pretty(self).map_err(|err| anyhow!(err.to_string()))
    }

    pub fn print(&self) {
        stdoutln!("{}", self.to_string().unwrap());
    }
}
#[derive(Debug, Serialize)]
struct ValidateJsonError {
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    column: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<usize>,
}

/// Convert line, column => (line, column, offset)
fn get_offset_by_line_and_col(text: &str, line: usize, col: usize) -> (usize, usize, usize) {
    if line == 0 {
        return (0, 0, 0);
    }
    (
        line,
        col,
        text.chars()
            .enumerate()
            .filter_map(|(p, ch)| if ch == '\n' { Some(p) } else { None })
            .nth(line - 1)
            .unwrap_or_default()
            + 1
            + col,
    )
}
/// Convert offset => (line, column, offset)
fn get_line_and_col_by_offset(text: &str, offset: usize) -> (usize, usize, usize) {
    (
        text[..offset].lines().count() - 1,
        text[..offset].chars().count() - text[..offset].rfind('\n').unwrap_or(0) - 1,
        offset,
    )
}

fn find_error_for_dep(dep: &Dependence, dove_str: &str) -> Option<ValidateJson> {
    let check_path = |path: &str| -> Option<ValidateJson> {
        if Path::new(path).exists() {
            return None;
        }
        let message = format!("Path {:?} not found", path);
        let offset = dove_str
            .find(path)
            .map(|offset| get_line_and_col_by_offset(dove_str, offset));
        Some(ValidateJson::new_error(6, &message, offset))
    };

    match dep {
        Dependence::Git(git_data) => git_data.path.as_ref().and_then(|t| check_path(t)),
        Dependence::Path(dep_path) => check_path(&dep_path.path),
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use crate::context::{get_context, load_manifest};

    use super::*;

    #[test]
    fn paths_in_metadata_are_absolute() {
        fn check_absolute<P: AsRef<Path>>(path: &P) {
            let path = path.as_ref();
            assert!(
                path.starts_with(env!("CARGO_MANIFEST_DIR")),
                "Path {:?} is not absolute",
                path
            );
        }

        let move_project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("resources")
            .join("test_move_project");
        let manifest = load_manifest(&move_project_dir).unwrap();
        let context = get_context(move_project_dir, manifest).unwrap();

        let metadata = into_metadata(context).unwrap();

        assert_eq!(metadata.package.dialect, "pont".to_string());

        // non-existent paths ain't present in the metadata
        assert_eq!(metadata.package.local_dependencies.len(), 2);
        check_absolute(&metadata.package.local_dependencies[0]);
        check_absolute(&metadata.package.local_dependencies[1]);

        check_absolute(&metadata.layout.modules_dir);
        check_absolute(&metadata.layout.scripts_dir);
        check_absolute(&metadata.layout.tests_dir);
        check_absolute(&metadata.layout.modules_output);
        check_absolute(&metadata.layout.bundles_output);
        check_absolute(&metadata.layout.scripts_output);
        check_absolute(&metadata.layout.transactions_output);
        check_absolute(&metadata.layout.deps);
        check_absolute(&metadata.layout.artifacts);
        check_absolute(&metadata.layout.index);
    }
}
