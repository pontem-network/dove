use crate::cmd::{Cmd, load_dependencies};
use crate::context::Context;
use anyhow::Error;
use structopt::StructOpt;
use lang::compiler::file::{MoveFile, find_move_files, load_move_files};
use lang::flow::meta_extractor::{ScriptMetadata, Meta};
use lang::flow::builder::{Artifacts, MoveBuilder, StaticResolver};
use termcolor::{StandardStream, ColorChoice};
use move_core_types::language_storage::TypeTag;
use serde::{Serialize, Deserialize};
use move_lang::parser::lexer::{Lexer, Tok};
use move_lang::parser::syntax::parse_type;
use move_lang::{compiled_unit, errors::output_errors};
use std::fmt::Debug;
use std::str::FromStr;
use lang::compiler::address::ss58::{ss58_to_diem, replace_ss58_addresses};
use std::fs;
use move_lang::compiled_unit::CompiledUnit;
use move_core_types::account_address::AccountAddress;
use lang::lexer::unwrap_spanned_ty;
use lang::compiler::mut_string::MutString;
use move_core_types::value::MoveValue;
use crate::stdoutln;

/// Create transaction.
#[derive(StructOpt, Debug)]
pub struct CreateTransactionCmd {
    #[structopt(help = "Script call declaration.\
     Example: 'create_balance<0x01::Dfinance::USD>([10,10], true, 68656c6c6f776f726c64, 100)'")]
    call: Option<String>,
    #[structopt(help = "Script name.", long = "name", short = "n")]
    script_name: Option<String>,
    #[structopt(help = "Output file name.", long = "output", short = "o")]
    output: Option<String>,
    #[structopt(help = "Script file name.", long = "file", short = "f")]
    file_name: Option<String>,
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
}

impl Cmd for CreateTransactionCmd {
    fn apply(mut self, ctx: Context) -> Result<(), Error> {
        let output_filename = self.output.take();

        let builder = TransactionBuilder::new(self, &ctx)?;
        let (script_name, transaction) = builder.build()?;

        store_transaction(&ctx, &output_filename.unwrap_or(script_name), transaction)
    }
}

struct TransactionBuilder<'a> {
    script_file_name: Option<String>,
    script_name: Option<String>,
    type_parameters: Vec<TypeTag>,
    args: Vec<String>,
    dove_ctx: &'a Context,
}

impl<'a> TransactionBuilder<'a> {
    pub fn new(cmd: CreateTransactionCmd, ctx: &'a Context) -> Result<TransactionBuilder, Error> {
        let (mut script_name, mut type_parameters, mut args) = if let Some(call) = cmd.call {
            let (script_name, type_parameters, args) = Self::parse_call(&call)?;
            (Some(script_name), type_parameters, args)
        } else {
            (None, vec![], vec![])
        };

        if let Some(cmd_script_name) = cmd.script_name {
            script_name = Some(cmd_script_name);
        }

        if let Some(cmd_type_parameters) = cmd.type_parameters {
            type_parameters = cmd_type_parameters
                .iter()
                .map(|tp| {
                    let mut mut_string = MutString::new(tp);
                    replace_ss58_addresses(tp, &mut mut_string, &mut Default::default());
                    mut_string.freeze()
                })
                .map(|tp| parse_type_params(&mut Lexer::new(&tp, "tp", Default::default())))
                .collect::<Result<_, _>>()?;
        }

        if let Some(cmd_args) = cmd.args {
            args = cmd_args
                .iter()
                .map(|arg| {
                    let mut mut_string = MutString::new(arg);
                    replace_ss58_addresses(arg, &mut mut_string, &mut Default::default());
                    mut_string.freeze()
                })
                .collect();
        }

        Ok(TransactionBuilder {
            script_file_name: cmd.file_name,
            script_name,
            type_parameters,
            args,
            dove_ctx: ctx,
        })
    }

