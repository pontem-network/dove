use syntax::ast::{Definition, SourceFile};
use tree_sitter::Point;

#[derive(Debug, Eq, PartialEq)]
pub enum Scope {
    TopLevel,
    Address,
    Module,
    Script,
    Struct,
    Function,
    Other,
}

pub fn determine_scope(source_file: SourceFile, pos: (usize, usize)) -> Scope {
    // first position of node belongs to previous node for autocompletion
    // let column = pos.1.saturating_sub(1);
    let column = pos.1;
    let pos = Point::new(pos.0, column);

    let definition = match source_file.definition() {
        Some(definition) => definition,
        None => {
            return Scope::TopLevel;
        }
    };
    match definition {
        Definition::ScriptBlock(script_block) => {
            if !script_block.is_position_inside((pos.row, pos.column)) {
                return Scope::TopLevel;
            }

            let mut scope = Scope::Script;
            let mut spanned_node = script_block
                .node
                .named_descendant_for_point_range(pos, pos)
                .unwrap();
            loop {
                match spanned_node.kind() {
                    // TODO: make generic cast() function, Node -> AstNode using kind()
                    // TODO: make NodeKind enum
                    "usual_function_definition" => {
                        scope = if spanned_node.start_position() == pos {
                            Scope::Script
                        } else {
                            Scope::Function
                        };
                        break;
                    }
                    "script_block" => {
                        break;
                    }
                    _ => {
                        spanned_node = spanned_node.parent().unwrap();
                        continue;
                    }
                }
            }
            scope
        }
        Definition::AddressBlock(address_block) => {
            if !address_block.is_position_inside((pos.row, pos.column)) {
                return Scope::TopLevel;
            }
            Scope::Module
        }
        Definition::ModuleBlock(module_block) => {
            if !module_block.is_position_inside((pos.row, pos.column)) {
                return Scope::TopLevel;
            }
            Scope::Module
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn into_file_and_pos(text: &str) -> (String, (usize, usize)) {
        let mut replaced = vec![];
        let mut start = None;
        for (line, line_text) in text.split('\n').enumerate() {
            match line_text.find("<|>") {
                Some(character) => {
                    let new_line_text = line_text.replace("<|>", "");
                    start = Some((line, character));
                    replaced.push(new_line_text);
                }
                None => replaced.push(line_text.to_string()),
            }
        }
        let replaced = replaced.join("\n");
        (replaced, start.unwrap())
    }

    fn determine_pos_scope(source: &str) -> Scope {
        let (source, pos) = into_file_and_pos(source);
        let source_file = SourceFile::new(source);
        determine_scope(source_file, pos)
    }

    #[test]
    fn toplevel_scope() {
        assert_eq!(determine_pos_scope("<|>"), Scope::TopLevel);
        assert_eq!(determine_pos_scope("\n\n\n<|>\n    \n"), Scope::TopLevel);
        assert_eq!(determine_pos_scope("<|> module {}"), Scope::TopLevel);
        assert_eq!(determine_pos_scope("<|>module {}"), Scope::TopLevel);
        assert_eq!(determine_pos_scope("module {}<|>"), Scope::TopLevel);
        assert_eq!(
            determine_pos_scope("script { fun main() {} }<|>"),
            Scope::TopLevel
        );
        assert_eq!(determine_pos_scope("module {} <|>"), Scope::TopLevel);
        assert_eq!(determine_pos_scope("module Module {} <|>"), Scope::TopLevel);
    }

    #[test]
    fn script_scope() {
        assert_eq!(determine_pos_scope("script {<|>  }"), Scope::Script);
        assert_eq!(
            determine_pos_scope("script {<|> fun main() {} }"),
            Scope::Script
        );
        assert_eq!(
            determine_pos_scope("script { fun main() {} <|>}"),
            Scope::Script
        );
        assert_eq!(
            determine_pos_scope("script { use 0x0::Transaction; <|>fun main() {}}"),
            Scope::Script
        );
        assert_eq!(
            determine_pos_scope("script { <|>use 0x0::Transaction; fun main() {}}"),
            Scope::Script
        );
    }

    #[test]
    fn script_function_scope() {
        assert_eq!(
            determine_pos_scope("script { fun main() {<|>} }"),
            Scope::Function
        );
        assert_eq!(
            determine_pos_scope("script { fun main() {let a = 1; <|>} }"),
            Scope::Function
        );
    }

    // #[test]
    // fn module_scope_inside_module_block() {
    //     let source = "address 0x0 { module Module { fun myfunction() {} } module Module2 {} }";
    //     let file = SourceFile::new(source.to_string());
    //     assert_eq!(determine_scope(file.clone(), (0, 14)), Scope::Address);
    //     assert_eq!(determine_scope(file.clone(), (0, 30)), Scope::Module);
    // }
}
