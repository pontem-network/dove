use std::str::FromStr;
use std::fmt::Debug;
use anyhow::{bail, anyhow, ensure, Error};
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::TypeTag;
use diem_types::account_config::{diem_root_address, treasury_compliance_account_address};
use crate::compiler::metadata::FuncMeta;
use crate::tx::model::{Signer, Signers, ScriptArg, Address};
use crate::tx::parser::parse_vec;

/// Transaction config.
pub struct Config {
    /// Allow only functions with script visibility to be used.
    pub script_func_only: bool,
    /// Prohibit the definition of signers.
    pub deny_signers_definition: bool,
    /// Create execution context.
    pub exe_context: bool,
}

impl Config {
    /// Returns transaction config for chain transaction.
    pub fn for_tx() -> Config {
        Config {
            script_func_only: true,
            deny_signers_definition: true,
            exe_context: false,
        }
    }

    /// Returns transaction config for local execution.
    pub fn for_run() -> Config {
        Config {
            script_func_only: false,
            deny_signers_definition: false,
            exe_context: true,
        }
    }
}

pub fn select_function<T>(
    mut func: Vec<(T, FuncMeta)>,
    addr: AccountAddress,
    type_tag: &[TypeTag],
    args: &[String],
    cfg: &Config,
) -> Result<(T, FuncMeta), Error> {
    if func.is_empty() {
        bail!("Couldn't find a function with given signature.");
    } else if func.len() > 1 {
        let mut func = func
            .into_iter()
            .filter(|(_, f)| f.type_parameters.len() == type_tag.len())
            .filter(|(_, f)| {
                prepare_function_signature(
                    &f.parameters,
                    args,
                    !cfg.deny_signers_definition,
                    addr,
                )
                .is_ok()
            })
            .collect::<Vec<_>>();
        if func.is_empty() {
            bail!("Couldn't find a function with given signature.");
        } else if func.len() > 1 {
            bail!(
                "More than one functions with the given signature was found.\
                   Please pass the file path to specify the module. -f FILE_NAME"
            );
        } else {
            Ok(func.remove(0))
        }
    } else {
        Ok(func.remove(0))
    }
}

pub fn prepare_function_signature(
    code_args: &[(String, String)],
    call_args: &[String],
    use_explicit_signers: bool,
    addr: AccountAddress,
) -> Result<(Signers, Vec<ScriptArg>), Error> {
    let signers_count = code_args
        .iter()
        .take_while(|(_, tp)| tp == "signer")
        .count();
    let params_count = code_args.len() - signers_count;

    if call_args.len() < params_count {
        bail!(
            "The function accepts {} parameters, {} are passed",
            params_count,
            call_args.len()
        );
    }

    let args_index = call_args.len() - params_count;
    let params = code_args[signers_count..]
        .iter()
        .zip(&call_args[args_index..])
        .map(|((name, tp), val)| prepare_arg(name, tp, val))
        .collect::<Result<Vec<_>, Error>>()?;

    if use_explicit_signers {
        let signers = call_args[..args_index]
            .iter()
            .map(|arg| {
                if arg.starts_with("0x") {
                    AccountAddress::from_hex_literal(arg)
                        .map_err(|err| anyhow!("Failed to parse signer:{}", err))
                } else {
                    Signer::from_str(arg).map(|s| match s {
                        Signer::Root => diem_root_address(),
                        Signer::Treasury => treasury_compliance_account_address(),
                        Signer::Placeholder => addr,
                    })
                }
            })
            .collect::<Result<Vec<AccountAddress>, Error>>()
            .map_err(|err| anyhow!("Failed to parse signer:{}", err))?;
        ensure!(
            signers.len() == signers_count,
            "The function accepts {} signers, {} are passed",
            signers_count,
            signers.len()
        );
        Ok((Signers::Explicit(signers), params))
    } else {
        let mut signers = (0..signers_count)
            .take_while(|i| *i < call_args.len())
            .map(|i| Signer::from_str(&call_args[i]).ok())
            .take_while(|s| s.is_some())
            .flatten()
            .collect::<Vec<_>>();
        let explicit_signers = signers.len();

        for _ in explicit_signers..signers_count {
            signers.push(Signer::Placeholder);
        }

        Ok((Signers::Implicit(signers), params))
    }
}

