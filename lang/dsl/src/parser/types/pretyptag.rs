use move_core_types::identifier::Identifier;
use move_core_types::language_storage::StructTag as TStructTag;
use move_ir_types::location::Spanned;
use move_lang::errors::Error;
use move_lang::parser::ast::{Type, Type_, ModuleAccess_};
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::TypeTag;
use crate::parser::types::to_typeparams::str_to_typeparams;

pub type PreTypeTag = Spanned<PreTypeTag_>;

#[derive(Debug, PartialEq, Hash, Eq, Clone, PartialOrd, Ord)]
pub enum PreTypeTag_ {
    Bool,
    U8,
    U64,
    U128,
    Address,
    Signer,
    Vector(Box<PreTypeTag>),
    Struct(StructTag),
    Alias(AliasTag),
}

#[derive(Debug, PartialEq, Hash, Eq, Clone, PartialOrd, Ord)]
pub struct StructTag {
    pub address: AccountAddress,
    pub module: Identifier,
    pub name: Identifier,
    pub type_args: Vec<PreTypeTag>,
}
#[derive(Debug, PartialEq, Hash, Eq, Clone, PartialOrd, Ord)]
pub struct AliasTag {
    pub name: Identifier,
    pub type_args: Vec<PreTypeTag>,
}

pub trait ToPreTypeTag {
    fn to_pretypetag(&self) -> Result<PreTypeTag, Error>;
}
impl ToPreTypeTag for Type {
    fn to_pretypetag(&self) -> Result<PreTypeTag, Error> {
        type_to_pretypetag(self)
    }
}
impl ToPreTypeTag for &str {
    fn to_pretypetag(&self) -> Result<PreTypeTag, Error> {
        str_to_typeparams(&self).and_then(|v| type_to_pretypetag(&v))
    }
}
impl ToPreTypeTag for String {
    fn to_pretypetag(&self) -> Result<PreTypeTag, Error> {
        self.as_str().to_pretypetag()
    }
}
impl ToPreTypeTag for PreTypeTag {
    fn to_pretypetag(&self) -> Result<PreTypeTag, Error> {
        Ok(self.to_owned())
    }
}

pub trait ToTypeTag {
    fn to_typetag(&self) -> Result<TypeTag, Error>
    where
        Self: Sized + ToPreTypeTag,
    {
        pretypetag_to_typetag(&self.to_pretypetag()?)
    }
}
impl<T: ToPreTypeTag> ToTypeTag for T {
    fn to_typetag(&self) -> Result<TypeTag, Error> {
        pretypetag_to_typetag(&self.to_pretypetag()?)
    }
}

pub trait ToVecTag {
    fn to_pretypetag(&self) -> Result<Vec<PreTypeTag>, Error>;
    fn to_typetag(&self) -> Result<Vec<TypeTag>, Error>;
}
impl<T: ToPreTypeTag> ToVecTag for Vec<T> {
    fn to_pretypetag(&self) -> Result<Vec<PreTypeTag>, Error> {
        self.iter().map(|v| v.to_pretypetag()).collect()
    }
    fn to_typetag(&self) -> Result<Vec<TypeTag>, Error> {
        self.iter().map(|v| v.to_typetag()).collect()
    }
}

pub trait ToAnyhowError<T> {
    fn error_anyhow(self) -> Result<T, anyhow::Error>;
}
impl<T> ToAnyhowError<T> for Result<T, Error> {
    fn error_anyhow(self) -> Result<T, anyhow::Error> {
        self.map_err(|err| anyhow::Error::msg(format!("{:?}", err)))
    }
}

