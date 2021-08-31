use move_lang::compiled_unit::CompiledUnit;
use anyhow::Error;
use codespan_reporting::term::termcolor::{StandardStream, ColorChoice};
use move_lang::errors::output_errors;
use move_lang::shared::Flags;
use crate::context::Context;
use lang::compiler::build;
use move_lang::compiled_unit;

pub(crate) fn move_build(
    ctx: &Context,
    targets: &[String],
    deps: &[String],
) -> Result<Vec<CompiledUnit>, Error> {
    let sender = ctx.account_address_str()?;
    let (files, prog) = build(
        targets,
        deps,
        ctx.dialect.as_ref(),
        &sender,
        None,
        Flags::empty(),
    )?;

    match prog {
        Err(errors) => {
            let mut writer = StandardStream::stderr(ColorChoice::Auto);
            output_errors(&mut writer, files, errors);
            anyhow::bail!("Could not compile:{}", ctx.project_name())
        }
        Ok(compiled_units) => {
            let (compiled_units, ice_errors) = compiled_unit::verify_units(compiled_units);

            if !ice_errors.is_empty() {
                let mut writer = StandardStream::stderr(ColorChoice::Auto);
                output_errors(&mut writer, files, ice_errors);
                anyhow::bail!("could not verify:{}", ctx.project_name())
            } else {
                Ok(compiled_units)
            }
        }
    }
}