fn prepare_arg(arg_name: &str, arg_type: &str, arg_value: &str) -> Result<ScriptArg, Error> {
    fn parse_err<D: Debug>(name: &str, tp: &str, value: &str, err: D) -> Error {
        anyhow!(
            "Parameter '{}' has type {}. Failed to parse {}. Error:'{:?}'",
            name,
            tp,
            value,
            err
        )
    }
    macro_rules! parse_primitive {
        ($script_arg:expr) => {
            $script_arg(
                arg_value
                    .parse()
                    .map_err(|err| parse_err(arg_name, arg_type, arg_value, err))?,
            )
        };
    }

    Ok(match arg_type {
        "bool" => parse_primitive!(ScriptArg::Bool),
        "u8" => parse_primitive!(ScriptArg::U8),
        "u64" => parse_primitive!(ScriptArg::U64),
        "u128" => parse_primitive!(ScriptArg::U128),
        "address" => ScriptArg::Address(
            AccountAddress::from_hex_literal(arg_value)
                .map_err(|err| parse_err(arg_name, arg_type, arg_value, err))?,
        ),
        "vector<bool>" => ScriptArg::VectorBool(
            parse_vec(arg_value, "bool")
                .map_err(|err| parse_err(arg_name, arg_type, arg_value, err))?,
        ),
        "vector<u8>" => {
            let vec = if arg_value.contains('[') {
                parse_vec(arg_value, "u8")
                    .map_err(|err| parse_err(arg_name, arg_type, arg_value, err))?
            } else {
                hex::decode(arg_value)
                    .map_err(|err| parse_err(arg_name, arg_type, arg_value, err))?
            };
            ScriptArg::VectorU8(vec)
        }
        "vector<u64>" => ScriptArg::VectorU64(
            parse_vec(arg_value, "u64")
                .map_err(|err| parse_err(arg_name, arg_type, arg_value, err))?,
        ),
        "vector<u128>" => ScriptArg::VectorU128(
            parse_vec(arg_value, "u64")
                .map_err(|err| parse_err(arg_name, arg_type, arg_value, err))?,
        ),
        "vector<address>" => {
            let addresses = parse_vec::<Address>(arg_value, "vector<address>")
                .map_err(|err| parse_err(arg_name, arg_type, arg_value, err))?
                .iter()
                .map(|addr| addr.addr)
                .collect();
            ScriptArg::VectorAddress(addresses)
        }
        other => anyhow::bail!("Unexpected script parameter: {}", other),
    })
}

#[cfg(test)]
mod call_tests {
    use diem_types::account_config::{diem_root_address, treasury_compliance_account_address};
    use move_core_types::language_storage::CORE_CODE_ADDRESS;
    use move_core_types::account_address::AccountAddress;
    use crate::tx::model::{Signer, Signers, ScriptArg};
    use super::prepare_function_signature;

    fn s(v: &str) -> String {
        v.to_string()
    }

    fn param(n: &str, t: &str) -> (String, String) {
        (s(n), s(t))
    }

    fn addr(v: &str) -> AccountAddress {
        AccountAddress::from_hex_literal(v).unwrap()
    }

