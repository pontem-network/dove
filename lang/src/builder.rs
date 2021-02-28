use crate::compiler::dialects::Dialect;
use crate::compiler::address::ProvidedAccountAddress;
use crate::compiler::file::MoveFile;
use crate::compiler::{CompileFlow, compile};
use crate::compiler::parser::ParsingMeta;
use diem::move_lang::compiled_unit::CompiledUnit;
use diem::move_lang::errors::{Errors, FilesSourceText};

pub struct Artifacts {
    pub files: FilesSourceText,
    pub prog: Result<Vec<CompiledUnit>, Errors>,
}

pub struct MoveBuilder<'a> {
    dialect: &'a dyn Dialect,
    sender: Option<&'a ProvidedAccountAddress>,
}

impl<'a> MoveBuilder<'a> {
    pub fn new(
        dialect: &'a dyn Dialect,
        sender: Option<&'a ProvidedAccountAddress>,
    ) -> MoveBuilder<'a> {
        MoveBuilder { dialect, sender }
    }

    pub fn build(self, targets: &[MoveFile], deps: &[MoveFile]) -> Artifacts {
        compile(self.dialect, targets, deps, self.sender, self)
    }
}

impl<'a> CompileFlow<Artifacts> for MoveBuilder<'a> {
    fn after_translate(
        &mut self,
        meta: ParsingMeta,
        result: Result<Vec<CompiledUnit>, Errors>,
    ) -> Artifacts {
        let prog = result.map_err(|errors| meta.offsets_map.transform(errors));
        Artifacts {
            files: meta.source_map,
            prog,
        }
    }
}
