use std::collections::HashMap;
use crate::compiler::errors::{CompilerError, CompilerErrorPart};
use utils::MoveFilePath;

pub fn len_difference(orig: &str, replacement: &str) -> isize {
    orig.len() as isize - replacement.len() as isize
}

#[derive(Debug, Clone)]
pub struct Layer {
    pub pos: usize,
    pub offset: isize,
}

impl Layer {
    pub fn translate(&self, pos: usize) -> usize {
        if pos < self.pos {
            pos
        } else {
            (pos as isize + self.offset) as usize
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct FileSourceMap {
    layers: Vec<Layer>,
    address_replacements: Vec<(String, String)>,
}

impl FileSourceMap {
    pub fn insert_layer(&mut self, pos: usize, offset: isize) {
        self.layers.push(Layer { pos, offset });
    }

    pub fn insert_address_layer(
        &mut self,
        original_end_pos: usize,
        original: String,
        replacement: String,
    ) {
        let len_diff = len_difference(&original, &replacement);
        self.insert_layer(original_end_pos, len_diff);
        self.insert_address_replacement(original, replacement);
    }

    pub fn insert_address_replacement(&mut self, original: String, replacement: String) {
        self.address_replacements.push((original, replacement))
    }

    pub fn translate_error_part(&self, error_part: CompilerErrorPart) -> CompilerErrorPart {
        let CompilerErrorPart {
            location:
                Location {
                    fpath,
                    span: loc_span,
                },
            message,
        } = error_part;
        let loc_span = self.translate_span(loc_span);
        let message = self.translate_error_message(message);
        CompilerErrorPart {
            location: Location {
                fpath,
                span: loc_span,
            },
            message,
        }
    }

    pub fn translate_span(&self, (start, end): (usize, usize)) -> (usize, usize) {
        let start = self.translate_pos(start);
        let end = self.translate_pos(end);
        (start, end)
    }

    fn translate_pos(&self, pos: usize) -> usize {
        let mut real_pos = pos;
        for layer in self.layers.iter().rev() {
            real_pos = layer.translate(real_pos);
        }
        real_pos
    }

    fn translate_error_message(&self, message: String) -> String {
        let mut message = message;
        for (orig, replacement) in self.address_replacements.iter().rev() {
            message = message.replace(replacement, orig);
        }
        message
    }
}

#[derive(Debug, Default, Clone)]
pub struct ProjectSourceMap(pub HashMap<MoveFilePath, FileSourceMap>);

impl ProjectSourceMap {
    pub fn with_file_map(fpath: MoveFilePath, map: FileSourceMap) -> ProjectSourceMap {
        let mut project_map = ProjectSourceMap::default();
        project_map.0.insert(fpath, map);
        project_map
    }

    pub fn insert(&mut self, fpath: MoveFilePath, map: FileSourceMap) {
        self.0.insert(fpath, map);
    }

    pub fn transform(&self, error: CompilerError) -> CompilerError {
        let mut translated_parts = vec![];
        for error_part in error.parts.into_iter() {
            let file_source_map = &self.0[error_part.location.fpath];
            let new_error_part = file_source_map.translate_error_part(error_part);
            translated_parts.push(new_error_part);
        }
        CompilerError {
            parts: translated_parts,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Location {
    pub fpath: MoveFilePath,
    pub span: (usize, usize),
}

impl Location {
    pub fn is_inside_interval(&self, start: usize, end: usize) -> bool {
        let (loc_start, _) = self.span;
        loc_start >= start && loc_start <= end
    }
}
