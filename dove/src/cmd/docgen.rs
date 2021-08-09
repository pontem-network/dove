use anyhow::Result;

use structopt::StructOpt;
use crate::cmd::Cmd;
use crate::context::Context;
use move_model::model::GlobalEnv;
use crate::docs::generate_docs;
use std::fs;
use lang::compiler::build_global_env;
use lang::compiler::file::find_move_files;

/// Generate documentation.
#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
pub struct DocGen {}

impl Cmd for DocGen {
    fn apply(self, ctx: Context) -> Result<()> {
        let dirs = ctx.paths_for(&[
            &ctx.manifest.layout.scripts_dir,
            &ctx.manifest.layout.modules_dir,
        ]);

        let (index, _) = ctx.build_index()?;
        let dep_list = index.into_deps_roots();
        let sender = ctx.account_address_str()?;

        let source_list = find_move_files(&dirs)
            .map(|path| path.to_string_lossy().to_string())
            .collect::<Vec<_>>();

        let env = build_global_env(source_list, dep_list, ctx.dialect.as_ref(), &sender)?;

        Self::gen(&ctx, env)
    }
}

impl DocGen {
    /// Generate documentation
    pub fn gen(ctx: &Context, env: GlobalEnv) -> Result<()> {
        let docs_output = ctx.path_for(&ctx.manifest.layout.docs_output);
        if docs_output.exists() {
            fs::remove_dir_all(&docs_output)?;
        }
        let modules_dir = ctx.path_for(&ctx.manifest.layout.modules_dir);
        let scripts_dir = ctx.path_for(&ctx.manifest.layout.scripts_dir);

        let names = vec![
            modules_dir.to_string_lossy().to_string(),
            scripts_dir.to_string_lossy().to_string(),
        ];
        generate_docs(
            &env,
            &ctx.manifest.doc,
            names,
            docs_output.to_string_lossy().to_string(),
        )
    }
}