    pub fn parse_call(call: &str) -> Result<(String, Vec<TypeTag>, Vec<String>), Error> {
        let mut mut_string = MutString::new(call);
        replace_ss58_addresses(call, &mut mut_string, &mut Default::default());
        let call = mut_string.freeze();

        let map_err = |err| Error::msg(format!("{:?}", err));
        let mut lexer = Lexer::new(&call, "call", Default::default());
        lexer.advance().map_err(map_err)?;
        if lexer.peek() != Tok::IdentifierValue {
            return Err(anyhow!("Invalid call script format.\
             Expected function identifier. Use pattern \
             'script_name<comma separated type parameters>(comma separated parameters WITHOUT signers)'"));
        }

        let script_name = lexer.content().to_owned();

        lexer.advance().map_err(map_err)?;

        let type_parameters = if lexer.peek() == Tok::Less {
            let mut type_parameter = vec![];

            lexer.advance().map_err(map_err)?;
            while lexer.peek() != Tok::Greater {
                if lexer.peek() == Tok::EOF {
                    return Err(anyhow!("Invalid call script format.\
             Invalid type parameters format.. Use pattern \
             'script_name<comma separated type parameters>(comma separated parameters WITHOUT signers)'"));
                }

                if lexer.peek() == Tok::Comma {
                    lexer.advance().map_err(map_err)?;
                    continue;
                }

                type_parameter.push(parse_type_params(&mut lexer)?);
            }
            lexer.advance().map_err(map_err)?;
            type_parameter
        } else {
            vec![]
        };

        if lexer.peek() != Tok::LParen {
            return Err(anyhow!("Invalid call script format.\
             Invalid script arguments format.. Left paren '(' is expected. Use pattern \
             'script_name<comma separated type parameters>(comma separated parameters WITHOUT signers)'"));
        }

        let mut arguments = vec![];

        lexer.advance().map_err(map_err)?;
        while lexer.peek() != Tok::RParen {
            if lexer.peek() == Tok::EOF {
                return Err(anyhow!("Invalid call script format.\
             Invalid arguments format.. Use pattern \
             'script_name<comma separated type parameters>(comma separated parameters WITHOUT signers)'"));
            }

            if lexer.peek() == Tok::Comma {
                lexer.advance().map_err(map_err)?;
                continue;
            }

            if lexer.peek() == Tok::LBracket {
                let mut token = String::new();
                token.push_str(lexer.content());
                lexer.advance().map_err(map_err)?;
                while lexer.peek() != Tok::RBracket {
                    token.push_str(lexer.content());
                    lexer.advance().map_err(map_err)?;
                }
                token.push_str(lexer.content());
                arguments.push(token);
            } else {
                let mut token = String::new();
                token.push_str(lexer.content());
                lexer.advance().map_err(map_err)?;
                while lexer.peek() != Tok::Comma && lexer.peek() != Tok::RParen {
                    token.push_str(lexer.content());
                    lexer.advance().map_err(map_err)?;
                }
                arguments.push(token);
                if lexer.peek() == Tok::RParen {
                    break;
                }
            }
            lexer.advance().map_err(map_err)?;
        }

        Ok((script_name, type_parameters, arguments))
    }

    fn lookup_script_by_file_name(&self, fname: &str) -> Result<(MoveFile, Meta), Error> {
        let script_path = self
            .dove_ctx
            .path_for(&self.dove_ctx.manifest.layout.scripts_dir);
        let file_path = if !fname.ends_with("move") {
            let mut path = script_path.join(fname);
            path.set_extension("move");
            path
        } else {
            script_path.join(fname)
        };
        if !file_path.exists() {
            return Err(anyhow!("File [{}] not found", fname));
        }

        let sender = self.dove_ctx.account_address()?;
        let script = MoveFile::load(&file_path)?;
        let mut scripts =
            ScriptMetadata::extract(self.dove_ctx.dialect.as_ref(), Some(sender), &[&script])?;
        if scripts.is_empty() {
            return Err(anyhow!("Script not found in file '{}'", fname));
        }

        let meta = if scripts.len() > 1 {
            let mut scripts = scripts
                .into_iter()
                .filter(|sc| {
                    if let Some(script_name) = &self.script_name {
                        &sc.name == script_name
                    } else {
                        false
                    }
                })
                .collect::<Vec<_>>();
            if scripts.len() > 1 {
                return Err(anyhow!(
                    "There are several scripts with the name '{:?}' in file '{}'",
                    self.script_name,
                    fname
                ));
            } else {
                scripts.remove(0)
            }
        } else {
            scripts.remove(0)
        };

        Ok((script, meta))
    }

