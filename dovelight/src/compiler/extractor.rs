use move_core_types::language_storage::{ModuleId, CORE_CODE_ADDRESS};
use std::collections::HashSet;
use move_lang::parser::ast::{Definition, Script, ModuleDefinition, LeadingNameAccess, LeadingNameAccess_, ModuleMember, Function, StructDefinition, UseDecl, FriendDecl, Constant, SpecBlock};
use move_core_types::account_address::AccountAddress;
use move_core_types::ident_str;
use move_core_types::identifier::Identifier;

#[derive(Debug, Default)]
pub struct ImportsExtractor {
    sources: HashSet<ModuleId>,
    imports: HashSet<ModuleId>,
}

impl ImportsExtractor {
    pub fn extract(&mut self, defs: &[Definition]) {
        for def in defs {
            match def {
                Definition::Module(module) => {
                    self.extract_module(None, module);
                }
                Definition::Address(address) => {
                    let addr = address.addr_value.map(|a| AccountAddress::new(a.value.into_bytes()))
                        .unwrap_or_else(|| match &address.addr.value {
                            LeadingNameAccess_::AnonymousAddress(a) => AccountAddress::new(a.into_bytes()),
                            LeadingNameAccess_::Name(_) => CORE_CODE_ADDRESS,
                        });
                    for module in &address.modules {
                        self.extract_module(Some(addr), module);
                    }
                }
                Definition::Script(script) => {
                    self.extract_script(script);
                }
            }
        }
    }

    fn extract_script(&mut self, script: &Script) {
        for def in &script.uses {
            self.extract_use_imports(def);
        }
        for def in &script.constants {
            self.extract_const_imports(def);
        }
        self.extract_function_imports(&script.function);
        for def in &script.specs {
            self.extract_spec_imports(def);
        }
    }

    fn extract_module(&mut self, address: Option<AccountAddress>, module: &ModuleDefinition) {
        let address = module.address.as_ref().and_then(|a| match a.value {
            LeadingNameAccess_::AnonymousAddress(a) => Some(AccountAddress::new(a.into_bytes())),
            LeadingNameAccess_::Name(_) => None,
        }).or_else(|| address)
            .unwrap_or_else(|| CORE_CODE_ADDRESS);

        self.sources.insert(ModuleId::new(address, Identifier::new(module.name.0.value.as_str()).unwrap()));

        for member in &module.members {
            match member {
                ModuleMember::Function(def) => self.extract_function_imports(def),
                ModuleMember::Struct(def) => self.extract_struct_imports(def),
                ModuleMember::Use(def) => self.extract_use_imports(def),
                ModuleMember::Friend(def) => self.extract_friend_imports(def),
                ModuleMember::Constant(def) => self.extract_const_imports(def),
                ModuleMember::Spec(def) => self.extract_spec_imports(def),
            }
        }
    }

    fn extract_function_imports(&mut self, func: &Function) {}
    fn extract_struct_imports(&mut self, def: &StructDefinition) {}
    fn extract_use_imports(&mut self, def: &UseDecl) {}
    fn extract_friend_imports(&mut self, def: &FriendDecl) {}
    fn extract_const_imports(&mut self, def: &Constant) {}
    fn extract_spec_imports(&mut self, def: &SpecBlock) {}
}