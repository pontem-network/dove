use lang::compiler::mut_string::MutString;
use move_lang::parser::ast::Type;
use move_lang::errors::Error;
use move_lang::parser::lexer::Lexer;
use move_lang::parser::syntax::parse_type;
use lang::compiler::address::ss58::replace_ss58_addresses;

/// parse type params
// N
// N<t1, ... , tn>
pub fn str_to_typeparams(str: &str) -> Result<Type, Error> {
    let mut mut_string = MutString::new(str);
    replace_ss58_addresses(str, &mut mut_string, &mut Default::default());
    let tp = mut_string.freeze();
    let mut lexer = Lexer::new(&tp, "tp", Default::default());
    lexer.advance()?;
    parse_type(&mut lexer)
}

#[cfg(test)]
mod tests {
    use move_lang::parser::ast::{Type, Type_, ModuleAccess};
    use crate::parser::types::to_typeparams::str_to_typeparams;

    fn get_apply(tp: &Type) -> Option<(Box<ModuleAccess>, Vec<Type>)> {
        match &tp.value {
            Type_::Apply(m, pt) => Some((m.clone(), pt.clone())),
            _ => None,
        }
    }
    fn get_name(tp: &Type) -> Option<String> {
        get_apply(tp).map(|(m, l)| {
            let mut r = m.to_string();
            if !l.is_empty() {
                let t = l
                    .iter()
                    .filter_map(|t| get_name(t))
                    .collect::<Vec<String>>()
                    .join(", ");
                r = r + "<" + &t + ">"
            }
            r
        })
    }

    #[test]
    fn test_str_to_typeparams() {
        assert_eq!(
            Some("u8".to_string()),
            get_name(&str_to_typeparams("u8").unwrap())
        );

        assert_eq!(
            Some("0x1::Block::T<Block::T>".to_string()),
            get_name(&str_to_typeparams("0x1::Block::T<Block::T>").unwrap())
        );
        assert_eq!(
            Some("T<T, u8>".to_string()),
            get_name(&str_to_typeparams("T<T, u8>").unwrap())
        );
    }
}