    fn lookup_script_by_name(&self, name: &str) -> Result<(MoveFile, Meta), Error> {
        let script_path = self
            .dove_ctx
            .path_for(&self.dove_ctx.manifest.layout.scripts_dir);
        let sender = self.dove_ctx.account_address()?;
        let mut files = find_move_files(&script_path)?
            .iter()
            .map(MoveFile::load)
            .filter_map(|mf| match mf {
                Ok(mf) => {
                    if mf.content().contains(name) {
                        Some(mf)
                    } else {
                        None
                    }
                }
                Err(err) => {
                    warn!("{:?}", err);
                    None
                }
            })
            .map(|mf| {
                ScriptMetadata::extract(self.dove_ctx.dialect.as_ref(), Some(sender), &[&mf])
                    .map(|meta| (mf, meta))
            })
            .filter_map(|script| match script {
                Ok((mf, meta)) => Some((mf, meta)),
                Err(err) => {
                    warn!("{:?}", err);
                    None
                }
            })
            .filter(|(_, meta)| meta.iter().any(|meta| *name == meta.name))
            .collect::<Vec<_>>();

        if files.is_empty() {
            return Err(anyhow!("Script not found."));
        }

        if files.len() > 1 {
            let name_list = files
                .iter()
                .map(|(mf, _)| mf.name())
                .collect::<Vec<_>>()
                .join(", ");
            return Err(anyhow!(
                "There are several scripts with the name '{:?}' in files ['{}'].",
                name,
                name_list
            ));
        }

        let (file, mut meta) = files.remove(0);
        if meta.is_empty() {
            return Err(anyhow!("Script not found."));
        }

        if meta.len() > 1 {
            return Err(anyhow!(
                "There are several scripts with the name '{:?}' in file '{}'.",
                name,
                file.name()
            ));
        }
        Ok((file, meta.remove(0)))
    }

    fn lookup_script(&self) -> Result<(MoveFile, Meta), Error> {
        if let Some(file_name) = &self.script_file_name {
            return self.lookup_script_by_file_name(file_name);
        }

        if let Some(name) = &self.script_name {
            return self.lookup_script_by_name(name);
        }

        let script_path = self
            .dove_ctx
            .path_for(&self.dove_ctx.manifest.layout.scripts_dir);
        let files = find_move_files(&script_path)?;
        if files.len() == 1 {
            let mf = MoveFile::load(&files[0])?;
            let sender = self.dove_ctx.account_address()?;
            let mut meta =
                ScriptMetadata::extract(self.dove_ctx.dialect.as_ref(), Some(sender), &[&mf])?;
            if meta.is_empty() {
                return Err(anyhow!("Script not found."));
            }
            if meta.len() > 1 {
                return Err(anyhow!("Failed to determine script. There are several scripts. Use '--name' to determine the script."));
            }
            Ok((mf, meta.remove(0)))
        } else {
            Err(anyhow!("Failed to determine script. There are several scripts. Use '--name' or '--file' to determine the script."))
        }
    }

    fn build_script(&self, script: MoveFile) -> Result<Vec<CompiledUnit>, Error> {
        let mut index = self.dove_ctx.build_index()?;

        let module_dir = self
            .dove_ctx
            .path_for(&self.dove_ctx.manifest.layout.modules_dir)
            .to_str()
            .map(|path| path.to_owned())
            .ok_or_else(|| anyhow!("Failed to convert module dir path"))?;

        let dep_set = index.make_dependency_set(&[module_dir.as_str(), script.name()])?;
        let mut dep_list = load_dependencies(dep_set)?;
        dep_list.extend(load_move_files(&[module_dir])?);

        let sender = self.dove_ctx.account_address()?;
        let Artifacts { files, prog } = MoveBuilder::new(
            self.dove_ctx.dialect.as_ref(),
            Some(sender),
            StaticResolver::new(dep_list),
        )
        .build(&[&script]);

        match prog {
            Err(errors) => {
                let mut writer = StandardStream::stderr(ColorChoice::Auto);
                output_errors(&mut writer, files, errors);
                Err(anyhow!(
                    "could not compile:{}",
                    self.dove_ctx.project_name()
                ))
            }
            Ok(compiled_units) => {
                let (compiled_units, ice_errors) = compiled_unit::verify_units(compiled_units);

                if !ice_errors.is_empty() {
                    let mut writer = StandardStream::stderr(ColorChoice::Auto);
                    output_errors(&mut writer, files, ice_errors);
                    Err(anyhow!("could not verify:{}", self.dove_ctx.project_name()))
                } else {
                    Ok(compiled_units)
                }
            }
        }
    }

