use lsp_types::CompletionItem;

pub fn get_keywords() -> Vec<CompletionItem> {
    vec![
        "struct", "resource", "module", "fun", "return", "public", "native", "mut", "let",
        "move", "copy",
    ]
    .into_iter()
    .map(|kw| format!("{} ", kw))
    .map(|kw| CompletionItem::new_simple(kw, "keyword".to_string()))
    .collect()
}

pub fn get_builtins() -> Vec<CompletionItem> {
    vec![
        "borrow_global",
        "emit_event",
        "borrow_global_mut",
        "exists",
        "move_from",
        "move_to_sender",
    ]
    .into_iter()
    .map(|label| CompletionItem::new_simple(label.to_string(), "builtin".to_string()))
    .collect()
}
