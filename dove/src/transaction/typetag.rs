use move_core_types::language_storage::TypeTag;
use anyhow::Error;
use move_lang::parser::ast::Type;
use move_lang::parser::lexer::Lexer;
use move_lang::parser::syntax::parse_type;
use lang::lexer::unwrap_spanned_ty;
use lang::compiler::mut_string::MutString;
use lang::compiler::address::ss58::replace_ss58_addresses;

/// Convert T => TypeTag
pub trait ConvertToTypeTag: Sized + 'static {
    /// Convert T => TypeTag
    fn to_typetag(&self) -> Result<TypeTag, Error>;
}

impl ConvertToTypeTag for Type {
    fn to_typetag(&self) -> Result<TypeTag, Error> {
        unwrap_spanned_ty(self.clone())
    }
}
impl ConvertToTypeTag for String {
    fn to_typetag(&self) -> Result<TypeTag, Error> {
        let mut mut_string = MutString::new(self);
        replace_ss58_addresses(self, &mut mut_string, &mut Default::default());
        let tp = mut_string.freeze();
        let mut lexer = Lexer::new(&tp, "tp", Default::default());
        lexer
            .advance()
            .map_err(|err| Error::msg(format!("{:?}", err)))?;
        parse_type_params(&mut lexer)
    }
}

/// Convert Vec<ConvertToTypeTag> => Vec<TypeTags>
pub trait ConvertVecTypeToVecTypeTag {
    /// Convert Vec<ConvertToTypeTag> => Vec<TypeTags>
    fn to_typetag(&self) -> Result<Vec<TypeTag>, Error>;
}
impl<T: ConvertToTypeTag> ConvertVecTypeToVecTypeTag for Vec<T> {
    fn to_typetag(&self) -> Result<Vec<TypeTag>, Error> {
        self.iter()
            .map(|value| value.to_typetag())
            .collect::<Result<Vec<TypeTag>, _>>()
    }
}

/// parse type params
///
/// u8 => TypeTag::U8
/// u64 => TypeTag::U64
/// ...
pub fn parse_type_params(lexer: &mut Lexer) -> Result<TypeTag, Error> {
    let ty = parse_type(lexer).map_err(|err| Error::msg(format!("{:?}", err)))?;
    unwrap_spanned_ty(ty)
}
