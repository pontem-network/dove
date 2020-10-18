use codespan::{
    ByteIndex, ColumnIndex, LineIndex, LineIndexOutOfBoundsError, Location, LocationError,
    RawIndex, Span,
};
use lsp_types::Position;

/// A file that is stored in the database.
#[derive(Debug, Clone)]
pub struct File<S> {
    /// The source code of the file.
    source: S,
    /// The starting byte indices in the source code.
    line_starts: Vec<ByteIndex>,
}

impl<S> File<S>
where
    S: AsRef<str>,
{
    pub fn new(source: S) -> Self {
        let line_starts = codespan_reporting9::files::line_starts(source.as_ref())
            .map(|i| ByteIndex::from(i as u32))
            .collect();

        File {
            source,
            line_starts,
        }
    }

    pub fn position(&self, byte_index: ByteIndex) -> Result<Position, LocationError> {
        let line_index = self.line_index(byte_index);
        let line_start_index =
            self.line_start(line_index)
                .map_err(|_| LocationError::OutOfBounds {
                    given: byte_index,
                    span: self.source_span(),
                })?;
        let line_src = self
            .source
            .as_ref()
            .get(line_start_index.to_usize()..byte_index.to_usize())
            .ok_or_else(|| {
                let given = byte_index;
                if given >= self.source_span().end() {
                    let span = self.source_span();
                    LocationError::OutOfBounds { given, span }
                } else {
                    LocationError::InvalidCharBoundary { given }
                }
            })?;

        let loc = Location {
            line: line_index,
            column: ColumnIndex::from(line_src.chars().count() as u32),
        };
        Ok(Position::new(
            loc.line.to_usize() as u64,
            loc.column.to_usize() as u64,
        ))
    }

    fn line_start(&self, line_index: LineIndex) -> Result<ByteIndex, LineIndexOutOfBoundsError> {
        use std::cmp::Ordering;

        match line_index.cmp(&self.last_line_index()) {
            Ordering::Less => Ok(self.line_starts[line_index.to_usize()]),
            Ordering::Equal => Ok(self.source_span().end()),
            Ordering::Greater => Err(LineIndexOutOfBoundsError {
                given: line_index,
                max: self.last_line_index(),
            }),
        }
    }

    fn last_line_index(&self) -> LineIndex {
        LineIndex::from(self.line_starts.len() as RawIndex)
    }

    fn line_index(&self, byte_index: ByteIndex) -> LineIndex {
        match self.line_starts.binary_search(&byte_index) {
            // Found the start of a line
            Ok(line) => LineIndex::from(line as u32),
            Err(next_line) => LineIndex::from(next_line as u32 - 1),
        }
    }

    fn source_span(&self) -> Span {
        Span::from_str(self.source.as_ref())
    }
}
