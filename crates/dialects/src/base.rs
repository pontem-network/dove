use anyhow::Context;
use anyhow::Result;
use move_core_types::gas_schedule::CostTable;
use move_core_types::parser::parse_transaction_argument;
use move_lang::parser::ast::Definition;
use move_lang::parser::syntax;
use move_lang::{strip_comments_and_verify, FileCommentMap};

use utils::{FilesSourceText, MoveFile};

use crate::lang::{
    check_defs, into_exec_compiler_error, replace_sender_placeholder, ProgramCommentsMap,
    PreBytecodeProgram,
};
use crate::shared::errors::{CompilerError, ExecCompilerError, FileSourceMap, ProjectSourceMap};
use crate::shared::results::{ChainStateChanges, ResourceChange};
use crate::shared::{line_endings, ProvidedAccountAddress};

use crate::lang::session::{init_execution_session};
use crate::lang;
use crate::lang::executor::{prepare_fake_network_state, convert_txn_arg, execute_script};

pub trait Dialect {
    fn name(&self) -> &str;

    fn normalize_account_address(&self, addr: &str) -> Result<ProvidedAccountAddress>;

    fn cost_table(&self) -> CostTable;

    fn replace_addresses(&self, source_text: &str, source_map: &mut FileSourceMap) -> String;

    fn normalize_source_text(
        &self,
        file: MoveFile,
        sender: &ProvidedAccountAddress,
    ) -> (MoveFile, FileSourceMap) {
        let (fname, source_text) = file;
        let (mut source_text, mut file_source_map) = line_endings::normalize(source_text);
        source_text = replace_sender_placeholder(
            source_text,
            &sender.normalized_original,
            &mut file_source_map,
        );
        source_text = self.replace_addresses(&source_text, &mut file_source_map);
        ((fname, source_text), file_source_map)
    }

    fn parse_file(
        &self,
        file: MoveFile,
        sender: &ProvidedAccountAddress,
    ) -> Result<(Vec<Definition>, String, FileSourceMap, FileCommentMap), ExecCompilerError> {
        // let (fname, source_text) = file;
        let ((fname, source_text), file_source_map) = self.normalize_source_text(file, sender);

        let (stripped_source_text, comment_map) = strip_comments_and_verify(fname, &source_text)
            .map_err(|errors| {
                into_exec_compiler_error(
                    errors,
                    ProjectSourceMap::with_file_map(fname, FileSourceMap::default()),
                )
            })?;
        let (defs, _) =
            syntax::parse_file_string(fname, &stripped_source_text, FileCommentMap::default())
                .map_err(|errors| {
                    into_exec_compiler_error(
                        errors,
                        ProjectSourceMap::with_file_map(fname, file_source_map.clone()),
                    )
                })?;
        Ok((defs, source_text, file_source_map, comment_map))
    }

    fn parse_files(
        &self,
        current_file: MoveFile,
        deps: &[MoveFile],
        sender: &ProvidedAccountAddress,
    ) -> Result<
        (
            Vec<Definition>,
            Vec<Definition>,
            ProjectSourceMap,
            ProgramCommentsMap,
        ),
        ExecCompilerError,
    > {
        let mut exec_compiler_error = ExecCompilerError::default();

        let mut project_offsets_map = ProjectSourceMap::default();
        let mut comment_map = ProgramCommentsMap::new();

        let script_defs = match self.parse_file(current_file.clone(), &sender) {
            Ok((defs, normalized_source_text, offsets_map, comments)) => {
                project_offsets_map.0.insert(current_file.0, offsets_map);
                comment_map.insert(current_file.0, (normalized_source_text, comments));
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
                Ok((defs, normalized_source_text, offsets_map, file_comment_map)) => {
                    project_offsets_map.0.insert(dep_file.0, offsets_map);
                    comment_map.insert(dep_file.0, (normalized_source_text, file_comment_map));
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
        Ok((script_defs, dep_defs, project_offsets_map, comment_map))
    }

    fn check_with_compiler(
        &self,
        current: MoveFile,
        deps: Vec<MoveFile>,
        sender: &ProvidedAccountAddress,
    ) -> Result<(), Vec<CompilerError>> {
        let (script_defs, dep_defs, offsets_map, _) = self
            .parse_files(current, &deps, sender)
            .map_err(|errors| errors.transform_with_source_map())?;

        match check_defs(script_defs, dep_defs, sender.as_address()) {
            Ok(_) => Ok(()),
            Err(errors) => {
                Err(into_exec_compiler_error(errors, offsets_map).transform_with_source_map())
            }
        }
    }

    fn compile_to_prebytecode_program(
        &self,
        script: MoveFile,
        deps: &[MoveFile],
        sender: ProvidedAccountAddress,
    ) -> Result<(PreBytecodeProgram, ProgramCommentsMap, ProjectSourceMap), ExecCompilerError>
    {
        let (mut file_defs, dep_defs, project_offsets_map, comments) =
            self.parse_files(script, deps, &sender)?;
        file_defs.extend(dep_defs);

        let program = check_defs(file_defs, vec![], sender.as_address())
            .map_err(|errors| into_exec_compiler_error(errors, project_offsets_map.clone()))?;
        Ok((program, comments, project_offsets_map))
    }

    fn compile_and_run(
        &self,
        script: MoveFile,
        deps: &[MoveFile],
        provided_sender: ProvidedAccountAddress,
        genesis_changes: Vec<ResourceChange>,
        args: Vec<String>,
    ) -> Result<ChainStateChanges> {
        let genesis_write_set = lang::resources::changes_into_writeset(genesis_changes)
            .with_context(|| "Provided genesis serialization error")?;

        let (program, comments, project_source_map) =
            self.compile_to_prebytecode_program(script, deps, provided_sender.clone())?;
        let execution_session = init_execution_session(program, comments, provided_sender)
            .map_err(|errors| into_exec_compiler_error(errors, project_source_map))?;

        let modules = execution_session.modules();
        let network_state = prepare_fake_network_state(modules, genesis_write_set);

        let mut script_args = Vec::with_capacity(args.len());
        for passed_arg in args {
            let transaction_argument = parse_transaction_argument(&passed_arg)?;
            let script_arg = convert_txn_arg(transaction_argument);
            script_args.push(script_arg);
        }

        let (serialized_script, meta) = execution_session.into_script()?;
        execute_script(
            meta,
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