    fn prepare_arguments(
        &self,
        args_type: &[(String, String)],
    ) -> Result<(usize, usize, Vec<ScriptArg>), Error> {
        let total_args = args_type.len();

        fn parse_err<D: Debug>(name: &str, tp: &str, index: usize, value: &str, err: D) -> Error {
            anyhow!(
                "Parameter '{}' has {} type. Failed to parse {} [{}]. Error:'{:?}'",
                name,
                tp,
                value,
                index,
                err
            )
        }

        args_type.iter().try_fold(
            (0, 0, Vec::new()),
            |(signers, args_index, mut values), (name, tp)| match tp.as_str() {
                "signer" => Ok((signers + 1, args_index, values)),
                "bool" => {
                    let arg = self.argument(args_index, total_args)?;
                    values.push(ScriptArg::Bool(
                        arg.parse()
                            .map_err(|err| parse_err(name, tp, args_index, arg, err))?,
                    ));
                    Ok((signers, args_index + 1, values))
                }
                "u8" => {
                    let arg = self.argument(args_index, total_args)?;
                    values.push(ScriptArg::U8(
                        arg.parse()
                            .map_err(|err| parse_err(name, tp, args_index, arg, err))?,
                    ));
                    Ok((signers, args_index + 1, values))
                }
                "u64" => {
                    let arg = self.argument(args_index, total_args)?;
                    values.push(ScriptArg::U64(
                        arg.parse()
                            .map_err(|err| parse_err(name, tp, args_index, arg, err))?,
                    ));
                    Ok((signers, args_index + 1, values))
                }
                "u128" => {
                    let arg = self.argument(args_index, total_args)?;
                    values.push(ScriptArg::U128(
                        arg.parse()
                            .map_err(|err| parse_err(name, tp, args_index, arg, err))?,
                    ));
                    Ok((signers, args_index + 1, values))
                }
                "address" => {
                    let arg = self.argument(args_index, total_args)?;
                    values.push(ScriptArg::Address(Address::from_str(arg)?.addr));
                    Ok((signers, args_index + 1, values))
                }
                "vector<u8>" => {
                    let arg = self.argument(args_index, total_args)?;
                    let buffer = if arg.contains('[') {
                        parse_vec(arg, "u8")?
                    } else {
                        hex::decode(arg)?
                    };
                    values.push(ScriptArg::VectorU8(buffer));
                    Ok((signers, args_index + 1, values))
                }
                "vector<u64>" => {
                    let arg = self.argument(args_index, total_args)?;
                    values.push(ScriptArg::VectorU64(parse_vec(arg, "u64")?));
                    Ok((signers, args_index + 1, values))
                }
                "vector<u128>" => {
                    let arg = self.argument(args_index, total_args)?;
                    values.push(ScriptArg::VectorU128(parse_vec(arg, "u128")?));
                    Ok((signers, args_index + 1, values))
                }
                "vector<address>" => {
                    let arg = self.argument(args_index, total_args)?;
                    let address = parse_vec::<Address>(arg, "vector<address>")?
                        .iter()
                        .map(|addr| addr.addr)
                        .collect();
                    values.push(ScriptArg::VectorAddress(address));
                    Ok((signers, args_index + 1, values))
                }
                &_ => Err(anyhow!("Unexpected script parameter: {}", tp)),
            },
        )
    }

    fn argument(&self, index: usize, total_expected: usize) -> Result<&String, Error> {
        self.args
            .get(index)
            .ok_or_else(|| anyhow!("{} arguments are expected.", total_expected))
    }

