use move_core_types::language_storage::TypeTag;
use anyhow::Error;
use move_core_types::account_address::AccountAddress;
use move_core_types::identifier::Identifier;
use move_lang::parser::lexer::{Lexer, Tok};
use move_lang::parser::syntax::{parse_type, parse_address_bytes, consume_token};
use lang::lexer::unwrap_spanned_ty;
use std::str::FromStr;
use move_symbol_pool::Symbol;

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
pub(crate) fn parse_call(call: &str) -> Result<Call, Error> {
    let mut lexer = Lexer::new(&call, Symbol::from("call"));

    // Get the name of the function|script
    let error_message = "Invalid call format: expected function identifier.\n\n\
         Use pattern:\n\
         SCRIPT_FUNCTION_NAME<TYPE1, TYPE2, ...>(PARAM1, PARAM2, ...)\n\
         or\n\
         ACCOUNT_ADDRESS::MODULE_NAME::FUNCTION_NAME<TYPE1, TYPE2, ...>(PARAM1, PARAM2, ...)";

    lexer
        .advance()
        .map_err(|err| anyhow!("{}\n\n{:?}", error_message, err))?;

    let address = if lexer.peek() == Tok::NumValue {
        let address = AccountAddress::new(
            parse_address_bytes(&mut lexer)
                .map_err(|_| anyhow!("{}\n\nInvalid account address.", error_message))?
                .value
                .into_bytes(),
        );
        consume_token(&mut lexer, Tok::ColonColon).map_err(|_| {
            anyhow!(
                "{}\n\n A double colon was expected after the module address.",
                error_message
            )
        })?;
        Some(address)
    } else {
        None
    };

    if lexer.peek() != Tok::IdentifierValue {
        anyhow::bail!(
            "{}\n\nA script name or module module was expected.",
            error_message
        );
    }
    let name = lexer.content().to_owned();
    lexer
        .advance()
        .map_err(|err| anyhow!("{}\n\n{:?}", error_message, err))?;

    let mut call = if lexer.peek() == Tok::ColonColon {
        lexer
            .advance()
            .map_err(|err| anyhow!("{}\n\n{:?}", error_message, err))?;
        if lexer.peek() != Tok::IdentifierValue {
            anyhow::bail!(
                "{}\n\nA script name or module module was expected.",
                error_message
            );
        }
        let func = lexer.content().to_owned();
        lexer
            .advance()
            .map_err(|err| anyhow!("{}\n\n{:?}", error_message, err))?;
        Call::Function {
            address,
            module: Identifier::new(name)?,
            func: Identifier::new(func)?,
            type_tag: vec![],
            args: vec![],
        }
    } else {
        Call::Script {
            name: Identifier::new(name)?,
            type_tag: vec![],
            args: vec![],
        }
    };

    call.set_tp_params(parse_type_params(&mut lexer)?);
    call.set_args(parse_args(&mut lexer)?);

    Ok(call)
}

fn parse_type_params(lexer: &mut Lexer) -> Result<Vec<TypeTag>, Error> {
    let error_message = "Invalid call script format: Invalid type parameters format.\n\n\
         Use pattern:\n\
         SCRIPT_FUNCTION_NAME<TYPE1, TYPE2, ...>(PARAM1, PARAM2, ...)\
         or\n\
         ACCOUNT_ADDRESS::MODULE_NAME::FUNCTION_NAME<TYPE1, TYPE2, ...>(PARAM1, PARAM2, ...)";
    let map_err = |_| anyhow!("{}", &error_message);

    if lexer.peek() == Tok::Less {
        let mut type_parameter = vec![];

        lexer.advance().map_err(map_err)?;
        while lexer.peek() != Tok::Greater {
            if lexer.peek() == Tok::EOF {
                anyhow::bail!(error_message);
            }

            if lexer.peek() == Tok::Comma {
                lexer
                    .advance()
                    .map_err(|err| anyhow!("{}\n\n{}", &error_message, err[0].1))?;
                continue;
            }

            let type_str = lexer.content().to_string();
            type_parameter.push(
                parse_type_param(lexer)
                    .map_err(|_| anyhow!("{}\n\nUnknown: {}", &error_message, type_str))?,
            );
        }
        lexer
            .advance()
            .map_err(|err| anyhow!("{}\n\n{:?}", &error_message, err))?;
        Ok(type_parameter)
    } else {
        Ok(vec![])
    }
}