    #[test]
    fn test_args_types() {
        let (signers, args) =
            prepare_function_signature(&[], &[], true, CORE_CODE_ADDRESS).unwrap();
        assert_eq!(signers.len(), 0);
        assert_eq!(args.len(), 0);

        let (signers, args) =
            prepare_function_signature(&[param("val", "u8")], &[s("1")], true, CORE_CODE_ADDRESS)
                .unwrap();
        assert_eq!(signers.len(), 0);
        assert_eq!(args, vec![ScriptArg::U8(1)]);

        let (signers, args) = prepare_function_signature(
            &[param("d", "bool"), param("t", "bool")],
            &[s("true"), s("false")],
            true,
            CORE_CODE_ADDRESS,
        )
        .unwrap();
        assert_eq!(signers.len(), 0);
        assert_eq!(args, vec![ScriptArg::Bool(true), ScriptArg::Bool(false)]);

        let (signers, args) = prepare_function_signature(
            &[param("d", "u64"), param("t", "u64"), param("r", "u128")],
            &[s("0"), s("1000000000"), s("10000000000000000")],
            true,
            CORE_CODE_ADDRESS,
        )
        .unwrap();
        assert_eq!(signers.len(), 0);
        assert_eq!(
            args,
            vec![
                ScriptArg::U64(0),
                ScriptArg::U64(1000000000),
                ScriptArg::U128(10000000000000000),
            ]
        );

        let (signers, args) = prepare_function_signature(
            &[param("d", "address")],
            &[s("0x1")],
            true,
            CORE_CODE_ADDRESS,
        )
        .unwrap();
        assert_eq!(signers.len(), 0);
        assert_eq!(args, vec![ScriptArg::Address(CORE_CODE_ADDRESS)]);

        let (signers, args) = prepare_function_signature(
            &[
                param("b", "vector<bool>"),
                param("d", "vector<u8>"),
                param("q", "vector<u8>"),
                param("q1", "vector<u8>"),
                param("w", "vector<u64>"),
                param("l", "vector<u128>"),
                param("a", "vector<address>"),
            ],
            &[
                s("[true, false]"),
                s("[100]"),
                s("[]"),
                s("0102"),
                s("[1000, 0]"),
                s("[0]"),
                s("[0x1, 0x2]"),
            ],
            true,
            CORE_CODE_ADDRESS,
        )
        .unwrap();
        assert_eq!(signers.len(), 0);
        assert_eq!(
            args,
            vec![
                ScriptArg::VectorBool(vec![true, false]),
                ScriptArg::VectorU8(vec![100]),
                ScriptArg::VectorU8(vec![]),
                ScriptArg::VectorU8(vec![1, 2]),
                ScriptArg::VectorU64(vec![1000, 0]),
                ScriptArg::VectorU128(vec![0]),
                ScriptArg::VectorAddress(vec![addr("0x1"), addr("0x2")]),
            ]
        );
    }

    #[test]
    fn test_signers() {
        let (signers, args) = prepare_function_signature(
            &[
                param("val", "signer"),
                param("val", "signer"),
                param("val", "signer"),
                param("val", "signer"),
                param("val", "signer"),
            ],
            &[s("_"), s("0x2"), s("root"), s("tr"), s("_")],
            true,
            CORE_CODE_ADDRESS,
        )
        .unwrap();
        assert_eq!(
            signers,
            Signers::Explicit(vec![
                addr("0x1"),
                addr("0x2"),
                diem_root_address(),
                treasury_compliance_account_address(),
                addr("0x1"),
            ])
        );
        assert_eq!(args.len(), 0);

        let (signers, args) = prepare_function_signature(
            &[
                param("val", "signer"),
                param("val", "signer"),
                param("val", "signer"),
                param("val", "signer"),
                param("val", "signer"),
                param("val", "u8"),
            ],
            &[s("_"), s("0x2"), s("root"), s("tr"), s("_"), s("1")],
            true,
            CORE_CODE_ADDRESS,
        )
        .unwrap();
        assert_eq!(
            signers,
            Signers::Explicit(vec![
                addr("0x1"),
                addr("0x2"),
                diem_root_address(),
                treasury_compliance_account_address(),
                addr("0x1"),
            ])
        );
        assert_eq!(args, vec![ScriptArg::U8(1)]);

        let (signers, args) = prepare_function_signature(
            &[
                param("val", "signer"),
                param("val", "signer"),
                param("val", "signer"),
                param("val", "signer"),
                param("val", "u8"),
            ],
            &[s("_"), s("root"), s("tr"), s("_"), s("1")],
            false,
            CORE_CODE_ADDRESS,
        )
        .unwrap();
        assert_eq!(
            signers,
            Signers::Implicit(vec![
                Signer::Placeholder,
                Signer::Root,
                Signer::Treasury,
                Signer::Placeholder,
            ])
        );
        assert_eq!(args, vec![ScriptArg::U8(1)]);

        let (signers, args) = prepare_function_signature(
            &[
                param("val", "signer"),
                param("val", "signer"),
                param("val", "signer"),
                param("val", "signer"),
                param("val", "signer"),
                param("val", "u8"),
            ],
            &[s("_"), s("root"), s("tr"), s("_"), s("1")],
            false,
            CORE_CODE_ADDRESS,
        )
        .unwrap();
        assert_eq!(
            signers,
            Signers::Implicit(vec![
                Signer::Placeholder,
                Signer::Root,
                Signer::Treasury,
                Signer::Placeholder,
                Signer::Placeholder,
            ])
        );
        assert_eq!(args, vec![ScriptArg::U8(1)]);
    }
}
