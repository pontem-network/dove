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
use crate::shared::{line_endings, ProvidedAccountAddress};

use crate::lang::session::{init_execution_session};
use crate::lang;
use crate::lang::executor::{convert_txn_arg, execute_script, FakeRemoteCache, ExecutionResult};

pub trait Dialect {
    fn name(&self) -> &str;

    fn normalize_account_address(&self, addr: &str) -> Result<ProvidedAccountAddress>;

    fn cost_table(&self) -> CostTable;

    fn replace_addresses(&self, source_text: &str, source_map: &mut FileSourceMap) -> String;
}
