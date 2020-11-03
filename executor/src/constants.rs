use move_lang::parser::ast::{Constant, Exp_, Value_, ModuleDefinition, ModuleMember, Definition};
use move_lang::shared::{Address, Identifier};
use crate::session::ConstsMap;
use move_lang::parser::ast;

fn extract_integer_constant_value(constant: &Constant) -> Option<u128> {
    match &constant.value.value {
        Exp_::Value(val) => match val.value {
            Value_::U8(num) => Some(num as u128),
            Value_::U64(num) => Some(num as u128),
            Value_::U128(num) => Some(num as u128),
            _ => None,
        },
        Exp_::InferredNum(val) => Some(val.to_owned()),
        _ => None,
    }
}

fn constants(module: &ModuleDefinition) -> impl Iterator<Item = &Constant> {
    module.members.iter().filter_map(|m| match m {
        ModuleMember::Constant(c) => Some(c),
        _ => None,
    })
}

fn extract_constants_from_module(
    address: &Address,
    module: &ModuleDefinition,
    consts: &mut ConstsMap,
) {
    let address = format!("{}", address);
    let module_name = module.name.value();
    for constant in constants(module) {
        let const_name = constant.name.value();
        if const_name.starts_with("ERR_") {
            if let Some(val) = extract_integer_constant_value(constant) {
                consts.insert(
                    (address.clone(), module_name.to_owned(), val),
                    const_name.to_owned(),
                );
            }
        }
    }
}

pub fn extract_error_constants(program: &ast::Program, consts: &mut ConstsMap) {
    let definitions = program
        .source_definitions
        .iter()
        .chain(program.lib_definitions.iter());
    for definition in definitions {
        if let Definition::Address(_, address, modules) = definition {
            for module in modules {
                extract_constants_from_module(address, module, consts);
            }
        }
    }
}
