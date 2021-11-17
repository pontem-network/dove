use move_core_types::language_storage::{CORE_CODE_ADDRESS, TypeTag};
use anyhow::Error;
use move_core_types::account_address::AccountAddress;
use move_core_types::identifier::Identifier;
use move_lang::parser::lexer::{Lexer, Tok};
use move_lang::parser::syntax::{parse_type, parse_address_bytes, consume_token, Context};
use lang::lexer::unwrap_spanned_ty;
use std::str::FromStr;
use move_lang::Flags;
use move_lang::shared::CompilationEnv;
use move_package::source_package::parsed_manifest::AddressDeclarations;
use move_symbol_pool::Symbol;

const ERROR_MESSAGE: &str = "Invalid call format: expected function identifier.\n\n\
         Use pattern:\n\
         SCRIPT_FUNCTION_NAME<TYPE1, TYPE2, ...>(PARAM1, PARAM2, ...)\n\
         or\n\
         ACCOUNT_ADDRESS::MODULE_NAME::FUNCTION_NAME<TYPE1, TYPE2, ...>(PARAM1, PARAM2, ...)";


/// Call.
#[derive(Debug)]
pub enum Call {
    /// Function call declaration.
    Function {
        /// Module address.
        address: Option<AccountAddress>,
        /// Module name.
        module: Identifier,
        /// Function name.
        func: Identifier,
        /// Function type parameter.
        type_tag: Vec<TypeTag>,
        /// Function args.
        args: Vec<String>,
    },
    /// Script call declaration.
    Script {
        /// Script name.
        name: Identifier,
        /// Function type parameter.
        type_tag: Vec<TypeTag>,
        /// Function args.
        args: Vec<String>,
    },
}

impl Call {
    pub(crate) fn set_args(&mut self, new_args: Vec<String>) {
        match self {
            Call::Function { args, .. } => {
                *args = new_args;
            }
            Call::Script { args, .. } => {
                *args = new_args;
            }
        }
    }

    pub(crate) fn set_tp_params(&mut self, new_tags: Vec<TypeTag>) {
        match self {
            Call::Function { type_tag, .. } => {
                *type_tag = new_tags;
            }
            Call::Script { type_tag, .. } => {
                *type_tag = new_tags;
            }
        }
    }

    #[cfg(test)]
    pub fn script(self) -> (Identifier, Vec<TypeTag>, Vec<String>) {
        if let Call::Script {
            name,
            type_tag,
            args,
        } = self
        {
            (name, type_tag, args)
        } else {
            panic!("Script is expected")
        }
    }

    #[cfg(test)]
    pub fn func(
        self,
    ) -> (
        Option<AccountAddress>,
        Identifier,
        Identifier,
        Vec<TypeTag>,
        Vec<String>,
    ) {
        if let Call::Function {
            address,
            module,
            func,
            type_tag,
            args,
        } = self
        {
            (address, module, func, type_tag, args)
        } else {
            panic!("Function is expected")
        }
    }
}

/// Parse call
/// Return: Ok(Script name, Type parameters, Function arguments) or Error
pub(crate) fn parse_call(addr_map: &AddressDeclarations, call: &str) -> Result<Call, Error> {
    let mut lexer = Lexer::new(&call, Symbol::from("call"));
    let mut env = CompilationEnv::new(Flags::empty(), Default::default());
    let mut ctx = Context::new(&mut env, &mut lexer);

    ctx.tokens
        .advance()
        .map_err(|err| anyhow!("{}\n\n{:?}", ERROR_MESSAGE, err))?;

    let mut call = parse_call_body(addr_map, &mut ctx)?;
    call.set_tp_params(parse_type_params(&mut ctx)?);
    call.set_args(parse_args(&mut ctx)?);
    Ok(call)
}

