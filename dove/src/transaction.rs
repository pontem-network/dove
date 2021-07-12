use std::str::FromStr;
use std::path::PathBuf;
use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use anyhow::Error;
use termcolor::{StandardStream, ColorChoice};
use move_lang::parser::lexer::{Lexer, Tok};
use move_lang::parser::syntax::parse_type;
use move_lang::compiled_unit;
use move_lang::errors::output_errors;
use move_lang::compiled_unit::CompiledUnit;
use move_core_types::language_storage::TypeTag;
use move_core_types::account_address::AccountAddress;
use move_core_types::value::MoveValue;
use move_executor::explain::PipelineExecutionResult;
use move_executor::executor::Executor;
use lang::lexer::unwrap_spanned_ty;
use lang::compiler::file::{MoveFile, find_move_files, load_move_files};
use lang::compiler::address::ss58::{replace_ss58_addresses, ss58_to_diem};
use lang::compiler::mut_string::MutString;
use lang::compiler::dialects::Dialect;
use lang::flow::builder::{Artifacts, MoveBuilder, StaticResolver};
use crate::context::Context;
use crate::cmd::load_dependencies;
use lang::compiler::metadata::Meta;

/// Creating a transaction to run or save
pub struct TransactionBuilder<'a> {
    /// file name in folder "project_name/scripts/*.move"
    pub script_file_name: Option<String>,
    /// script name/function name - to run
    pub script_name: Option<String>,
    /// List of Types to pass to the function
    pub type_parameters: Vec<TypeTag>,
    /// List of Arguments to pass to the function
    pub args: Vec<String>,
    /// List of "signers" to pass to the function
    pub signers: Vec<AccountAddress>,
    /// Launch data: dialect, manifest, project directory
    pub dove_ctx: &'a Context,
}
impl<'a> TransactionBuilder<'a> {
    /// create an empty TransactionBuilder
    pub fn new(ctx: &Context) -> TransactionBuilder {
        TransactionBuilder {
            script_file_name: None,
            script_name: None,
            type_parameters: Vec::new(),
            args: Vec::new(),
            signers: Vec::new(),
            dove_ctx: ctx,
        }
    }
    // =============================================================================================
    /// Initialize parameters by cmd call
    /// None => The value of None is ignored. Not assigned to a parameter
    pub fn with_cmd_call(&mut self, call: Option<String>) -> Result<&mut Self, Error> {
        if let Some(call) = call {
            let (script_name, type_parameters, args) = parse_call(&call)?;
            self.script_name = Some(script_name);
            self.type_parameters = type_parameters;
            self.args = args;
        }
        Ok(self)
    }
    /// Set the "file name" based on the transmitted data from cmd
    /// None => The value of None is ignored. Not assigned to a parameter
    pub fn with_cmd_script_file_name(&mut self, script_file_name: Option<String>) -> &mut Self {
        if script_file_name.is_some() {
            self.script_file_name = script_file_name;
        }
        self
    }
    /// Set the "script name" based on the transmitted data from cmd
    /// None => The value of None is ignored. Not assigned to a parameter
    pub fn with_cmd_script_name(&mut self, script_name: Option<String>) -> &mut Self {
        if script_name.is_some() {
            self.script_name = script_name;
        }
        self
    }
    /// Set the "script name" based on the transmitted data from cmd
    /// None => The value of None is ignored. Not assigned to a parameter
    pub fn with_cmd_type_parameters(
        &mut self,
        type_parameters: Option<Vec<String>>,
    ) -> Result<&mut Self, Error> {
        if let Some(type_parameters) = type_parameters {
            self.type_parameters = type_parameters
                .iter()
                .map(|tp| {
                    let mut mut_string = MutString::new(tp);
                    replace_ss58_addresses(tp, &mut mut_string, &mut Default::default());
                    mut_string.freeze()
                })
                .map(|tp| {
                    let mut lexer = Lexer::new(&tp, "tp", Default::default());
                    lexer
                        .advance()
                        .map_err(|err| Error::msg(format!("{:?}", err)))?;
                    parse_type_params(&mut lexer)
                })
                .collect::<Result<_, _>>()?;
        }
        Ok(self)
    }
    /// Set the "args" based on the transmitted data from cmd
    /// None => The value of None is ignored. Not assigned to a parameter
    pub fn with_cmd_args(&mut self, args: Option<Vec<String>>) -> &mut Self {
        if let Some(args) = args {
            self.args = args
                .iter()
                .map(|arg| {
                    let mut mut_string = MutString::new(arg);
                    replace_ss58_addresses(arg, &mut mut_string, &mut Default::default());
                    mut_string.freeze()
                })
                .collect();
        }
        self
    }
    /// Set the "signers" based on the transmitted data from cmd
    /// None => The value of None is ignored. Not assigned to a parameter
    pub fn with_cmd_signers(&mut self, signers: Option<Vec<String>>) -> Result<&mut Self, Error> {
        let mut signers = signers
            .unwrap_or_default()
            .iter()
            .map(|signer| self.dove_ctx.dialect.parse_address(signer))
            .collect::<Result<Vec<_>, Error>>()?;
        if signers.is_empty() {
            signers.push(self.dove_ctx.account_address()?);
        }
        self.signers = signers;
        Ok(self)
    }