    pub fn build(self) -> Result<(String, Transaction), Error> {
        let (script, meta) = self.lookup_script()?;
        let units = self.build_script(script)?;

        let unit = units
            .into_iter()
            .find(|unit| {
                let is_module = match &unit {
                    CompiledUnit::Module { .. } => false,
                    CompiledUnit::Script { .. } => true,
                };
                is_module && unit.name() == meta.name
            })
            .map(|unit| unit.serialize())
            .map(|mut unit| {
                self.dove_ctx
                    .dialect
                    .adapt_to_target(&mut unit)
                    .map(|_| unit)
            })
            .ok_or_else(|| anyhow!("Script '{}' not found", meta.name))??;

        if meta.type_parameters.len() != self.type_parameters.len() {
            return Err(anyhow!(
                "Script '{}' takes {} type parameters, {} passed",
                meta.name,
                meta.type_parameters.len(),
                self.type_parameters.len()
            ));
        }

        let (signers, args_count, args) = self.prepare_arguments(&meta.parameters)?;

        if self.args.len() != args_count {
            return Err(anyhow!(
                "Script '{}' takes {} parameters, {} passed",
                meta.name,
                args_count,
                self.args.len()
            ));
        }

        Ok((
            meta.name,
            Transaction::new(signers as u8, unit, args, self.type_parameters)?,
        ))
    }
}

/// Script argument type.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum ScriptArg {
    /// u8
    U8(u8),
    /// u64
    U64(u64),
    /// u128
    U128(u128),
    /// bool
    Bool(bool),
    /// address
    Address(AccountAddress),
    /// vector<u8>
    VectorU8(Vec<u8>),
    /// vector<u64>
    VectorU64(Vec<u64>),
    /// vector<u128>
    VectorU128(Vec<u128>),
    /// vector<bool>
    VectorBool(Vec<bool>),
    /// vector<address>
    VectorAddress(Vec<AccountAddress>),
}

impl From<ScriptArg> for MoveValue {
    fn from(arg: ScriptArg) -> Self {
        match arg {
            ScriptArg::U8(val) => MoveValue::U8(val),
            ScriptArg::U64(val) => MoveValue::U64(val),
            ScriptArg::U128(val) => MoveValue::U128(val),
            ScriptArg::Bool(val) => MoveValue::Bool(val),
            ScriptArg::Address(val) => MoveValue::Address(val),
            ScriptArg::VectorU8(val) => MoveValue::vector_u8(val),
            ScriptArg::VectorU64(val) => {
                MoveValue::Vector(val.into_iter().map(MoveValue::U64).collect())
            }
            ScriptArg::VectorU128(val) => {
                MoveValue::Vector(val.into_iter().map(MoveValue::U128).collect())
            }
            ScriptArg::VectorBool(val) => {
                MoveValue::Vector(val.into_iter().map(MoveValue::Bool).collect())
            }
            ScriptArg::VectorAddress(val) => {
                MoveValue::Vector(val.into_iter().map(MoveValue::Address).collect())
            }
        }
    }
}

/// Transaction model.
#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    signers_count: u8,
    code: Vec<u8>,
    args: Vec<Vec<u8>>,
    type_args: Vec<TypeTag>,
}

impl Transaction {
    /// Create a new transaction.
    pub fn new(
        signers_count: u8,
        code: Vec<u8>,
        args: Vec<ScriptArg>,
        type_args: Vec<TypeTag>,
    ) -> Result<Transaction, Error> {
        let args = args
            .into_iter()
            .map(ScriptArg::into)
            .map(|val: MoveValue| bcs::to_bytes(&val))
            .collect::<Result<_, _>>()
            .map_err(Error::msg)?;

        Ok(Transaction {
            signers_count,
            code,
            args,
            type_args,
        })
    }
}

fn parse_type_params(lexer: &mut Lexer) -> Result<TypeTag, Error> {
    let ty = parse_type(lexer).map_err(|err| Error::msg(format!("{:?}", err)))?;
    unwrap_spanned_ty(ty)
}

