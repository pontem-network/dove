use std::collections::HashMap;
use diem::move_lang::errors::Errors;
use diem::move_ir_types::location::Loc;
use codespan::{Span, ByteIndex};

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
pub struct FileOffsetMap {
    layers: Vec<Layer>,
    address_replacements: Vec<(String, String)>,
}

impl FileOffsetMap {
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

    pub fn translate_error(&self, (loc, msg): (Loc, String)) -> (Loc, String) {
        (
            Loc::new(loc.file(), self.translate_span(loc.span())),
            self.translate_message(msg),
        )
    }

    pub fn translate_span(&self, span: Span) -> Span {
        Span::new(
            ByteIndex(self.translate_pos(span.start().to_usize()) as u32),
            ByteIndex(self.translate_pos(span.end().to_usize()) as u32),
        )
    }

    fn translate_pos(&self, pos: usize) -> usize {
        let mut real_pos = pos;
        for layer in self.layers.iter().rev() {
            real_pos = layer.translate(real_pos);
        }
        real_pos
    }

    fn translate_message(&self, mut msg: String) -> String {
        for (orig, replacement) in self.address_replacements.iter().rev() {
            msg = msg.replace(replacement, orig);
        }
        msg
    }
}

#[derive(Debug, Default, Clone)]
pub struct ProjectOffsetMap(pub HashMap<&'static str, FileOffsetMap>);

impl ProjectOffsetMap {
    pub fn with_file_map(fpath: &'static str, map: FileOffsetMap) -> ProjectOffsetMap {
        let mut project_map = ProjectOffsetMap::default();
        project_map.0.insert(fpath, map);
        project_map
    }

    pub fn insert(&mut self, fpath: &'static str, map: FileOffsetMap) {
        self.0.insert(fpath, map);
    }

    pub fn transform(&self, errors: Errors) -> Errors {
        errors
            .into_iter()
            .map(|error| {
                error
                    .into_iter()
                    .map(|(loc, msg)| self.0[loc.file()].translate_error((loc, msg)))
                    .collect()
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct Location {
    pub fpath: &'static str,
    pub span: (usize, usize),
}

impl Location {
    pub fn is_inside_interval(&self, start: usize, end: usize) -> bool {
        let (loc_start, _) = self.span;
        loc_start >= start && loc_start <= end
    }
}
