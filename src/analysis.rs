use lsp_types::Diagnostic;
use move_lang::errors::FilesSourceText;
use move_lang::shared::Address;

use crate::compiler::check_with_compiler;
use crate::compiler::utils::get_canonical_fname;

#[derive(Default, Debug)]
pub struct AnalysisChange {
    address_changed: Option<Address>,
    files_changed: Vec<(&'static str, String)>,
}

impl AnalysisChange {
    pub fn new() -> Self {
        AnalysisChange::default()
    }

    pub fn change_file(&mut self, fname: &'static str, new_text: String) {
        self.files_changed
            .push((&get_canonical_fname(fname), new_text))
    }

    pub fn change_sender_address(&mut self, new_address: Address) {
        self.address_changed = Some(new_address);
    }
}

#[derive(Debug, Default, Clone)]
pub struct Analysis {
    sender_address: Address,
    available_module_files: FilesSourceText,
}

impl Analysis {
    pub fn available_module_files(&self) -> &FilesSourceText {
        &self.available_module_files
    }

    pub fn sender_address(&self) -> Address {
        self.sender_address
    }

    pub fn apply_change(&mut self, change: AnalysisChange) {
        if let Some(address) = change.address_changed {
            self.sender_address = address;
        }
        for (fname, new_text) in change.files_changed {
            self.available_module_files.insert(fname, new_text);
        }
    }

    pub fn check_with_libra_compiler(
        &self,
        canonical_fname: &'static str,
        text: &str,
    ) -> Vec<Diagnostic> {
        match check_with_compiler(canonical_fname, text, self) {
            Err(diagnostics) => diagnostics,
            Ok(_) => vec![],
        }
    }
}