fn parse_vec<E>(tkn: &str, tp_name: &str) -> Result<Vec<E>, Error>
where
    E: FromStr,
{
    let map_err = |err| Error::msg(format!("{:?}", err));

    let mut lexer = Lexer::new(tkn, "vec", Default::default());
    lexer.advance().map_err(map_err)?;

    if lexer.peek() != Tok::LBracket {
        return Err(anyhow!("Vector in format  [n1, n2, ..., nn] is expected."));
    }
    lexer.advance().map_err(map_err)?;

    let mut elements = vec![];
    while lexer.peek() != Tok::RBracket {
        match lexer.peek() {
            Tok::Comma => {
                lexer.advance().map_err(map_err)?;
                continue;
            }
            Tok::EOF => {
                return Err(anyhow!("unexpected end of vector."));
            }
            _ => {
                elements.push(E::from_str(lexer.content()).map_err(|_| {
                    anyhow!(
                        "Failed to parse vector element. {} type is expected. Actual:'{}'",
                        tp_name,
                        lexer.content()
                    )
                })?);
                lexer.advance().map_err(map_err)?;
            }
        }
    }
    Ok(elements)
}

fn store_transaction(ctx: &Context, name: &str, tx: Transaction) -> Result<(), Error> {
    let tx_dir = ctx.path_for(&ctx.manifest.layout.transactions_output);
    if !tx_dir.exists() {
        fs::create_dir_all(&tx_dir)?;
    }

    let mut tx_file = tx_dir.join(name);
    if !name.to_lowercase().ends_with(".mvt") {
        tx_file.set_extension("mvt");
    }

    if tx_file.exists() {
        fs::remove_file(&tx_file)?;
    }
    stdoutln!("Store transaction:{:?}", tx_file);
    Ok(fs::write(&tx_file, bcs::to_bytes(&tx)?)?)
}

struct Address {
    addr: AccountAddress,
}

impl FromStr for Address {
    type Err = Error;

    fn from_str(addr: &str) -> Result<Self, Self::Err> {
        let addr = match ss58_to_diem(addr) {
            Ok(addr) => AccountAddress::from_hex_literal(&addr)?,
            Err(_) => AccountAddress::from_hex_literal(&addr)?,
        };
        Ok(Address { addr })
    }
}

#[cfg(test)]
mod test {
    use crate::cmd::tx::TransactionBuilder;
    use move_core_types::language_storage::{TypeTag, StructTag};
    use move_core_types::language_storage::CORE_CODE_ADDRESS;
    use move_core_types::identifier::Identifier;

    #[test]
    fn test_parse_call() {
        let (name, tp, args) = TransactionBuilder::parse_call("create_account<u8, 0x01::Dfinance::USD<u8>>(10, 68656c6c6f, [10, 23], true, 1exaAg2VJRQbyUBAeXcktChCAqjVP9TUxF3zo23R2T6EGdE)").unwrap();
        assert_eq!(name, "create_account");
        assert_eq!(
            tp,
            vec![
                TypeTag::U8,
                TypeTag::Struct(StructTag {
                    address: CORE_CODE_ADDRESS,
                    module: Identifier::new("Dfinance").unwrap(),
                    name: Identifier::new("USD").unwrap(),
                    type_params: vec![TypeTag::U8],
                })
            ]
        );
        assert_eq!(
            args,
            vec![
                "10".to_owned(),
                "68656c6c6f".to_owned(),
                "[10,23]".to_owned(),
                "true".to_owned(),
                "0x1CF326C5AAA5AF9F0E2791E66310FE8F044FAADAF12567EAA0976959D1F7731F".to_owned()
            ]
        );

        let (name, tp, args) = TransactionBuilder::parse_call(
            "create_account<0x01::Dfinance::USD>([true, false], [0x01, 0x02])",
        )
        .unwrap();
        assert_eq!(name, "create_account");
        assert_eq!(
            tp,
            vec![TypeTag::Struct(StructTag {
                address: CORE_CODE_ADDRESS,
                module: Identifier::new("Dfinance").unwrap(),
                name: Identifier::new("USD").unwrap(),
                type_params: vec![],
            })]
        );
        assert_eq!(
            args,
            vec!["[true,false]".to_owned(), "[0x01,0x02]".to_owned()]
        );

        let (name, tp, args) = TransactionBuilder::parse_call("create_account()").unwrap();
        assert_eq!(name, "create_account");
        assert_eq!(tp, Vec::<TypeTag>::new());
        assert_eq!(args, Vec::<String>::new());

        let (name, tp, args) = TransactionBuilder::parse_call("create_account<>()").unwrap();
        assert_eq!(name, "create_account");
        assert_eq!(tp, Vec::<TypeTag>::new());
        assert_eq!(args, Vec::<String>::new());
    }
}
