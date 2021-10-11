use move_lang::errors::{FilesSourceText, Errors};
use move_lang::callback::Interact;
use move_lang::{parser, MatchedFileCommentMap, strip_comments_and_verify};
use move_lang::parser::syntax::parse_file_string;

pub fn parse_file(
    files: &mut FilesSourceText,
    fname: &'static str,
    source_buffer: String,
    interact: &mut dyn Interact,
) -> anyhow::Result<(Vec<parser::ast::Definition>, MatchedFileCommentMap, Errors)> {
    let mut errors: Errors = Vec::new();
    let (no_comments_buffer, comment_map) = match strip_comments_and_verify(fname, &source_buffer)
    {
        Err(errs) => {
            errors.extend(errs.into_iter());
            files.insert(fname, source_buffer);
            return Ok((vec![], MatchedFileCommentMap::new(), errors));
        }
        Ok(result) => result,
    };
    let (defs, comments) = match parse_file_string(fname, &no_comments_buffer, comment_map) {
        Ok(defs_and_comments) => defs_and_comments,
        Err(errs) => {
            errors.extend(errs);
            (vec![], MatchedFileCommentMap::new())
        }
    };
    files.insert(fname, source_buffer);
    interact.analyze_ast(&defs);
    Ok((defs, comments, errors))
}

// @todo tests