fn parse_call_body(addr_map: &AddressDeclarations, ctx: &mut Context) -> Result<Call, Error> {
    let address = if ctx.tokens.peek() == Tok::NumValue {
        let address = AccountAddress::new(
            parse_address_bytes(ctx)
                .map_err(|_| anyhow!("{}\n\nInvalid account address.", ERROR_MESSAGE))?
                .value
                .into_bytes(),
        );
        consume_token(&mut ctx.tokens, Tok::ColonColon).map_err(|_| {
            anyhow!(
                    "{}\n\n A double colon was expected after the module address.",
                    ERROR_MESSAGE
                )
        })?;
        Some(address)
    } else {
        None
    };
    let mut tokens = vec![];

    loop {
        if ctx.tokens.peek() != Tok::IdentifierValue {
            break;
        }

        tokens.push(ctx.tokens.content().to_string());
        ctx.tokens
            .advance()
            .map_err(|err| anyhow!("{}\n\n{:?}", ERROR_MESSAGE, err))?;
        if ctx.tokens.peek() == Tok::ColonColon {
            ctx.tokens
                .advance()
                .map_err(|err| anyhow!("{}\n\n{:?}", ERROR_MESSAGE, err))?;
        }
    }

    Ok(match tokens.len() {
        1 => {
            if let Some(addr) = address {
                return Err(Error::msg(ERROR_MESSAGE));
            }
            Call::Script {
                name: Identifier::new(tokens.remove(0))?,
                type_tag: vec![],
                args: vec![],
            }
        }
        2 => {
            Call::Function {
                address,
                module: Identifier::new(tokens.remove(0))?,
                func: Identifier::new(tokens.remove(0))?,
                type_tag: vec![],
                args: vec![],
            }
        }
        3 => {
            if let Some(addr) = address {
                return Err(Error::msg(ERROR_MESSAGE));
            }

            let named_addr = tokens.remove(0);
            let addr = addr_map
                .get(&Symbol::from(named_addr.as_str()))
                .and_then(|a| a.clone())
                .ok_or_else(|| anyhow!("Address {} not found.", named_addr))?;

            Call::Function {
                address: Some(addr),
                module: Identifier::new(tokens.remove(0))?,
                func: Identifier::new(tokens.remove(0))?,
                type_tag: vec![],
                args: vec![],
            }
        }
        _ => {
            return Err(Error::msg(ERROR_MESSAGE));
        }
    })
}

pub enum CallBody {
    Script(String),
    Module(String, String),
    ModuleWithAddress(AccountAddress, String, String),
}

fn parse_type_params(ctx: &mut Context) -> Result<Vec<TypeTag>, Error> {
    let error_message = "Invalid call script format: Invalid type parameters format.\n\n\
         Use pattern:\n\
         SCRIPT_FUNCTION_NAME<TYPE1, TYPE2, ...>(PARAM1, PARAM2, ...)\
         or\n\
         ACCOUNT_ADDRESS::MODULE_NAME::FUNCTION_NAME<TYPE1, TYPE2, ...>(PARAM1, PARAM2, ...)";
    let map_err = |_| anyhow!("{}", &error_message);

    if ctx.tokens.peek() == Tok::Less {
        let mut type_parameter = vec![];

        ctx.tokens.advance().map_err(map_err)?;
        while ctx.tokens.peek() != Tok::Greater {
            if ctx.tokens.peek() == Tok::EOF {
                anyhow::bail!(error_message);
            }

            if ctx.tokens.peek() == Tok::Comma {
                ctx.tokens
                    .advance()
                    .map_err(|err| anyhow!("{}\n\n{:?}", &error_message, err))?;
                continue;
            }

            let type_str = ctx.tokens.content().to_string();
            type_parameter.push(
                parse_type_param(ctx)
                    .map_err(|_| anyhow!("{}\n\nUnknown: {}", &error_message, type_str))?,
            );
        }
        ctx.tokens
            .advance()
            .map_err(|err| anyhow!("{}\n\n{:?}", &error_message, err))?;
        Ok(type_parameter)
    } else {
        Ok(vec![])
    }
}

fn parse_args(ctx: &mut Context) -> Result<Vec<String>, Error> {
    let error_message = "Invalid call script format: Invalid script arguments format.\n\n\
         Use pattern:\n\
         SCRIPT_FUNCTION_NAME<TYPE1, TYPE2, ...>(PARAM1, PARAM2, ...)\
         or\n\
         ACCOUNT_ADDRESS::MODULE_NAME::FUNCTION_NAME<TYPE1, TYPE2, ...>(PARAM1, PARAM2, ...)";
    let map_err = |_| anyhow!("{}", &error_message);

    if ctx.tokens.peek() == Tok::LParen {
        let mut arguments = vec![];
        ctx.tokens.advance().map_err(map_err)?;
        while ctx.tokens.peek() != Tok::RParen {
            if ctx.tokens.peek() == Tok::EOF {
                anyhow::bail!("{}", &error_message);
            }

            if ctx.tokens.peek() == Tok::Comma {
                ctx.tokens
                    .advance()
                    .map_err(|err| anyhow!("{}\n\n{:?}", &error_message, err))?;
                continue;
            }

            let mut token = String::new();
            token.push_str(ctx.tokens.content());
            let sw = ctx.tokens.peek() == Tok::LBracket;
            ctx.tokens.advance().map_err(map_err)?;
            if sw {
                while ctx.tokens.peek() != Tok::RBracket {
                    token.push_str(ctx.tokens.content());
                    ctx.tokens
                        .advance()
                        .map_err(|_| anyhow!("{}", &error_message))?;
                }
                token.push_str(ctx.tokens.content());
            } else {
                while ctx.tokens.peek() != Tok::Comma && ctx.tokens.peek() != Tok::RParen {
                    token.push_str(ctx.tokens.content());
                    ctx.tokens.advance().map_err(map_err)?;
                }
            }
            arguments.push(token);
            if !sw && ctx.tokens.peek() == Tok::RParen {
                break;
            }
            ctx.tokens.advance().map_err(map_err)?;
        }
        Ok(arguments)
    } else {
        Ok(vec![])
    }
}

