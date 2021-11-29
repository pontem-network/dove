use anyhow::Error;
use move_core_types::account_address::AccountAddress;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::{StructTag, TypeTag};
use move_lang::parser::ast::{LeadingNameAccess_, NameAccessChain_, Type, Type_};

pub fn unwrap_spanned_ty(ty: Type) -> Result<TypeTag, Error> {
    fn unwrap_spanned_ty_(ty: Type, this: Option<AccountAddress>) -> Result<TypeTag, Error> {
        let st = match ty.value {
            Type_::Apply(ma, mut ty_params) => {
                match (ma.value, this) {
                    // N
                    (NameAccessChain_::One(name), this) => match name.value.as_ref() {
                        "bool" => TypeTag::Bool,
                        "u8" => TypeTag::U8,
                        "u64" => TypeTag::U64,
                        "u128" => TypeTag::U128,
                        "address" => TypeTag::Address,
                        "signer" => TypeTag::Signer,
                        "Vec" if ty_params.len() == 1 => TypeTag::Vector(
                            unwrap_spanned_ty_(ty_params.pop().unwrap(), this)
                                .unwrap()
                                .into(),
                        ),
                        _ => bail!(
                            "Could not parse input: type without struct name & module address"
                        ),
                    },
                    (NameAccessChain_::Two(_, _), None) => {
                        bail!("Could not parse input: type without module address");
                    }
                    (NameAccessChain_::Three(access, name), this) => {
                        let (addr, m_name) = access.value;
                        let address = match addr.value {
                            LeadingNameAccess_::AnonymousAddress(addr) => AccountAddress::new(addr.into_bytes()),
                            LeadingNameAccess_::Name(name) => {
                                this.ok_or_else(|| anyhow!("Could not parse input: unsupported named address. Name '{}'.", name))?
                            }
                        };
                        TypeTag::Struct(StructTag {
                            address,
                            module: Identifier::new(m_name.value.as_str())?,
                            name: Identifier::new(name.value.as_str())?,
                            type_params: ty_params
                                .into_iter()
                                .map(|ty| unwrap_spanned_ty_(ty, Some(address)))
                                .map(|res| match res {
                                    Ok(st) => st,
                                    Err(err) => panic!("{:?}", err),
                                })
                                .collect(),
                        })
                    }
                    (NameAccessChain_::Two(access, name), Some(address)) => {
                        let m_name = match access.value {
                            LeadingNameAccess_::AnonymousAddress(_) => {
                                bail!("Could not parse input: type without module name");
                            }
                            LeadingNameAccess_::Name(name) => name,
                        };

                        TypeTag::Struct(StructTag {
                            address,
                            module: Identifier::new(m_name.value.as_str())?,
                            name: Identifier::new(name.value.as_str())?,
                            type_params: ty_params
                                .into_iter()
                                .map(|ty| unwrap_spanned_ty_(ty, Some(address)))
                                .map(|res| match res {
                                    Ok(st) => st,
                                    Err(err) => panic!("{:?}", err),
                                })
                                .collect(),
                        })
                    }
                }
            }
            _ => {
                bail!("Could not parse input: unsupported type");
            }
        };

        Ok(st)
    }

    unwrap_spanned_ty_(ty, None)
}

#[cfg(test)]
mod tests {
    use move_lang::Flags;
    use move_lang::parser::lexer::Lexer;
    use move_lang::parser::syntax::{Context, parse_type};
    use move_lang::shared::CompilationEnv;
    use move_symbol_pool::Symbol;

    use super::*;

    fn parse(source: &str) -> Result<TypeTag, Error> {
        let mut lexer = Lexer::new(source, Symbol::from("source"));
        lexer
            .advance()
            .map_err(|err| anyhow!("Query parsing error:\n\t{:?}", err))?;
        let mut env = CompilationEnv::new(Flags::empty(), Default::default());
        let mut context = Context::new(&mut env, &mut lexer);
        let ty = parse_type(&mut context)
            .map_err(|err| anyhow!("Query parsing error:\n\t{:?}", err))?;
        unwrap_spanned_ty(ty)
    }

    #[test]
    fn test_parse() {
        assert!(parse("0x1::Foo").is_err());

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
            .map(|inp| (inp, parse(inp)))
            .for_each(|(inp, res)| {
                assert!(res.is_ok(), "failed on '{}'", inp);
                println!("{:?}", res.unwrap());
            });
    }
}
