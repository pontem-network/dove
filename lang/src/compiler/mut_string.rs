use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct MutString<'a> {
    source: &'a str,
    patch_set: BinaryHeap<Patch>,
    length_diff: isize,
}

impl<'a> MutString<'a> {
    pub fn new(source: &'a str) -> MutString {
        MutString {
            source,
            patch_set: Default::default(),
            length_diff: 0,
        }
    }

    /// Create patch to the source string.
    /// Patches should not overlap!
    pub fn make_patch(&mut self, start_offset: usize, end_offset: usize, new_value: NewValue) {
        let current_len = (end_offset - start_offset) as isize;
        let patch_len = new_value.len() as isize;
        self.length_diff += patch_len - current_len;
        self.patch_set.push(Patch {
            source_range: start_offset..end_offset,
            value: new_value,
        });
    }

    pub fn freeze(self) -> String {
        let result_len = self.source.len() + self.patch_set.len();
        let mut result = String::with_capacity(result_len);
        let mut last_patch = 0;
        for Patch {
            source_range,
            value,
        } in self.patch_set
        {
            result.push_str(&self.source[last_patch..source_range.start]);
            last_patch = source_range.end;
            result.push_str(value.as_ref());
        }
        result.push_str(&self.source[last_patch..]);
        result
    }
}

impl<'a> AsRef<str> for MutString<'a> {
    fn as_ref(&self) -> &str {
        self.source
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NewValue {
    Borrowed(&'static str),
    Owned(String),
}

impl NewValue {
    pub fn len(&self) -> usize {
        match self {
            NewValue::Borrowed(val) => val.len(),
            NewValue::Owned(val) => val.len(),
        }
    }
}

impl AsRef<str> for NewValue {
    fn as_ref(&self) -> &str {
        match self {
            NewValue::Borrowed(val) => val,
            NewValue::Owned(val) => val.as_ref(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Patch {
    source_range: Range<usize>,
    value: NewValue,
}

impl PartialOrd for Patch {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other
            .source_range
            .start
            .partial_cmp(&self.source_range.start)
    }
}

impl Ord for Patch {
    fn cmp(&self, other: &Self) -> Ordering {
        other.source_range.start.cmp(&self.source_range.start)
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Cow;

    use crate::compiler::mut_string::{MutString, NewValue};

    #[test]
    fn test_mut_string() {
        let source = r"
            script {
                use 0x01::Event;
                use 1exaAg2VJRQbyUBAeXcktChCAqjVP9TUxF3zo23R2T6EGdE::Math;
                use 0x02::Invald;

                fun main(account: &signer, a: u64, b: u64) {
                    let sum = Math::add(a, b);
                    Event::emit(account, sum);
                }
            }
        ";
        let mut mut_str = MutString::new(source);
        mut_str.make_patch(
            75,
            75 + 47,
            NewValue::Borrowed(
                "0x1CF326C5AAA5AF9F0E2791E66310FE8F044FAADAF12567EAA0976959D1F7731F",
            ),
        );
        mut_str.make_patch(129, 163, NewValue::Borrowed(""));

        assert_eq!(
            mut_str.freeze().as_str(),
            r"
            script {
                use 0x01::Event;
                use 0x1CF326C5AAA5AF9F0E2791E66310FE8F044FAADAF12567EAA0976959D1F7731F::Math;

                fun main(account: &signer, a: u64, b: u64) {
                    let sum = Math::add(a, b);
                    Event::emit(account, sum);
                }
            }
        "
        );
    }
}
