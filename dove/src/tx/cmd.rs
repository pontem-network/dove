use crate::context::Context;
use anyhow::Error;
use structopt::StructOpt;
use std::fmt::Debug;
use std::convert::TryFrom;
use std::mem;
use crate::tx::parser::{parse_call, Call, parse_tp_param};

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

impl CallDeclarationCmd {
    pub fn take(&mut self) -> Self {
        Self {
            call: mem::take(&mut self.call),
            type_parameters: self.type_parameters.take(),
            args: self.args.take(),
            file_name: self.file_name.take(),
        }
    }
}

impl TryFrom<CallDeclarationCmd> for CallDeclaration {
    type Error = Error;

    fn try_from(cmd: CallDeclarationCmd) -> Result<Self, Self::Error> {
        let mut call = parse_call(&cmd.call)?;
        if let Some(args) = cmd.args {
            call.set_args(args);
        }

        if let Some(tp) = cmd.type_parameters {
            call.set_tp_params(
                tp.iter()
                    .map(|tp| parse_tp_param(tp))
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
