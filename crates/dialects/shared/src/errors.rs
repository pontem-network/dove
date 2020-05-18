use std::collections::{BTreeMap, HashMap};

use serde::export::Formatter;
use std::fmt;

use utils::FilePath;

fn is_inside_interval(pos: usize, interval: (usize, usize)) -> bool {
    pos >= interval.0 && pos <= interval.1
}

// needs to be sorted
pub type OffsetsMap = BTreeMap<(usize, usize), usize>;

#[derive(Debug, Default, Clone)]
pub struct ProjectOffsetsMap(pub HashMap<FilePath, OffsetsMap>);

impl ProjectOffsetsMap {
    pub fn with_file_map(fpath: FilePath, map: OffsetsMap) -> ProjectOffsetsMap {
        let mut project_map = ProjectOffsetsMap::default();
        project_map.0.insert(fpath, map);
        project_map
    }

    pub fn insert(&mut self, fpath: FilePath, map: OffsetsMap) {
        self.0.insert(fpath, map);
    }

    fn translate_pos(&self, pos: usize, fpath: FilePath) -> usize {
        for (interval, offset) in &self.0[fpath] {
            if is_inside_interval(pos, *interval) {
                return pos - *offset;
            }
        }
        pos
    }

    pub fn apply_offsets_to_error(&self, error: CompilerError) -> CompilerError {
        let mut translated_parts = vec![];
        for error_part in error.parts.into_iter() {
            let Location {
                fpath,
                span: (loc_start, loc_end),
            } = error_part.location;
            let loc_start = self.translate_pos(loc_start, fpath);
            let loc_end = self.translate_pos(loc_end, fpath);
            let new_error_part = error_part.with_new_span(loc_start, loc_end);
            translated_parts.push(new_error_part);
        }
        CompilerError {
            parts: translated_parts,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Location {
    pub fpath: FilePath,
    pub span: (usize, usize),
}

impl Location {
    pub fn is_inside_interval(&self, start: usize, end: usize) -> bool {
        let (loc_start, _) = self.span;
        loc_start >= start && loc_start <= end
    }
}

#[derive(Debug, Clone)]
pub struct CompilerErrorPart {
    pub location: Location,
    pub message: String,
}

impl CompilerErrorPart {
    pub fn with_new_span(self, start: usize, end: usize) -> CompilerErrorPart {
        let location = Location {
            fpath: self.location.fpath,
            span: (start, end),
        };
        CompilerErrorPart {
            location,
            message: self.message,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CompilerError {
    pub parts: Vec<CompilerErrorPart>,
}

#[derive(Debug, Default)]
pub struct ExecCompilerError(pub Vec<CompilerError>, pub ProjectOffsetsMap);

impl ExecCompilerError {
    pub fn apply_offsets(self) -> Vec<CompilerError> {
        let ExecCompilerError(errors, offsets_map) = self;
        errors
            .into_iter()
            .map(|error| offsets_map.apply_offsets_to_error(error))
            .collect()
    }

    pub fn extend(&mut self, other: ExecCompilerError) {
        let ExecCompilerError(errors, proj_offsets_map) = other;
        self.0.extend(errors);
        for (fpath, offsets_map) in proj_offsets_map.0.into_iter() {
            self.1.insert(fpath, offsets_map);
        }
    }
}

impl std::error::Error for ExecCompilerError {}

impl fmt::Display for ExecCompilerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