    // =============================================================================================
    /// Create transaction
    /// Return: Ok(name_transaction, Transaction)
    pub fn to_transaction(&self) -> Result<(String, Transaction), Error> {
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

        let (signers_count, args) = self.prepare_arguments(&meta.parameters)?;

        Ok((
            meta.name,
            Transaction::new(
                signers_count as u8,
                unit,
                args,
                self.type_parameters.clone(),
            )?,
        ))
    }
    /// Find and run the script using the specified parameters
    pub fn run(&self) -> Result<PipelineExecutionResult, Error> {
        let (script, _meta) = self.lookup_script()?;
        let dep_list = self.get_dep_list(script.name().as_ref())?;
        let executor = Executor::new(self.dove_ctx.dialect.as_ref(), self.signers[0], dep_list);
        executor.execute_script(script, Some(self.signers.clone()), self.args.clone())
    }
    // =============================================================================================
    fn lookup_script_by_file_name(&self, fname: &str) -> Result<(MoveFile, Meta), Error> {
        let file_path = self
            .dove_ctx
            .path_for(&self.dove_ctx.manifest.layout.scripts_dir)
            .join(fname)
            .with_extension("move");
        ensure!(file_path.exists(), "File [{}] not found", fname);

        let sender = self.dove_ctx.account_address()?;
        let script = MoveFile::load(&file_path)?;
        let mut scripts =
            ScriptMetadata::extract(self.dove_ctx.dialect.as_ref(), Some(sender), &[&script])?;
        ensure!(!scripts.is_empty(), "Script not found in file '{}'", fname);

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
                anyhow::bail!(
                    "Failed to determine script. There are several scripts in file [{}]. Use '--name' to determine the script.",
                    file_path.display()
                );
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
        ensure!(!files.is_empty(), "Script not found.");

        if files.len() > 1 {
            let name_list = files
                .iter()
                .map(|(mf, _)| mf.name())
                .collect::<Vec<_>>()
                .join(", ");
            anyhow::bail!(
                "There are several scripts with the name '{}' in files ['{}'].",
                name,
                name_list
            );
        }

        let (file, mut meta) = files.remove(0);
        ensure!(!meta.is_empty(), "Script not found.");
        if meta.len() > 1 {
            let file = split_movefile_into_scripts(file.to_owned(), Some(name))?.remove(0);
            let meta_index = meta
                .iter()
                .enumerate()
                .find_map(|(index, meta)| if meta.name == name { Some(index) } else { None })
                .map_or_else(|| Err(anyhow!("meta not found")), Ok)?;
            Ok((file, meta.remove(meta_index)))
        } else {
            Ok((file, meta.remove(0)))
        }
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
            ensure!(!meta.is_empty(), "Script not found.");
            ensure!( meta.len() < 2, "Failed to determine script. There are several scripts. Use '--name' to determine the script.");

            Ok((mf, meta.remove(0)))
        } else {
            anyhow::bail!("Failed to determine script. There are several scripts. Use '--name' or '--file' to determine the script.")
        }
    }
    // =============================================================================================
    fn prepare_arguments(
        &self,
        arguments: &[(String, String)],
    ) -> Result<(usize, Vec<ScriptArg>), Error> {
        fn parse_err<D: Debug>(name: &str, tp: &str, value: &str, err: D) -> Error {
            anyhow!(
                "Parameter '{}' has type {}. Failed to parse {}. Error:'{:?}'",
                name,
                tp,
                value,
                err
            )
        }

        assert_eq!(self.args.len(), arguments.len());
        let mut signers_count = 0;
        let mut values = Vec::with_capacity(arguments.len());
        for ((arg_name, arg_type), arg_value) in arguments.iter().zip(&self.args) {
            macro_rules! parse_primitive {
                ($script_arg:expr) => {
                    values.push($script_arg(
                        arg_value
                            .parse()
                            .map_err(|err| parse_err(arg_name, arg_type, arg_value, err))?,
                    ))
                };
            }
            match arg_type.as_str() {
                "signer" => {
                    signers_count += 1;
                }
                "bool" => parse_primitive!(ScriptArg::Bool),
                "u8" => parse_primitive!(ScriptArg::U8),
                "u64" => parse_primitive!(ScriptArg::U64),
                "u128" => parse_primitive!(ScriptArg::U128),
                "address" => parse_primitive!(ScriptArg::Address),
                "vector<u8>" => {
                    let vec = if arg_value.contains('[') {
                        parse_vec(arg_value, "u8")
                            .map_err(|err| parse_err(arg_name, arg_type, arg_value, err))?
                    } else {
                        hex::decode(arg_value)
                            .map_err(|err| parse_err(arg_name, arg_type, arg_value, err))?
                    };
                    values.push(ScriptArg::VectorU8(vec));
                }
                "vector<u64>" => values
                    .push(ScriptArg::VectorU64(parse_vec(arg_value, "u64").map_err(
                        |err| parse_err(arg_name, arg_type, arg_value, err),
                    )?)),
                "vector<u128>" => values
                    .push(ScriptArg::VectorU128(parse_vec(arg_value, "u64").map_err(
                        |err| parse_err(arg_name, arg_type, arg_value, err),
                    )?)),
                "vector<address>" => {
                    let addresses = parse_vec::<Address>(arg_value, "vector<address>")
                        .map_err(|err| parse_err(arg_name, arg_type, arg_value, err))?
                        .iter()
                        .map(|addr| addr.addr)
                        .collect();
                    values.push(ScriptArg::VectorAddress(addresses));
                }
                other => anyhow::bail!("Unexpected script parameter: {}", other),
            }
        }
        Ok((signers_count, values))
    }

    fn build_script(&'a self, script: MoveFile<'a, 'a>) -> Result<Vec<CompiledUnit>, Error> {
        let dep_list = self.get_dep_list(script.name())?.clone();

        let sender = self.dove_ctx.account_address()?;
        let Artifacts { files, prog, .. } = MoveBuilder::new(
            self.dove_ctx.dialect.as_ref(),
            Some(sender),
            StaticResolver::new(dep_list),
        )
        .build(&[&script], false);

        match prog {
            Err(errors) => {
                let mut writer = StandardStream::stderr(ColorChoice::Auto);
                output_errors(&mut writer, files, errors);
                anyhow::bail!("could not compile:{}", self.dove_ctx.project_name())
            }
            Ok(compiled_units) => {
                let (compiled_units, ice_errors) = compiled_unit::verify_units(compiled_units);

                if !ice_errors.is_empty() {
                    let mut writer = StandardStream::stderr(ColorChoice::Auto);
                    output_errors(&mut writer, files, ice_errors);
                    anyhow::bail!("could not verify:{}", self.dove_ctx.project_name())
                } else {
                    Ok(compiled_units)
                }
            }
        }
    }

    fn get_dep_list<'b>(&self, srcipt_path: &str) -> Result<Vec<MoveFile<'b, 'b>>, Error> {
        let mut index = self.dove_ctx.build_index()?;

        let module_dir = self
            .dove_ctx
            .path_for(&self.dove_ctx.manifest.layout.modules_dir)
            .to_str()
            .map(|path| path.to_owned())
            .ok_or_else(|| anyhow!("Failed to convert module dir path"))?;

        let dep_set = index.make_dependency_set(&[module_dir.as_str(), srcipt_path])?;
        let mut dep_list = load_dependencies(dep_set)?;
        dep_list.extend(load_move_files(&[module_dir])?);
        Ok(dep_list.clone())
    }
}

