use anyhow::Result;

use structopt::StructOpt;
use crate::cmd::Cmd;
use crate::context::Context;
use move_model::model::GlobalEnv;
use crate::docs::generate_docs;
use std::fs;

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

        // // Build project index...
        // let mut index = ctx.build_index()?;
        //
        // // Load dependencies by set of path...
        // let dep_set = index.make_dependency_set(&dirs)?;
        // let dep_list = load_dependencies(dep_set)?;
        //
        // let source_list = load_move_files(&dirs)?;
        //
        // let source_ref = source_list.iter().collect::<Vec<_>>();
        //
        // // Build move files...
        // let sender = ctx.account_address()?;
        // let Artifacts {
        //     files: _,
        //     env,
        //     prog: _,
        // } = MoveBuilder::new(
        //     ctx.dialect.as_ref(),
        //     Some(sender),
        //     StaticResolver::new(dep_list),
        // )
        // .build(&source_ref, true);
        //
        // if let Some(env) = env {
        //     Self::gen(&ctx, env)
        // } else {
        //     unreachable!()
        // }
        todo!()
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
