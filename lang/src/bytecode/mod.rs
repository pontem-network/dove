pub mod accessor;
pub mod info;

use std::vec::IntoIter;
use anyhow::Error;
use crate::bytecode::accessor::{BytecodeAccess, BytecodeRef, BytecodeType};
use crate::bytecode::info::BytecodeInfo;

#[derive(Debug)]
pub struct SearchParams<'a> {
    pub tp: Option<BytecodeType>,
    pub package: Option<&'a str>,
    pub name: Option<&'a str>,
}

pub fn find<A>(
    accessor: A,
    params: SearchParams,
) -> Result<BytecodeIter<IntoIter<BytecodeRef>, A>, Error>
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

pub struct BytecodeIter<I: Iterator<Item = BytecodeRef>, A: BytecodeAccess> {
    refs: I,
    accessor: A,
}

impl<I: Iterator<Item = BytecodeRef>, A: BytecodeAccess> Iterator for BytecodeIter<I, A> {
    type Item = Result<BytecodeInfo, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let rf = self.refs.next()?;
        match self.accessor.load(rf) {
            Ok(Some(bytecode)) => Some(Ok(bytecode.into())),
            Err(err) => Some(Err(err)),
            Ok(None) => Some(Err(anyhow!("Bytecode not found."))),
        }
    }
}