pub fn type_to_pretypetag(tp: &Type) -> Result<PreTypeTag, Error> {
    fn to_(ty: &Type, this: Option<AccountAddress>) -> Result<PreTypeTag, Error> {
        let loc = ty.loc.to_owned();
        let st = match ty.value.to_owned() {
            Type_::Apply(ma, mut ty_params) => {
                match (ma.value, this) {
                    // N
                    (ModuleAccess_::Name(name), this) => match name.value.as_ref() {
                        "bool" => PreTypeTag_::Bool,
                        "u8" => PreTypeTag_::U8,
                        "u64" => PreTypeTag_::U64,
                        "u128" => PreTypeTag_::U128,
                        "address" => PreTypeTag_::Address,
                        "signer" => PreTypeTag_::Signer,
                        "Vec" if ty_params.len() == 1 => PreTypeTag_::Vector(
                            to_(ty_params.pop().as_ref().unwrap(), this)?.into(),
                        ),
                        name_alias => PreTypeTag_::Alias(AliasTag {
                            name: Identifier::new(name_alias)
                                .map_err(|err| vec![(loc, err.to_string())])?,
                            type_args: ty_params
                                .into_iter()
                                .map(|ty| to_(&ty, this))
                                .collect::<Result<Vec<PreTypeTag>, Error>>()?,
                        }),
                    },
                    // M.S
                    (ModuleAccess_::ModuleAccess(_module, _struct_name), None) => {
                        return Err(vec![(
                            ty.loc.to_owned(),
                            "Could not parse input: type without module address".to_string(),
                        )])
                    }
                    // M.S + parent address
                    (ModuleAccess_::ModuleAccess(name, struct_name), Some(this)) => {
                        PreTypeTag_::Struct(StructTag {
                            address: this,
                            module: Identifier::new(name.0.value)
                                .map_err(|err| vec![(loc, err.to_string())])?,
                            name: Identifier::new(struct_name.value)
                                .map_err(|err| vec![(loc, err.to_string())])?,
                            type_args: ty_params
                                .into_iter()
                                .map(|ty| to_(&ty, Some(this)))
                                .collect::<Result<Vec<PreTypeTag>, Error>>()?,
                        })
                    }

                    // OxADDR.M.S
                    (ModuleAccess_::QualifiedModuleAccess(module_id, struct_name), _) => {
                        let (address, name) = module_id.value;
                        let address = AccountAddress::new(address.to_u8());
                        PreTypeTag_::Struct(StructTag {
                            address,
                            module: Identifier::new(name)
                                .map_err(|err| vec![(loc, err.to_string())])?,
                            name: Identifier::new(struct_name.value)
                                .map_err(|err| vec![(loc, err.to_string())])?,
                            type_args: ty_params
                                .into_iter()
                                .map(|ty| to_(&ty, Some(address)))
                                .collect::<Result<Vec<PreTypeTag>, Error>>()?,
                        })
                    }
                }
            }
            _ => {
                return Err(vec![(
                    ty.loc.to_owned(),
                    "Could not parse input: unsupported type".to_string(),
                )])
            }
        };

        Ok(PreTypeTag::new(ty.loc.to_owned(), st))
    }

    to_(tp, None)
}

pub fn pretypetag_to_typetag(tp: &PreTypeTag) -> Result<TypeTag, Error> {
    fn to_(tp: &PreTypeTag) -> Result<TypeTag, Error> {
        let loc = tp.loc.to_owned();
        let st = match &tp.value {
            PreTypeTag_::Bool => TypeTag::Bool,
            PreTypeTag_::U8 => TypeTag::U8,
            PreTypeTag_::U64 => TypeTag::U64,
            PreTypeTag_::U128 => TypeTag::U128,
            PreTypeTag_::Address => TypeTag::Address,
            PreTypeTag_::Signer => TypeTag::Signer,
            PreTypeTag_::Vector(v) => TypeTag::Vector(to_(v)?.into()),
            PreTypeTag_::Struct(s) => TypeTag::Struct(TStructTag {
                address: s.address.to_owned(),
                module: s.module.to_owned(),
                name: s.name.to_owned(),
                type_params: s
                    .type_args
                    .iter()
                    .map(to_)
                    .collect::<Result<Vec<TypeTag>, Error>>()?,
            }),
            PreTypeTag_::Alias(_) => {
                return Err(vec![(
                    loc,
                    "Alias: Could not parse input: unsupported type".to_string(),
                )])
            }
        };

        Ok(st)
    }

    to_(tp)
}

#[cfg(test)]
mod tests {
    use move_lang::parser::syntax::spanned;
    use move_lang::shared::Address;
    use move_core_types::identifier::Identifier;
    use move_core_types::account_address::AccountAddress;
    use move_core_types::language_storage::{TypeTag, StructTag as TStructTag};
    use crate::parser::types::pretyptag::{
        PreTypeTag_, StructTag, PreTypeTag, AliasTag, ToPreTypeTag, ToTypeTag, ToVecTag,
    };

