use anyhow::Error as anyhow_error;
use move_ir_types::location::Spanned;
use move_core_types::language_storage::{TypeTag, StructTag};
use move_core_types::account_address::AccountAddress;
use move_core_types::identifier::Identifier;
use move_lang::parser::ast::{ModuleAccess_, Type, Type_};
use move_lang::errors::{Error};
use move_lang::parser::lexer::Lexer;
use move_lang::parser::syntax::parse_type;
use crate::compiler::mut_string::MutString;
use crate::compiler::address::ss58::replace_ss58_addresses;

type STypeTag = Spanned<TypeTag>;

/// Convert T => Spanned<TypeTag>
pub trait ConvertToTypeTag: Sized + 'static {
    /// Convert T => Spanned<TypeTag>
    fn to_typetag(&self) -> Result<STypeTag, Error>;
}

impl ConvertToTypeTag for Type {
    fn to_typetag(&self) -> Result<STypeTag, Error> {
        fn to_(ty: Type, this: Option<AccountAddress>) -> Result<Spanned<TypeTag>, Error> {
            let st = match ty.value.clone() {
                Type_::Apply(ma, mut ty_params) => {
                    match (ma.value, this) {
                        // N
                        (ModuleAccess_::Name(name), this) => match name.value.as_ref() {
                            "bool" => TypeTag::Bool,
                            "u8" => TypeTag::U8,
                            "u64" => TypeTag::U64,
                            "u128" => TypeTag::U128,
                            "address" => TypeTag::Address,
                            "signer" => TypeTag::Signer,
                            "Vec" if ty_params.len() == 1 => {
                                to_(ty_params.pop().unwrap(), this)?.value
                            }
                            _ => return Err(_error(
                                &ty,
                                "Could not parse input: type without struct name & module address",
                            )),
                        },
                        // M.S
                        (ModuleAccess_::ModuleAccess(_module, _struct_name), None) => {
                            return Err(_error(
                                &ty,
                                "Could not parse input: type without module address",
                            ))
                        }
                        // M.S + parent address
                        (ModuleAccess_::ModuleAccess(name, struct_name), Some(this)) => {
                            TypeTag::Struct(StructTag {
                                address: this,
                                module: Identifier::new(name.0.value)
                                    .map_err(|err| _error(&ty, err.to_string().as_str()))?,
                                name: Identifier::new(struct_name.value)
                                    .map_err(|err| _error(&ty, err.to_string().as_str()))?,
                                type_params: ty_params
                                    .into_iter()
                                    .map(|ty| to_(ty, Some(this)).map(|v| v.value))
                                    .collect::<Result<Vec<TypeTag>, Error>>()?,
                            })
                        }

                        // OxADDR.M.S
                        (ModuleAccess_::QualifiedModuleAccess(module_id, struct_name), _) => {
                            let (address, name) = module_id.value;
                            let address = AccountAddress::new(address.to_u8());
                            TypeTag::Struct(StructTag {
                                address,
                                module: Identifier::new(name)
                                    .map_err(|err| _error(&ty, err.to_string().as_str()))?,
                                name: Identifier::new(struct_name.value)
                                    .map_err(|err| _error(&ty, err.to_string().as_str()))?,
                                type_params: ty_params
                                    .into_iter()
                                    .map(|ty| to_(ty, Some(address)).map(|v| v.value))
                                    .collect::<Result<Vec<TypeTag>, Error>>()?,
                            })
                        }
                    }
                }
                _ => return Err(_error(&ty, "Could not parse input: unsupported type")),
            };

            _ok(&ty, st)
        }

        to_(self.clone(), None)
    }
}
impl ConvertToTypeTag for Result<Type, Error> {
    fn to_typetag(&self) -> Result<STypeTag, Error> {
        self.clone().and_then(|tp| tp.to_typetag())
    }
}

impl ConvertToTypeTag for String {
    /// parse type params
    ///
    /// u8 => TypeTag::U8
    /// u64 => TypeTag::U64
    /// ...
    fn to_typetag(&self) -> Result<STypeTag, Error> {
        let mut mut_string = MutString::new(self);
        replace_ss58_addresses(self, &mut mut_string, &mut Default::default());
        let tp = mut_string.freeze();
        let mut lexer = Lexer::new(&tp, "tp", Default::default());
        lexer.advance()?;
        parse_type(&mut lexer)?.to_typetag()
    }
}

/// Convert Vec<ConvertToTypeTag> => Vec<Spanned<TypeTag>>
pub trait ConvertVecTypeToVecTypeTag {
    /// Convert Vec<ConvertToTypeTag> => Vec<TypeTags>
    fn to_typetag(&self) -> Result<Vec<STypeTag>, Error>;
}
impl<T: ConvertToTypeTag> ConvertVecTypeToVecTypeTag for Vec<T> {
    fn to_typetag(&self) -> Result<Vec<STypeTag>, Error> {
        self.iter()
            .map(|value| value.to_typetag())
            .collect::<Result<Vec<STypeTag>, _>>()
    }
}

/// Convert Spanned<TypeTag> => TypeTag
trait UnSpanned {
    fn unspanned(self) -> TypeTag;
}
impl UnSpanned for STypeTag {
    fn unspanned(self) -> TypeTag {
        self.value
    }
}

