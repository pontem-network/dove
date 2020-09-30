use anyhow::Context;
use anyhow::Result;
use move_core_types::gas_schedule::CostTable;
use move_core_types::parser::parse_transaction_argument;
use move_lang::parser::ast::Definition;
use move_lang::parser::syntax;
use move_lang::strip_comments_and_verify;
use vm::file_format::CompiledScript;
use vm::CompiledModule;

use utils::{FilesSourceText, MoveFile};

use crate::lang;
use crate::lang::executor::{
    convert_txn_arg, execute_script, generate_bytecode, prepare_fake_network_state,
    serialize_script,
};
use crate::lang::{check_defs, into_exec_compiler_error, replace_sender_placeholder};
use crate::shared::errors::{CompilerError, ExecCompilerError, FileSourceMap, ProjectSourceMap};
use crate::shared::results::{ChainStateChanges, ResourceChange};
use crate::shared::{line_endings, ProvidedAccountAddress};

pub trait Dialect {
    fn name(&self) -> &str;

    fn normalize_account_address(&self, addr: &str) -> Result<ProvidedAccountAddress>;

    fn cost_table(&self) -> CostTable;

    fn replace_addresses(&self, source_text: &str, source_map: &mut FileSourceMap) -> String;

    fn parse_file(
        &self,
        file: MoveFile,
        sender: &ProvidedAccountAddress,
    ) -> Result<(Vec<Definition>, FileSourceMap), ExecCompilerError> {
        let (fname, source_text) = file;

        let (mut source_text, mut file_source_map) = line_endings::normalize(source_text);
        source_text = replace_sender_placeholder(
            source_text,
            &sender.normalized_original,
            &mut file_source_map,
        );
        source_text = self.replace_addresses(&source_text, &mut file_source_map);

        let (source_text, comment_map) =
            strip_comments_and_verify(fname, &source_text).map_err(|errors| {
                into_exec_compiler_error(
                    errors,
                    ProjectSourceMap::with_file_map(fname, FileSourceMap::default()),
                )
            })?;

        let (defs, _) =
            syntax::parse_file_string(fname, &source_text, comment_map).map_err(|errors| {
                into_exec_compiler_error(
                    errors,
                    ProjectSourceMap::with_file_map(fname, file_source_map.clone()),
                )
            })?;
        Ok((defs, file_source_map))
    }

    fn parse_files(
        &self,
        current_file: MoveFile,
        deps: &[MoveFile],
        sender: &ProvidedAccountAddress,
    ) -> Result<(Vec<Definition>, Vec<Definition>, ProjectSourceMap), ExecCompilerError> {
        let mut exec_compiler_error = ExecCompilerError::default();

        let mut project_offsets_map = ProjectSourceMap::default();
        let script_defs = match self.parse_file(current_file.clone(), &sender) {
            Ok((defs, offsets_map)) => {
                project_offsets_map.0.insert(current_file.0, offsets_map);
                defs
            }
            Err(error) => {
                exec_compiler_error.extend(error);
                vec![]
            }
        };

        let mut dep_defs = vec![];
        for dep_file in deps.iter() {
            let defs = match self.parse_file(dep_file.clone(), &sender) {
                Ok((defs, offsets_map)) => {
                    project_offsets_map.0.insert(dep_file.0, offsets_map);
                    defs
                }
                Err(error) => {
                    exec_compiler_error.extend(error);
                    vec![]
                }
            };
            dep_defs.extend(defs);
        }
        if !exec_compiler_error.0.is_empty() {
            return Err(exec_compiler_error);
        }
        Ok((script_defs, dep_defs, project_offsets_map))
    }

    fn check_with_compiler(
        &self,
        current: MoveFile,
        deps: Vec<MoveFile>,
        sender: &ProvidedAccountAddress,
    ) -> Result<(), Vec<CompilerError>> {
        let (script_defs, dep_defs, offsets_map) = self
            .parse_files(current, &deps, sender)
            .map_err(|errors| errors.transform_with_source_map())?;

        match check_defs(script_defs, dep_defs, sender.as_address()) {
            Ok(_) => Ok(()),
            Err(errors) => {
                Err(into_exec_compiler_error(errors, offsets_map).transform_with_source_map())
            }
        }
    }

    fn check_and_generate_bytecode(
        &self,
        file: MoveFile,
        deps: &[MoveFile],
        sender: ProvidedAccountAddress,
    ) -> Result<(Option<CompiledScript>, Vec<CompiledModule>), ExecCompilerError> {
        let (mut script_defs, modules_defs, project_offsets_map) =
            self.parse_files(file, deps, &sender)?;
        script_defs.extend(modules_defs);

        let program = check_defs(script_defs, vec![], sender.as_address())
            .map_err(|errors| into_exec_compiler_error(errors, project_offsets_map.clone()))?;
        generate_bytecode(program)
            .map_err(|errors| into_exec_compiler_error(errors, project_offsets_map))
    }

    fn compile_and_run(
        &self,
        script: MoveFile,
        deps: &[MoveFile],
        provided_senders: Vec<ProvidedAccountAddress>,
        genesis_changes: Vec<ResourceChange>,
        args: Vec<String>,
    ) -> Result<ChainStateChanges> {
        let genesis_write_set = lang::resources::changes_into_writeset(genesis_changes)
            .with_context(|| "Provided genesis serialization error")?;

        let compilation_sender = provided_senders[0].clone();

        let (compiled_script, compiled_modules) =
            self.check_and_generate_bytecode(script, deps, compilation_sender)?;
        let compiled_script =
            compiled_script.expect("compile_and_run should always be called with the script");

        let network_state = prepare_fake_network_state(compiled_modules, genesis_write_set);

        let serialized_script =
            serialize_script(compiled_script).context("Script serialization error")?;

        let mut script_args = Vec::with_capacity(args.len());
        for passed_arg in args {
            let transaction_argument = parse_transaction_argument(&passed_arg)?;
            let script_arg = convert_txn_arg(transaction_argument);
            script_args.push(script_arg);
        }

        let senders = provided_senders
            .into_iter()
            .map(|s| s.as_account_address())
            .collect();
        execute_script(
            senders,
            &network_state,
            serialized_script,
            script_args,
            self.cost_table(),
        )
    }

    fn print_compiler_errors_and_exit(
        &self,
        files: FilesSourceText,
        errors: Vec<CompilerError>,
    ) -> ! {
        lang::report_errors(files, errors)
    }
}