pub(crate) fn parse_tp_param(tp: &str) -> Result<TypeTag, Error> {
    let mut lexer = Lexer::new(tp, Symbol::from("tp"));
    let mut env = CompilationEnv::new(Flags::empty(), Default::default());
    let mut ctx = Context::new(&mut env, &mut lexer);

    ctx.tokens
        .advance()
        .map_err(|err| Error::msg(format!("{:?}", err)))?;
    parse_type_param(&mut ctx)
}

/// parse type params
///
/// u8 => TypeTag::U8
/// u64 => TypeTag::U64
/// ...
pub(crate) fn parse_type_param(ctx: &mut Context) -> Result<TypeTag, Error> {
    let ty = parse_type(ctx).map_err(|err| Error::msg(format!("{:?}", err)))?;
    unwrap_spanned_ty(ty)
}

pub(crate) fn parse_vec<E>(tkn: &str, tp_name: &str) -> Result<Vec<E>, Error>
    where
        E: FromStr,
{
    let map_err = |err| Error::msg(format!("{:?}", err));

    let mut lexer = Lexer::new(tkn, Symbol::from("vec"));

    lexer.advance().map_err(map_err)?;

    if lexer.peek() != Tok::LBracket {
        anyhow::bail!("Vector in format  [n1, n2, ..., nn] is expected.");
    }
    lexer.advance().map_err(map_err)?;

    let mut elements = vec![];
    while lexer.peek() != Tok::RBracket {
        match lexer.peek() {
            Tok::Comma => {
                lexer.advance().map_err(map_err)?;
                continue;
            }
            Tok::EOF => {
                anyhow::bail!("unexpected end of vector.");
            }
            _ => {
                elements.push(E::from_str(lexer.content()).map_err(|_| {
                    anyhow!(
                        "Failed to parse vector element. {} type is expected. Actual:'{}'",
                        tp_name,
                        lexer.content()
                    )
                })?);
                lexer.advance().map_err(map_err)?;
            }
        }
    }
    Ok(elements)
}

#[cfg(test)]
mod tests_call_parser {
    use std::collections::BTreeMap;
    use std::str::FromStr;
    use move_core_types::account_address::AccountAddress;
    use move_core_types::identifier::Identifier;
    use move_core_types::language_storage::{StructTag, TypeTag};
    use move_core_types::language_storage::CORE_CODE_ADDRESS;
    use move_symbol_pool::Symbol;
    use crate::tx::parser::parse_call;