fn parse_args(lexer: &mut Lexer) -> Result<Vec<String>, Error> {
    let error_message = "Invalid call script format: Invalid script arguments format.\n\n\
         Use pattern:\n\
         SCRIPT_FUNCTION_NAME<TYPE1, TYPE2, ...>(PARAM1, PARAM2, ...)\
         or\n\
         ACCOUNT_ADDRESS::MODULE_NAME::FUNCTION_NAME<TYPE1, TYPE2, ...>(PARAM1, PARAM2, ...)";
    let map_err = |_| anyhow!("{}", &error_message);

    if lexer.peek() == Tok::LParen {
        let mut arguments = vec![];
        lexer.advance().map_err(map_err)?;
        while lexer.peek() != Tok::RParen {
            if lexer.peek() == Tok::EOF {
                anyhow::bail!("{}", &error_message);
            }

            if lexer.peek() == Tok::Comma {
                lexer
                    .advance()
                    .map_err(|err| anyhow!("{}\n\n{}", &error_message, err[0].1))?;
                continue;
            }

            let mut token = String::new();
            token.push_str(lexer.content());
            let sw = lexer.peek() == Tok::LBracket;
            lexer.advance().map_err(map_err)?;
            if sw {
                while lexer.peek() != Tok::RBracket {
                    token.push_str(lexer.content());
                    lexer.advance().map_err(|_| anyhow!("{}", &error_message))?;
                }
                token.push_str(lexer.content());
            } else {
                while lexer.peek() != Tok::Comma && lexer.peek() != Tok::RParen {
                    token.push_str(lexer.content());
                    lexer.advance().map_err(map_err)?;
                }
            }
            arguments.push(token);
            if !sw && lexer.peek() == Tok::RParen {
                break;
            }
            lexer.advance().map_err(map_err)?;
        }
        Ok(arguments)
    } else {
        Ok(vec![])
    }
}

pub(crate) fn parse_tp_param(tp: &str) -> Result<TypeTag, Error> {
    let mut lexer = Lexer::new(tp, "tp");
    lexer
        .advance()
        .map_err(|err| Error::msg(format!("{:?}", err)))?;
    parse_type_param(&mut lexer)
}

/// parse type params
///
/// u8 => TypeTag::U8
/// u64 => TypeTag::U64
/// ...
pub(crate) fn parse_type_param(lexer: &mut Lexer) -> Result<TypeTag, Error> {
    let ty = parse_type(lexer).map_err(|err| Error::msg(format!("{:?}", err)))?;
    unwrap_spanned_ty(ty)
}

pub(crate) fn parse_vec<E>(tkn: &str, tp_name: &str) -> Result<Vec<E>, Error>
where
    E: FromStr,
{
    let map_err = |err| Error::msg(format!("{:?}", err));

    let mut lexer = Lexer::new(tkn, "vec");
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
    use move_core_types::identifier::Identifier;
    use move_core_types::language_storage::{StructTag, TypeTag};
    use move_core_types::language_storage::CORE_CODE_ADDRESS;
    use crate::tx::parser::{parse_call};
    use lang::compiler::dialects::DialectName;

    #[test]
    fn func_call() {
        let dialect = DialectName::Pont.get_dialect();
        let sender = "0x1";

        let (address, name, func, type_tag, args) =
            parse_call(dialect.as_ref(), sender, "Account::create_account")
                .unwrap()
                .func();
        assert_eq!(address, None);
        assert_eq!(name.as_str(), "Account");
        assert_eq!(func.as_str(), "create_account");
        assert!(type_tag.is_empty());
        assert!(args.is_empty());

        let (address, name, func, type_tag, args) =
            parse_call(dialect.as_ref(), sender, "0x1::Account::create_account")
                .unwrap()
                .func();
        assert_eq!(address, Some(CORE_CODE_ADDRESS));
        assert_eq!(name.as_str(), "Account");
        assert_eq!(func.as_str(), "create_account");
        assert!(type_tag.is_empty());
        assert!(args.is_empty());

        let (address, name, func, type_tag, args) = parse_call(
            dialect.as_ref(),
            sender,
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
            dialect.as_ref(),
            sender,
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
            parse_call(dialect.as_ref(), sender, "Account::create_account<u8>()")
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
        let dialect = DialectName::Pont.get_dialect();
        let sender = "0x1";

        let (name, type_tag, args) = parse_call(dialect.as_ref(), sender, "create_account<u8, 0x01::Dfinance::USD<u8>>(10, 68656c6c6f, [10, 23], true, 1exaAg2VJRQbyUBAeXcktChCAqjVP9TUxF3zo23R2T6EGdE)").unwrap().script();
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
                "0x1CF326C5AAA5AF9F0E2791E66310FE8F044FAADAF12567EAA0976959D1F7731F".to_owned(),
            ]
        );

        let (name, tp, args) = parse_call(
            dialect.as_ref(),
            sender,
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

        let (name, tp, args) = parse_call(dialect.as_ref(), sender, "create_account()")
            .unwrap()
            .script();
        assert_eq!(name.as_str(), "create_account");
        assert_eq!(tp, Vec::<TypeTag>::new());
        assert_eq!(args, Vec::<String>::new());

        let (name, tp, args) = parse_call(dialect.as_ref(), sender, "create_account<>()")
            .unwrap()
            .script();
        assert_eq!(name.as_str(), "create_account");
        assert_eq!(tp, Vec::<TypeTag>::new());
        assert_eq!(args, Vec::<String>::new());
    }
}
