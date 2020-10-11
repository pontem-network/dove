use std::str::Chars;
use crate::compiler::source_map::FileSourceMap;

struct NewNormalized<'a> {
    chars: Chars<'a>,
    prev_was_carriage_return: bool,
    pos: usize,
    source_map: &'a mut FileSourceMap,
}

impl<'a> NewNormalized<'a> {
    fn inner_next(&mut self) -> Option<char> {
        self.pos += 1;
        self.chars.next()
    }
}

const PATTERN_LENGTH: usize = 2;

impl Iterator for NewNormalized<'_> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        match self.inner_next() {
            Some('\n') if self.prev_was_carriage_return => {
                self.source_map.insert_layer(self.pos - PATTERN_LENGTH, 1);

                self.prev_was_carriage_return = false;
                match self.inner_next() {
                    Some('\r') => {
                        self.prev_was_carriage_return = true;
                        Some('\n')
                    }
                    any => {
                        self.prev_was_carriage_return = false;
                        any
                    }
                }
            }
            Some('\r') => {
                self.prev_was_carriage_return = true;
                Some('\n')
            }
            any => {
                self.prev_was_carriage_return = false;
                any
            }
        }
    }
}

pub fn normalize(s: String) -> (String, FileSourceMap) {
    let mut source_map = FileSourceMap::default();
    let normalized = NewNormalized {
        chars: s.chars(),
        prev_was_carriage_return: false,
        pos: 0,
        source_map: &mut source_map,
    };
    let normalized_string = normalized.collect::<String>();
    (normalized_string, source_map)
}