/// Convert Result<Spanned<TypeTag>,T> => Result<TypeTag,T>
trait ResultUnSpanned<T> {
    fn unspanned(self) -> Result<TypeTag, T>;
}
impl<T> ResultUnSpanned<T> for Result<STypeTag, T> {
    fn unspanned(self) -> Result<TypeTag, T> {
        self.map(|v| v.unspanned())
    }
}

/// Convert Result<Vec<Spanned<TypeTag>>,T> => Result<Vec<TypeTag>,T>
pub trait VecUnSpanned {
    /// Convert Vec<ConvertToTypeTag> => Vec<TypeTags>
    fn unspanned(self) -> Vec<TypeTag>;
}
impl VecUnSpanned for Vec<STypeTag> {
    fn unspanned(self) -> Vec<TypeTag> {
        self.iter()
            .map(|value| value.value.clone())
            .collect::<Vec<TypeTag>>()
    }
}

/// Convert Result<Vec<Spanned<TypeTag>>,T> => Result<Vec<TypeTag>,T>
pub trait ResultVecUnSpanned<T> {
    /// Convert Vec<ConvertToTypeTag> => Vec<TypeTags>
    fn unspanned(self) -> Result<Vec<TypeTag>, T>;
}
impl<T> ResultVecUnSpanned<T> for Result<Vec<STypeTag>, T> {
    fn unspanned(self) -> Result<Vec<TypeTag>, T> {
        self.map(|list| {
            list.iter()
                .map(|value| value.value.clone())
                .collect::<Vec<TypeTag>>()
        })
    }
}

pub trait ConvertMoveLangErrorToAnyhow<T>: Clone {
    fn error_to_anyhow(self) -> Result<T, anyhow_error>;
}
impl<T: Clone> ConvertMoveLangErrorToAnyhow<T> for Result<T, Error> {
    fn error_to_anyhow(self) -> Result<T, anyhow_error> {
        self.map_err(|err| anyhow_error::msg(format!("{:?}", err)))
    }
}

pub trait ResultUnspanedAndErrorToAnyhow {
    fn unspaned_and_error_to_anyhow(self) -> Result<TypeTag, anyhow_error>;
}
impl ResultUnspanedAndErrorToAnyhow for Result<STypeTag, Error> {
    fn unspaned_and_error_to_anyhow(self) -> Result<TypeTag, anyhow_error> {
        self.unspanned().error_to_anyhow()
    }
}

pub trait ResultVecUnspanedAndErrorToAnyhow {
    fn unspaned_and_error_to_anyhow(self) -> Result<Vec<TypeTag>, anyhow_error>;
}
impl ResultVecUnspanedAndErrorToAnyhow for Result<Vec<STypeTag>, Error> {
    fn unspaned_and_error_to_anyhow(self) -> Result<Vec<TypeTag>, anyhow_error> {
        self.unspanned().error_to_anyhow()
    }
}

fn _error(base: &Type, message: &str) -> Error {
    vec![(base.loc, message.to_string())]
}

fn _ok(base: &Type, tp: TypeTag) -> Result<STypeTag, Error> {
    Ok(STypeTag::new(base.loc, tp))
}

#[cfg(test)]
mod tests {
    use crate::lexer::to_typetag::ConvertToTypeTag;

    #[test]
    fn test_parse() {
        assert!("0x1::Foo".to_string().to_typetag().is_err());

        let inputs = [
            "0x1::Foo::Res",
            "0x1::Foo::Res<Bar::Struct>",
            "0x1::Foo::Res<0x1::Bar::Struct>",
            "0x1::Foo::Res<0x1::Bar::T>[42]",
            "0x1::Foo::Res<0x1::Bar::T<u128>>[42]",
            "0x1::Foo::Res<Bar::T<u128>>",
            "0x1::Foo::Res<Vec<u128>>",
            "0x1::Foo::Res<Vec<u128>>[42]",
            "0x1::Foo::Bar::Ignored<Parts>",
        ];

        inputs
            .iter()
            .cloned()
            .map(|inp| (inp, inp.to_string().to_typetag()))
            .for_each(|(inp, res)| {
                assert!(res.is_ok(), "failed on '{}'", inp);
                println!("{:?}", res.unwrap());
            });
    }

    #[test]
    fn test_parse_ss58() {
        // //Alice/ pub: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY =>
        // 0xD43593C715FDD31C61141ABD04A99FD6822C8558854CCDE39A5684E7A56DA27D
        const ADDR: &str = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
        assert!("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY::Foo"
            .to_string()
            .to_typetag()
            .is_err());

        let inputs = [
            &format!("{}::Foo::Res", ADDR),
            &format!("{}::Foo::Res<Bar::Struct>", ADDR),
            &format!("{}::Foo::Res<{0:}::Bar::Struct>", ADDR),
            &format!("{}::Foo::Res<{0:}::Bar::T>[42]", ADDR),
            &format!("{}::Foo::Res<{0:}::Bar::T<u128>>[42]", ADDR),
            &format!("{}::Foo::Res<Bar::T<u128>>", ADDR),
            &format!("{}::Foo::Res<Vec<u128>>", ADDR),
            &format!("{}::Foo::Res<Vec<u128>>[42]", ADDR),
            &format!("{}::Foo::Bar::Ignored<Parts>", ADDR),
        ];

        inputs
            .iter()
            .cloned()
            .map(|inp| (inp, inp.to_typetag()))
            .for_each(|(inp, res)| {
                assert!(res.is_ok(), "failed on '{}'", inp);
                println!("{:?}", res.unwrap());
            });
    }
}