fn split_movefile_into_scripts<'a>(
    base_movefile: MoveFile,
    need_script_name: Option<&str>,
) -> Result<Vec<MoveFile<'a, 'a>>, Error> {
    let mut find_scripts = parse_text_into_scripts(base_movefile.content())?;
    if let Some(script_name) = need_script_name {
        find_scripts = find_scripts
            .iter()
            .filter(|(name, _)| script_name == name)
            .cloned()
            .collect();
    }
    if find_scripts.is_empty() {
        return Err(anyhow!("Script(s) not found"));
    }
    let movefiles: Vec<MoveFile> = find_scripts
        .iter()
        .map(|(_, body)| MoveFile::with_content(base_movefile.name().to_owned(), body.clone()))
        .collect();

    Ok(movefiles)
}
fn parse_text_into_scripts(content: &str) -> Result<Vec<(String, String)>, Error> {
    let mut lexer = Lexer::new(&content, "source", Default::default());
    lexer.advance().unwrap();

    let mut scripts_positions: Vec<(String, usize, usize)> = Vec::new();

    while lexer.peek() != Tok::EOF {
        while lexer.peek() != Tok::Script {
            lexer.advance().unwrap();
        }
        let start = lexer.start_loc();
        let mut function_name = String::new();

        loop {
            lexer.advance().unwrap();
            match lexer.peek() {
                Tok::Fun => {
                    lexer.advance().unwrap();
                    if lexer.peek() != Tok::IdentifierValue {
                        return Err(anyhow!("Script is not valid "));
                    }
                    function_name = lexer.content().to_string();
                }
                Tok::Script => break,
                Tok::EOF => break,
                _ => (),
            };
        }
        scripts_positions.push((function_name, start, lexer.previous_end_loc()));
    }
    Ok(scripts_positions
        .iter()
        .map(|(name, start, end)| (name.clone(), content[*start..*end].to_string()))
        .collect::<Vec<(String, String)>>())
}
/// Parse call
/// Return: Ok(Script name, Type parameters, Function arguments) or Error
/// ```
/// use move_core_types::language_storage::TypeTag;
/// use dove::transaction::parse_call;
///
///    assert_eq!(
///         parse_call("script_name_1()").unwrap(),
///         ("script_name_1".to_string(), vec![], vec![])
///    );
///    assert_eq!(
///         parse_call("script_name_2(1,2)").unwrap(),
///         ("script_name_2".to_string(), vec![], vec!["1".to_string(), "2".to_string()])
///    );
///    assert_eq!(
///         parse_call("script_name_3<u8,u64>(1,2)").unwrap(),
///         ("script_name_3".to_string(), vec![TypeTag::U8,TypeTag::U64], vec!["1".to_string(), "2".to_string()])
///    );
/// ```
pub fn parse_call(call: &str) -> Result<(String, Vec<TypeTag>, Vec<String>), Error> {
    let mut mut_string = MutString::new(call);
    replace_ss58_addresses(call, &mut mut_string, &mut Default::default());
    let call = mut_string.freeze();

    let map_err = |err| Error::msg(format!("{:?}", err));
    let mut lexer = Lexer::new(&call, "call", Default::default());
    lexer.advance().map_err(map_err)?;
    if lexer.peek() != Tok::IdentifierValue {
        anyhow::bail!("Invalid call script format.\
             Expected function identifier. Use pattern \
             'script_name<comma separated type parameters>(comma separated parameters WITHOUT signers)'");
    }

    let script_name = lexer.content().to_owned();

    lexer.advance().map_err(map_err)?;

    let type_parameters = if lexer.peek() == Tok::Less {
        let mut type_parameter = vec![];

        lexer.advance().map_err(map_err)?;
        while lexer.peek() != Tok::Greater {
            if lexer.peek() == Tok::EOF {
                anyhow::bail!("Invalid call script format.\
                     Invalid type parameters format.. Use pattern \
                     'script_name<comma separated type parameters>(comma separated parameters WITHOUT signers)'");
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
        anyhow::bail!("Invalid call script format.\
             Invalid script arguments format.. Left paren '(' is expected. Use pattern \
             'script_name<comma separated type parameters>(comma separated parameters WITHOUT signers)'");
    }

    let mut arguments = vec![];

    lexer.advance().map_err(map_err)?;
    while lexer.peek() != Tok::RParen {
        if lexer.peek() == Tok::EOF {
            anyhow::bail!("Invalid call script format.\
                 Invalid arguments format.. Use pattern \
                 'script_name<comma separated type parameters>(comma separated parameters WITHOUT signers)'");
        }

        if lexer.peek() == Tok::Comma {
            lexer.advance().map_err(map_err)?;
            continue;
        }

        let mut token = String::new();
        token.push_str(lexer.content());
        let sw = lexer.peek() == Tok::LBracket;
        lexer.advance().map_err(map_err)?;
        if sw {
            while lexer.peek() != Tok::RBracket {
                token.push_str(lexer.content());
                lexer.advance().map_err(map_err)?;
            }
            token.push_str(lexer.content());
        } else {
            while lexer.peek() != Tok::Comma && lexer.peek() != Tok::RParen {
                token.push_str(lexer.content());
                lexer.advance().map_err(map_err)?;
            }
        }
        arguments.push(token);
        if !sw && lexer.peek() == Tok::RParen {
            break;
        }
        lexer.advance().map_err(map_err)?;
    }

    Ok((script_name, type_parameters, arguments))
}

/// parse type params
///
/// u8 => TypeTag::U8
/// u64 => TypeTag::U64
/// ...
pub fn parse_type_params(lexer: &mut Lexer) -> Result<TypeTag, Error> {
    let ty = parse_type(lexer).map_err(|err| Error::msg(format!("{:?}", err)))?;
    unwrap_spanned_ty(ty)
}
/// search for a script in project/scripts/*.move by name
pub fn lookup_script_by_name<'a>(
    name: &str,
    script_path: PathBuf,
    sender: AccountAddress,
    dialect: &'a dyn Dialect,
) -> Result<(MoveFile<'a, 'a>, Meta), Error> {
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
        .map(|mf| ScriptMetadata::extract(dialect, Some(sender), &[&mf]).map(|meta| (mf, meta)))
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
        anyhow::bail!("Script not found.");
    }

    if files.len() > 1 {
        let name_list = files
            .iter()
            .map(|(mf, _)| mf.name())
            .collect::<Vec<_>>()
            .join(", ");
        anyhow::bail!(
            "There are several scripts with the name '{:?}' in files ['{}'].",
            name,
            name_list
        );
    }

    let (file, mut meta) = files.remove(0);
    if meta.is_empty() {
        anyhow::bail!("Script not found.");
    }

    if meta.len() > 1 {
        anyhow::bail!(
            "There are several scripts with the name '{:?}' in file '{}'.",
            name,
            file.name()
        );
    }
    Ok((file, meta.remove(0)))
}

