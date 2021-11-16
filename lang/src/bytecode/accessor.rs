use std::borrow::Cow;
use anyhow::Error;
use move_binary_format::{CompiledModule};
use move_binary_format::file_format::CompiledScript;

#[derive(Debug)]
pub enum BytecodeType {
    Script,
    Module,
}

#[derive(Debug)]
pub enum Bytecode {
    Script(String, CompiledScript, CompiledModule),
    Module(CompiledModule),
}

#[derive(Debug)]
pub struct BytecodeRef<'a> {
    package: Cow<'a, str>,
    tp: BytecodeType,
    name: Cow<'a, str>,
}

pub trait BytecodeAccess {
    fn list<'a>(
        &self,
        package: Option<&'a str>,
        name: Option<&'a str>,
        tp: Option<BytecodeType>,
    ) -> Result<Vec<BytecodeRef<'a>>, Error>;
    fn load(&self, rf: &BytecodeRef) -> Result<Option<Bytecode>, Error>;
}
