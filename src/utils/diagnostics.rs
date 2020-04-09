use std::collections::HashMap;

use codespan::{FileId, Files};
use lsp_types::{Diagnostic, DiagnosticRelatedInformation, Location, Range, Url};
use move_ir_types::location::Loc;
use move_lang::errors::{Error, FilesSourceText};

use crate::compiler::utils::get_canonical_fname;

fn loc_into_range(
    files: &Files<String>,
    fname_to_file_id: &HashMap<&'static str, FileId>,
    location: Loc,
) -> Range {
    let canonical_loc_fname = get_canonical_fname(location.file());
    let file_id = fname_to_file_id
        .get(canonical_loc_fname)
        .unwrap_or_else(|| {
            panic!(
                "Key {:?} not found in fname_to_file_id mapping. Keys are {:?}",
                canonical_loc_fname,
                fname_to_file_id.keys()
            )
        });
    codespan_lsp::byte_span_to_range(files, *file_id, location.span())
        .expect("Cannot convert codespan::Span from libra compiler into lsp::Range type")
}

pub fn libra_error_into_diagnostic(files: &FilesSourceText, error: Error) -> Diagnostic {
    let mut fname_to_file_id = HashMap::new();
    let mut files_db: Files<String> = Files::new();
    for (&fname, text) in files {
        let canonical_fname = get_canonical_fname(fname);
        let file_id = files_db.add(canonical_fname, text.to_owned());
        fname_to_file_id.insert(canonical_fname, file_id);
    }

    let (primary_loc, primary_message) = error.get(0).unwrap().to_owned();
    let mut diagnostic = {
        let range = loc_into_range(&files_db, &fname_to_file_id, primary_loc);
        Diagnostic::new_simple(range, primary_message)
    };
    // first error is an actual one, others are related info
    if error.len() > 1 {
        let mut related_info = vec![];
        for (related_loc, related_message) in error[1..].iter() {
            let range = loc_into_range(&files_db, &fname_to_file_id, *related_loc);
            let file_url = Url::from_file_path(related_loc.file()).unwrap();
            let related_info_item = DiagnosticRelatedInformation {
                location: Location::new(file_url, range),
                message: related_message.to_string(),
            };
            related_info.push(related_info_item);
        }
        diagnostic.related_information = Some(related_info)
    }
    diagnostic
}