    #[test]
    fn func_call() {
        let (address, name, func, type_tag, args) =
            parse_call(&Default::default(), "Account::create_account")
                .unwrap()
                .func();
        assert_eq!(address, None);
        assert_eq!(name.as_str(), "Account");
        assert_eq!(func.as_str(), "create_account");
        assert!(type_tag.is_empty());
        assert!(args.is_empty());

        let (address, name, func, type_tag, args) =
            parse_call(&Default::default(), "0x1::Account::create_account")
                .unwrap()
                .func();
        assert_eq!(address, Some(CORE_CODE_ADDRESS));
        assert_eq!(name.as_str(), "Account");
        assert_eq!(func.as_str(), "create_account");
        assert!(type_tag.is_empty());
        assert!(args.is_empty());

        let (address, name, func, type_tag, args) = parse_call(
            &Default::default(),
            "0x1::Account::create_account<u8, 0x01::Dfinance::USD>",
        )
            .unwrap()
            .func();
        assert_eq!(address, Some(CORE_CODE_ADDRESS));
        assert_eq!(name.as_str(), "Account");
        assert_eq!(func.as_str(), "create_account");
        assert_eq!(
            type_tag,
            vec![
                TypeTag::U8,
                TypeTag::Struct(StructTag {
                    address: CORE_CODE_ADDRESS,
                    module: Identifier::new("Dfinance").unwrap(),
                    name: Identifier::new("USD").unwrap(),
                    type_params: vec![],
                }),
            ]
        );
        assert!(args.is_empty());

        let (address, name, func, type_tag, args) = parse_call(
            &Default::default(),
            "0x1::Account::create_account(10, 68656c6c6f, [10, 23],)",
        )
            .unwrap()
            .func();
        assert_eq!(address, Some(CORE_CODE_ADDRESS));
        assert_eq!(name.as_str(), "Account");
        assert_eq!(func.as_str(), "create_account");
        assert!(type_tag.is_empty());
        assert_eq!(
            args,
            vec![
                "10".to_owned(),
                "68656c6c6f".to_owned(),
                "[10,23]".to_owned(),
            ]
        );

        let (address, name, func, type_tag, args) =
            parse_call(&Default::default(), "Account::create_account<u8>()")
                .unwrap()
                .func();
        assert_eq!(address, None);
        assert_eq!(name.as_str(), "Account");
        assert_eq!(func.as_str(), "create_account");
        assert_eq!(type_tag, vec![TypeTag::U8]);
        assert!(args.is_empty());
    }

    #[test]
    fn script_call() {
        let (name, type_tag, args) = parse_call(
            &Default::default(),
            "create_account<u8, 0x01::Dfinance::USD<u8>>(10, 68656c6c6f, [10, 23], true, Std)",
        )
            .unwrap()
            .script();
        assert_eq!(name.as_str(), "create_account");
        assert_eq!(
            type_tag,
            vec![
                TypeTag::U8,
                TypeTag::Struct(StructTag {
                    address: CORE_CODE_ADDRESS,
                    module: Identifier::new("Dfinance").unwrap(),
                    name: Identifier::new("USD").unwrap(),
                    type_params: vec![TypeTag::U8],
                }),
            ]
        );
        assert_eq!(
            args,
            vec![
                "10".to_owned(),
                "68656c6c6f".to_owned(),
                "[10,23]".to_owned(),
                "true".to_owned(),
                "Std".to_owned(),
            ]
        );

        let (name, tp, args) = parse_call(
            &Default::default(),
            "create_account<0x01::Dfinance::USD>([true, false], [0x01, 0x02])",
        )
            .unwrap()
            .script();
        assert_eq!(name.as_str(), "create_account");
        assert_eq!(
            tp,
            vec![TypeTag::Struct(StructTag {
                address: CORE_CODE_ADDRESS,
                module: Identifier::new("Dfinance").unwrap(),
                name: Identifier::new("USD").unwrap(),
                type_params: vec![],
            })]
        );
        assert_eq!(
            args,
            vec!["[true,false]".to_owned(), "[0x01,0x02]".to_owned()]
        );

        let (name, tp, args) = parse_call(&Default::default(), "create_account()")
            .unwrap()
            .script();
        assert_eq!(name.as_str(), "create_account");
        assert_eq!(tp, Vec::<TypeTag>::new());
        assert_eq!(args, Vec::<String>::new());

        let (name, tp, args) = parse_call(&Default::default(), "create_account<>()")
            .unwrap()
            .script();
        assert_eq!(name.as_str(), "create_account");
        assert_eq!(tp, Vec::<TypeTag>::new());
        assert_eq!(args, Vec::<String>::new());
    }

    #[test]
    fn named_address() {
        let mut map = BTreeMap::new();
        map.insert(Symbol::from("Core"), Some(AccountAddress::from_hex_literal("0x13").unwrap()));
        let (address, name, func, type_tag, args) = parse_call(
            &map,
            "Core::Diem::create_account(Std)",
        ).unwrap()
            .func();
        assert_eq!(address, Some(AccountAddress::from_hex_literal("0x13").unwrap()));
        assert_eq!(name.as_str(), "Diem");
        assert_eq!(func.as_str(), "create_account");
        assert!(type_tag.is_empty());
        assert_eq!(args, vec!["Std".to_string()]);
    }

    #[test]
    #[should_panic]
    fn named_address_not_found() {
        let mut map = BTreeMap::new();
        map.insert(Symbol::from("CoRe"), Some(AccountAddress::from_hex_literal("0x13").unwrap()));
        parse_call(&map, "Core::Diem::create_account(Std)").unwrap();
    }
}
