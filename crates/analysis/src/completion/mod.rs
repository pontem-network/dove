use crate::completion::scope::{determine_scope, Scope};
use crate::db::{FilePosition, RootDatabase};
use lsp_types::CompletionItem;
use syntax::ast::SourceFile;

mod scope;

fn determine_completion_context(
    source_file: SourceFile,
    pos: (usize, usize),
) -> CompletionContext {
    CompletionContext::new(determine_scope(source_file, pos))
}

pub fn keywords(context: &CompletionContext) -> Vec<CompletionItem> {
    let items = match context.scope {
        Scope::TopLevel => vec!["script", "module", "address"],
        Scope::Module => vec!["fun", "struct", "resource", "public"],
        Scope::Script => vec!["fun"],
        Scope::Address => vec!["module"],
        Scope::Struct => vec![],
        Scope::Function => vec!["let", "return", "mut"],
        Scope::Other => vec![],
    };
    items
        .into_iter()
        .map(|kw| format!("{} ", kw))
        .map(|kw| CompletionItem::new_simple(kw, "keyword".to_string()))
        .collect()
}

pub fn builtins(context: &CompletionContext) -> Vec<CompletionItem> {
    let items = match context.scope {
        Scope::Function => vec![
            "borrow_global",
            "emit_event",
            "borrow_global_mut",
            "exists",
            "move_from",
            "move_to_sender",
        ],
        _ => vec![],
    };
    items
        .into_iter()
        .map(|label| CompletionItem::new_simple(label.to_string(), "builtin".to_string()))
        .collect()
}

pub struct CompletionContext {
    pub scope: Scope,
}

impl CompletionContext {
    pub fn new(scope: Scope) -> CompletionContext {
        CompletionContext { scope }
    }
}

pub fn completions(db: &RootDatabase, position: FilePosition) -> Vec<CompletionItem> {
    let source_file = db.source_file(position.fpath);
    let context = determine_completion_context(source_file, position.pos);

    let mut completions = vec![];
    completions.extend(keywords(&context));
    completions.extend(builtins(&context));
    completions
}