/// Script argument type.
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

fn parse_vec<E>(tkn: &str, tp_name: &str) -> Result<Vec<E>, Error>
where
    E: FromStr,
{
    let map_err = |err| Error::msg(format!("{:?}", err));

    let mut lexer = Lexer::new(tkn, "vec", Default::default());
    lexer.advance().map_err(map_err)?;

    if lexer.peek() != Tok::LBracket {
        anyhow::bail!("Vector in format  [n1, n2, ..., nn] is expected.");
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
                anyhow::bail!("unexpected end of vector.");
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
    use move_core_types::language_storage::{TypeTag, StructTag};
    use move_core_types::language_storage::CORE_CODE_ADDRESS;
    use move_core_types::identifier::Identifier;
    use crate::transaction::parse_call;

    #[test]
    fn test_parse_call() {
        let (name, tp, args) = parse_call("create_account<u8, 0x01::Dfinance::USD<u8>>(10, 68656c6c6f, [10, 23], true, 1exaAg2VJRQbyUBAeXcktChCAqjVP9TUxF3zo23R2T6EGdE)").unwrap();
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

        let (name, tp, args) =
            parse_call("create_account<0x01::Dfinance::USD>([true, false], [0x01, 0x02])")
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

        let (name, tp, args) = parse_call("create_account()").unwrap();
        assert_eq!(name, "create_account");
        assert_eq!(tp, Vec::<TypeTag>::new());
        assert_eq!(args, Vec::<String>::new());

        let (name, tp, args) = parse_call("create_account<>()").unwrap();
        assert_eq!(name, "create_account");
        assert_eq!(tp, Vec::<TypeTag>::new());
        assert_eq!(args, Vec::<String>::new());
    }
}
