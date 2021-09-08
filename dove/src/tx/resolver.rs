use std::fs;
use std::path::PathBuf;
use anyhow::Error;
use regex::Regex;
use move_core_types::account_address::AccountAddress;
use move_core_types::identifier::Identifier;
use lang::compiler::file::find_move_files;
use lang::compiler::metadata::{module_meta, script_meta, FuncMeta};
use crate::context::Context;

pub(crate) fn find_module_function(
    ctx: &Context,
    address: &AccountAddress,
    m_name: &Identifier,
    f_name: &Identifier,
    file: &Option<String>,
    script_only: bool,
) -> Result<Vec<(PathBuf, FuncMeta)>, Error> {
    let move_files = if let Some(file) = file {
        vec![ctx.path_for(&ctx.manifest.layout.modules_dir).join(file)]
    } else {
        let (_, interface) = ctx.build_index()?;
        let mut roots = vec![interface.dir.to_string_lossy().to_string()];
        roots.push(ctx.str_path_for(&ctx.manifest.layout.modules_dir)?);
        find_move_files(&roots).collect()
    };

    let move_files = find_by_regexp(
        move_files,
        &format!(r#"module([\s]+|[\s]+[\dA-Za-z{{}}]+::){}[\s]+\{{"#, m_name),
    )?;

    let sender = ctx.account_address_str()?;

    Ok(move_files
        .iter()
        .filter_map(|f| {
            module_meta(&f.to_string_lossy(), ctx.dialect.as_ref(), &sender)
                .ok()
                .map(|m| (f, m))
        })
        .flat_map(|(p, m)| {
            m.into_iter()
                .filter(|m| m.value.address == *address && &m.value.name == m_name)
                .flat_map(|m| m.value.funs)
                .filter(|f| &f.value.name == f_name)
                .filter(|f| {
                    if script_only {
                        f.value.visibility.is_script()
                    } else {
                        false
                    }
                })
                .map(|f| (p.to_owned(), f))
                .collect::<Vec<_>>()
        })
        .collect())
}

pub(crate) fn find_script(
    ctx: &Context,
    name: &Identifier,
    file: Option<String>,
) -> Result<Vec<(PathBuf, FuncMeta)>, Error> {
    let move_files = if let Some(file) = file {
        let mut file = ctx.path_for(&ctx.manifest.layout.scripts_dir).join(file);
        file.set_extension("move");
        vec![file]
    } else {
        find_move_files(&[ctx.path_for(&ctx.manifest.layout.scripts_dir)]).collect()
    };
    let move_files = find_by_regexp(move_files, &format!(r#"fun[\s]+{}"#, name.as_str()))?;
    let sender = ctx.account_address_str()?;
    Ok(move_files
        .iter()
        .filter_map(|p| {
            script_meta(&p.to_string_lossy(), ctx.dialect.as_ref(), &sender)
                .ok()
                .map(|f| (p, f))
        })
        .flat_map(|(p, m)| {
            m.into_iter()
                .filter(|m| &m.value.name == name)
                .map(|m| (p.to_owned(), m))
                .collect::<Vec<_>>()
        })
        .collect())
}

fn find_by_regexp(move_files: Vec<PathBuf>, regex: &str) -> Result<Vec<PathBuf>, Error> {
    let regexp = Regex::new(regex)?;
    let (mtch, not_mtch) = move_files
        .into_iter()
        .filter(|f| f.exists())
        .partition::<Vec<_>, _>(|f| {
            if let Ok(content) = fs::read_to_string(&f) {
                regexp.find(&content).is_some()
            } else {
                false
            }
        });

    Ok(if mtch.is_empty() { not_mtch } else { mtch })
}
