use anyhow::Error;
use structopt::StructOpt;
use std::fmt::Debug;
use std::convert::TryFrom;
use std::mem;
use move_package::source_package::parsed_manifest::AddressDeclarations;
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
        long = "parameters",
        short = "p"
    )]
    params: Option<Vec<String>>,
    #[structopt(
        help = r#"Move package name"#,
        name = "Move package name.",
        long = "package",
        short = "c"
    )]
    package: Option<String>,
}

impl CallDeclarationCmd {
    /// Takes call data.
    pub fn take(&mut self) -> Self {
        Self {
            call: mem::take(&mut self.call),
            type_parameters: self.type_parameters.take(),
            params: self.params.take(),
            package: self.package.take(),
        }
    }
}

impl TryFrom<(&AddressDeclarations, CallDeclarationCmd)> for CallDeclaration {
    type Error = Error;

    fn try_from(
        (addr_map, cmd): (&AddressDeclarations, CallDeclarationCmd),
    ) -> Result<Self, Self::Error> {
        let mut call = parse_call(addr_map, &cmd.call)?;
        if let Some(args) = cmd.params {
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
            package: cmd.package,
        })
    }
}

/// Call declaration.
#[derive(Debug)]
pub struct CallDeclaration {
    /// Call declaration.
    pub call: Call,
    /// Package
    pub package: Option<String>,
}
