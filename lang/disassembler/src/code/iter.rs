use vm::file_format::Bytecode;
use std::slice::Iter;

/// Bytecode iterator.
pub struct BytecodeIterator<'a> {
    iter: Iter<'a, Bytecode>,
    index: Option<usize>,
    code: &'a [Bytecode],
}

impl<'a> BytecodeIterator<'a> {
    /// Create a new bytecode iterator.
    pub fn new(code: &'a [Bytecode]) -> BytecodeIterator<'a> {
        BytecodeIterator {
            iter: code.iter(),
            index: None,
            code,
        }
    }

    /// Returns current bytecode instruction index.
    pub fn index(&self) -> usize {
        self.index.unwrap_or(0)
    }

    /// Returns a reference to all bytecode instructions.
    #[allow(dead_code)]
    pub fn as_slice(&self) -> &[Bytecode] {
        self.code
    }

    /// Returns a reference to remaining bytecode instructions.
    pub fn remaining_code(&self) -> &[Bytecode] {
        self.iter.as_slice()
    }

    /// Returns a bytecode instruction by absolute offset.
    pub fn absolute(&self, index: usize) -> &Bytecode {
        &self.code.get(index).unwrap_or(&Bytecode::Nop)
    }

    /// Returns a bytecode instruction by relative offset.
    pub fn by_relative(&self, offset: isize) -> &Bytecode {
        &self
            .code
            .get(self.index() + offset as usize)
            .unwrap_or(&Bytecode::Nop)
    }
}

impl<'a> Iterator for BytecodeIterator<'a> {
    type Item = &'a Bytecode;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(index) = self.index {
            self.index = Some(index + 1);
        } else {
            self.index = Some(0);
        }

        self.iter.next()
    }
}
