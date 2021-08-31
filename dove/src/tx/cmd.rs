use crate::context::Context;
use anyhow::Error;
use structopt::StructOpt;
use std::fmt::Debug;
use std::convert::TryFrom;
use crate::tx::parser::{parse_call, Call, parse_tp_param};
use lang::compiler::mut_string::MutString;
use lang::compiler::preprocessor::normalize_source_text;

/// Call declaration.
#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
pub struct CallDeclarationCmd {
    #[structopt(help = r#"Call declaration
Examples:
      'create_balance<0x01::Dfinance::USD>([10,10], true, 68656c6c6f776f726c64, 100)'
      'script_name()'
      'Module::function()'
      '0x1::Module::function()'
      '0x1::Module::function -a [10,10] true 68656c6c6f776f726c64 100 0x1 -type 0x01::Dfinance::USD'
      "#)]
    call: String,
    #[structopt(
        help = r#"Script type parametrs, e.g. 0x1::Dfinance::USD"#,
        name = "Script type parameters.",
        long = "type",
        short = "t"
    )]
    type_parameters: Option<Vec<String>>,
    #[structopt(
        help = r#"Script arguments, e.g. 10 20 30"#,
        name = "Script arguments.",
        long = "args",
        short = "a"
    )]
    args: Option<Vec<String>>,
    #[structopt(help = "File name.", long = "file", short = "f")]
    file_name: Option<String>,
}

impl TryFrom<(&Context, CallDeclarationCmd)> for CallDeclaration {
    type Error = Error;

    fn try_from((ctx, cmd): (&Context, CallDeclarationCmd)) -> Result<Self, Self::Error> {
        let sender = ctx.account_address_str()?;
        let mut call = parse_call(ctx.dialect.as_ref(), &sender, &cmd.call)?;

        if let Some(mut cmd_args) = cmd.args {
            for arg in cmd_args.as_mut_slice() {
                let mut mut_source = MutString::new(arg);
                normalize_source_text(ctx.dialect.as_ref(), (arg, &mut mut_source), &sender);
                let new_arg = mut_source.freeze();
                *arg = new_arg;
            }

            call.set_args(cmd_args);
        }

        if let Some(tp) = cmd.type_parameters {
            call.set_tp_params(
                tp.iter()
                    .map(|tp| {
                        let mut mut_source = MutString::new(tp);
                        normalize_source_text(
                            ctx.dialect.as_ref(),
                            (tp, &mut mut_source),
                            &sender,
                        );
                        let tp = mut_source.freeze();
                        parse_tp_param(&tp)
                    })
                    .collect::<Result<_, _>>()?,
            );
        }

        Ok(CallDeclaration {
            call,
            file_name: cmd.file_name,
        })
    }
}

/// Call declaration.
pub struct CallDeclaration {
    /// Call declaration.
    pub call: Call,
    /// Execution unit file name.
    pub file_name: Option<String>,
}