    fn sp(pr: PreTypeTag_) -> PreTypeTag {
        spanned("tp", 0, 0, pr)
    }

    #[test]
    fn test_to_pretypetag() {
        assert_eq!(PreTypeTag_::Bool, "bool".to_pretypetag().unwrap().value);
        assert_eq!(PreTypeTag_::U8, "u8".to_pretypetag().unwrap().value);
        assert_eq!(PreTypeTag_::U64, "u64".to_pretypetag().unwrap().value);
        assert_eq!(PreTypeTag_::U128, "u128".to_pretypetag().unwrap().value);
        assert_eq!(
            PreTypeTag_::Address,
            "address".to_pretypetag().unwrap().value
        );
        assert_eq!(PreTypeTag_::Signer, "signer".to_pretypetag().unwrap().value);
        assert_eq!(
            sp(PreTypeTag_::Vector(Box::new(sp(PreTypeTag_::U8)))),
            "Vec<u8>".to_pretypetag().unwrap()
        );
        assert_eq!(
            sp(PreTypeTag_::Struct(StructTag {
                address: AccountAddress::new(Address::DIEM_CORE.to_u8()),
                module: Identifier::new("Block").unwrap(),
                name: Identifier::new("T").unwrap(),
                type_args: vec![sp(PreTypeTag_::Struct(StructTag {
                    address: AccountAddress::new(Address::DIEM_CORE.to_u8()),
                    module: Identifier::new("Block").unwrap(),
                    name: Identifier::new("T").unwrap(),
                    type_args: Vec::new()
                }))]
            })),
            "0x1::Block::T<Block::T>".to_pretypetag().unwrap()
        );

        assert_eq!(
            sp(PreTypeTag_::Alias(AliasTag {
                name: Identifier::new("T").unwrap(),
                type_args: vec![sp(PreTypeTag_::U128)]
            })),
            "T<u128>".to_pretypetag().unwrap(),
        );
    }

    #[test]
    fn test_deny_ref() {
        use crate::parser::parse;

        assert!("T<&T>".to_pretypetag().is_err());
        assert!("0x1::Block::T<&Block::T>".to_pretypetag().is_err());
        assert!(parse("{ test<0x1::Block::T<Block::T>, T<T, &u8>>(); }", "dsl",).is_err());
    }

    #[test]
    fn test_to_typetag() {
        assert_eq!(TypeTag::Bool, "bool".to_typetag().unwrap());
        assert_eq!(TypeTag::U8, "u8".to_typetag().unwrap());
        assert_eq!(TypeTag::U64, "u64".to_typetag().unwrap());
        assert_eq!(TypeTag::U128, "u128".to_typetag().unwrap());
        assert_eq!(TypeTag::Address, "address".to_typetag().unwrap());
        assert_eq!(TypeTag::Signer, "signer".to_typetag().unwrap());
        assert_eq!(
            TypeTag::Vector(Box::new(TypeTag::U64)),
            "Vec<u64>".to_typetag().unwrap()
        );

        assert_eq!(
            TypeTag::Struct(TStructTag {
                address: AccountAddress::new(Address::DIEM_CORE.to_u8()),
                module: Identifier::new("Block").unwrap(),
                name: Identifier::new("T").unwrap(),
                type_params: vec![TypeTag::Struct(TStructTag {
                    address: AccountAddress::new(Address::DIEM_CORE.to_u8()),
                    module: Identifier::new("Block").unwrap(),
                    name: Identifier::new("T").unwrap(),
                    type_params: Vec::new()
                })]
            }),
            "0x1::Block::T<Block::T>".to_typetag().unwrap()
        );

        assert!("T<u128>".to_typetag().is_err());
    }

    #[test]
    fn test_vec_to_typetag() {
        assert_eq!(
            vec![TypeTag::U8, TypeTag::U64, TypeTag::U128],
            vec!["u8", "u64", "u128"].to_typetag().unwrap()
        );
    }
}
