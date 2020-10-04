use anyhow::Result;
use std::path::Path;
use crate::manifest::MoveToml;
use crate::builder::Builder;
use crate::cmd::fetch::make_rest_loader;

/// Execute check command.
pub fn execute(project_dir: &Path, manifest: MoveToml) -> Result<()> {
    let loader = make_rest_loader(project_dir, &manifest)?;
    let builder = Builder::new(project_dir, manifest, &loader, true, true);
    builder.init_build_layout()?;

    let source_map = builder.make_source_map()?;
    let bytecode_map = builder.load_dependencies(&source_map)?;
    let dep_list = builder.prepare_deps(bytecode_map)?;

    builder.check(source_map, dep_list)
}
