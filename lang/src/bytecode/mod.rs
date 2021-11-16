pub mod accessor;
pub mod info;

use std::vec::IntoIter;
use anyhow::Error;
use move_binary_format::access::{ModuleAccess, ScriptAccess};
use move_binary_format::CompiledModule;
use move_binary_format::file_format::{Ability, AbilitySet, empty_module, SignatureToken, StructHandleIndex, Visibility};
use move_core_types::account_address::AccountAddress;
use crate::bytecode::accessor::{Bytecode, BytecodeAccess, BytecodeRef, BytecodeType};
use crate::bytecode::info::BytecodeInfo;

pub struct SearchParams<'a> {
    tp: Option<BytecodeType>,
    package: Option<&'a str>,
    name: Option<&'a str>,
}

pub fn find<'a, A>(
    accessor: A,
    params: SearchParams<'a>,
) -> Result<BytecodeIter<IntoIter<BytecodeRef<'a>>, A>, Error>
    where
        A: BytecodeAccess,
{
    Ok(BytecodeIter {
        refs: accessor
            .list(params.package, params.name, params.tp)?
            .into_iter(),
        accessor,
    })
}

pub struct BytecodeIter<'a, I: Iterator<Item=BytecodeRef<'a>>, A: BytecodeAccess> {
    refs: I,
    accessor: A,
}

impl<'a, I: Iterator<Item=BytecodeRef<'a>>, A: BytecodeAccess> Iterator
for BytecodeIter<'a, I, A>
{
    type Item = Result<BytecodeInfo, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let rf = self.refs.next()?;
        match self.accessor.load(&rf) {
            Ok(Some(bytecode)) => Some(Ok(bytecode.into())),
            Err(err) => Some(Err(err)),
            Ok(None) => Some(Err(anyhow!("Bytecode not found. Ref:{:?}", rf))),
        }
    }
}
