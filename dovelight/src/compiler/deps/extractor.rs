use move_core_types::language_storage::{ModuleId, CORE_CODE_ADDRESS};
use std::collections::{HashSet, BTreeMap, HashMap};
use move_lang::parser::ast::{
    Definition, Script, ModuleDefinition, LeadingNameAccess, LeadingNameAccess_, ModuleMember,
    Function, StructDefinition, UseDecl, FriendDecl, Constant, SpecBlock, Type, Type_,
    NameAccessChain_, Use, FunctionBody_, Exp, SequenceItem, StructFields, NameAccessChain, Exp_,
    Sequence, Bind_, SequenceItem_, BindList, Bind,
};
use move_core_types::account_address::AccountAddress;
use move_core_types::identifier::Identifier;
use anyhow::Error;
use move_lang::parser::syntax::parse_file_string;
use move_lang::errors::report_errors_to_color_buffer;

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
                    let addr = address
                        .addr_value
                        .map(|a| AccountAddress::new(a.value.into_bytes()))
                        .unwrap_or_else(|| match &address.addr.value {
                            LeadingNameAccess_::AnonymousAddress(a) => {
                                AccountAddress::new(a.into_bytes())
                            }
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

    pub fn finish(mut self) -> HashSet<ModuleId> {
        for source in &self.sources {
            self.imports.remove(source);
        }
        self.imports
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
        let address = module
            .address
            .as_ref()
            .and_then(|a| match a.value {
                LeadingNameAccess_::AnonymousAddress(a) => {
                    Some(AccountAddress::new(a.into_bytes()))
                }
                LeadingNameAccess_::Name(_) => None,
            })
            .or_else(|| address)
            .unwrap_or_else(|| CORE_CODE_ADDRESS);

        self.sources.insert(ModuleId::new(
            address,
            Identifier::new(module.name.0.value.as_str()).unwrap(),
        ));

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

    fn extract_use_imports(&mut self, def: &UseDecl) {
        match &def.use_ {
            Use::Module(ident, _) | Use::Members(ident, _) => {
                self.add_import(ident.value.module.0.value.as_str(), &ident.value.address);
            }
        }
    }

    fn extract_function_imports(&mut self, func: &Function) {
        for (_, tp) in &func.signature.parameters {
            self.extract_type(tp);
        }
        self.extract_type(&func.signature.return_type);
        if let FunctionBody_::Defined(seq) = &func.body.value {
            self.extract_seq(seq);
        }
    }

    fn extract_seq(&mut self, (use_decl, items, _, exp): &Sequence) {
        for use_ in use_decl {
            self.extract_use_imports(use_);
        }

        for item in items {
            self.extract_seq_item(item);
        }

        if let Some(exp) = exp.as_ref() {
            self.extract_exp(exp);
        }
    }

    fn extract_struct_imports(&mut self, def: &StructDefinition) {
        if let StructFields::Defined(fields) = &def.fields {
            for (_, tp) in fields {
                self.extract_type(tp);
            }
        }
    }

    fn extract_friend_imports(&mut self, def: &FriendDecl) {
        self.extract_name_access_chain(&def.friend);
    }

    fn extract_const_imports(&mut self, def: &Constant) {
        self.extract_type(&def.signature);
        self.extract_exp(&def.value);
    }

    fn extract_spec_imports(&mut self, _: &SpecBlock) {
        //no-op
    }

    fn extract_types(&mut self, types: &[Type]) {
        for tp in types {
            self.extract_type(tp);
        }
    }

    fn extract_exp(&mut self, exp: &Exp) {
        match &exp.value {
            Exp_::Value(_)
            | Exp_::Move(_)
            | Exp_::Copy(_)
            | Exp_::UnresolvedError
            | Exp_::Spec(_)
            | Exp_::Unit
            | Exp_::Break
            | Exp_::Continue => {
                //no-op
            }
            Exp_::Name(access, types) => {
                self.extract_name_access_chain(access);
                if let Some(types) = types {
                    self.extract_types(types);
                }
            }
            Exp_::Call(access, types, exp) => {
                self.extract_name_access_chain(access);
                if let Some(types) = types {
                    self.extract_types(types);
                }
                for exp in &exp.value {
                    self.extract_exp(exp);
                }
            }
            Exp_::Pack(access, types, exp) => {
                self.extract_name_access_chain(access);
                if let Some(types) = types {
                    self.extract_types(types);
                }
                for (_, exp) in exp {
                    self.extract_exp(exp);
                }
            }
            Exp_::IfElse(exp_1, exp_2, exp_3) => {
                self.extract_exp(exp_1);
                self.extract_exp(exp_2);
                if let Some(exp) = exp_3 {
                    self.extract_exp(exp);
                }
            }
            Exp_::Block(seq) => {
                self.extract_seq(&seq);
            }
            Exp_::Lambda(bind, exp) => {
                self.extract_bind_list(bind);
                self.extract_exp(exp);
            }
            Exp_::Quant(_, range_list, exp, exp_1, exp_2) => {
                for range in &range_list.value {
                    let (bind, exp) = &range.value;
                    self.extract_bind(bind);
                    self.extract_exp(exp);
                }
                for list in exp {
                    for exp in list {
                        self.extract_exp(exp);
                    }
                }
                if let Some(exp) = exp_1 {
                    self.extract_exp(exp);
                }
                self.extract_exp(exp_2);
            }
            Exp_::ExpList(list) => {
                for exp in list {
                    self.extract_exp(exp);
                }
            }
            Exp_::Assign(exp, exp_1)
            | Exp_::BinopExp(exp, _, exp_1)
            | Exp_::Index(exp, exp_1)
            | Exp_::While(exp, exp_1) => {
                self.extract_exp(exp);
                self.extract_exp(exp_1);
            }
            Exp_::Return(ret) => {
                if let Some(exp) = ret {
                    self.extract_exp(exp);
                }
            }
            Exp_::Abort(exp)
            | Exp_::Dereference(exp)
            | Exp_::UnaryExp(_, exp)
            | Exp_::Loop(exp)
            | Exp_::Borrow(_, exp)
            | Exp_::Dot(exp, _) => {
                self.extract_exp(exp);
            }
            Exp_::Cast(exp, tp) | Exp_::Annotate(exp, tp) => {
                self.extract_exp(exp);
                self.extract_type(tp);
            }
        }
    }

    fn extract_seq_item(&mut self, item: &SequenceItem) {
        match &item.value {
            SequenceItem_::Seq(exp) => {
                self.extract_exp(&exp);
            }
            SequenceItem_::Declare(bind_list, tp) => {
                self.extract_bind_list(bind_list);
                if let Some(tp) = tp {
                    self.extract_type(tp);
                }
            }
            SequenceItem_::Bind(bind_list, tp, exp) => {
                self.extract_bind_list(bind_list);
                if let Some(tp) = tp {
                    self.extract_type(tp);
                }
                self.extract_exp(exp);
            }
        }
    }

    fn extract_bind_list(&mut self, bind_list: &BindList) {
        for bind in &bind_list.value {
            self.extract_bind(bind);
        }
    }

    fn extract_bind(&mut self, bind: &Bind) {
        match &bind.value {
            Bind_::Var(_) => {
                // no-op
            }
            Bind_::Unpack(access, types, binds) => {
                self.extract_name_access_chain(access);
                if let Some(types) = types {
                    self.extract_types(types);
                }
                for bind in binds {
                    self.extract_bind(&bind.1);
                }
            }
        }
    }

    fn extract_type(&mut self, tp: &Type) {
        match &tp.value {
            Type_::Apply(access, types) => {
                for tp in types {
                    self.extract_type(tp);
                }
                self.extract_name_access_chain(access.as_ref());
            }
            Type_::Ref(_, tp) => {
                self.extract_type(tp.as_ref());
            }
            Type_::Fun(_, tp) => {
                self.extract_type(tp.as_ref());
            }
            Type_::Unit => {
                //no-op
            }
            Type_::Multiple(types) => {
                for tp in types {
                    self.extract_type(tp);
                }
            }
        }
    }

    fn extract_name_access_chain(&mut self, access: &NameAccessChain) {
        match &access.value {
            NameAccessChain_::One(_) => {
                //no-op
            }
            NameAccessChain_::Two(addr, name) => {
                self.add_import(name.value.as_str(), addr);
            }
            NameAccessChain_::Three(access, _) => {
                let (addr, name) = &access.value;
                self.add_import(name.value.as_str(), addr);
            }
        }
    }

    fn add_import(&mut self, name: &str, addr: &LeadingNameAccess) {
        if let Ok(name) = Identifier::new(name) {
            match &addr.value {
                LeadingNameAccess_::AnonymousAddress(addr) => {
                    let addr = AccountAddress::new(addr.into_bytes());
                    self.imports.insert(ModuleId::new(addr, name));
                }
                LeadingNameAccess_::Name(_) => {
                    //no-op
                }
            }
        }
    }
}

pub fn extract_source_deps(source: &str) -> Result<HashSet<ModuleId>, Error> {
    let mut extractor = ImportsExtractor::default();
    let def = parse_file_string("source", &source, BTreeMap::new());
    match def {
        Ok((def, _)) => {
            extractor.extract(&def);
        }
        Err(errors) => {
            let mut files = HashMap::new();
            files.insert("source", source.to_string());
            let error = report_errors_to_color_buffer(files, errors);
            let err = String::from_utf8_lossy(&error).to_string();
            return Err(Error::msg(err));
        }
    };
    Ok(extractor.finish())
}

#[cfg(test)]
mod tests {
    use move_lang::parser::syntax::parse_file_string;
    use std::collections::{BTreeMap, HashSet};
    use crate::compiler::deps::extractor::ImportsExtractor;
    use move_core_types::language_storage::ModuleId;
    use move_core_types::account_address::AccountAddress;
    use move_core_types::identifier::Identifier;

    #[test]
    pub fn test_extract_imports() {
        let mut extractor = ImportsExtractor::default();
        let source = "\
            script {
                use 0x1::Dove;
                use 0x1::_;
                use 0x1::Dove as Dove1;
                use 0x1::This;

                fun main(d: 0x2::Foo, f: u8) {
                    let t: 0x3::Doo;
                }
            }

            address 0x6 {
                module This1 {
                    use 0x10::Foo;
                    friend 0x42::A::A;
                    friend 0x43::A::A;

                    #[test_only]
                    struct F {
                    }
                }
            }

            module 0x4::This {
            }
        ";
        let def = parse_file_string("source.move", source, BTreeMap::new())
            .unwrap()
            .0;
        extractor.extract(&def);

        let source = "\
            module 0x43::A {
            }
        ";
        let def = parse_file_string("source.move", source, BTreeMap::new())
            .unwrap()
            .0;
        extractor.extract(&def);

        let mut expected = HashSet::new();
        expected.insert(id("0x1", "This"));
        expected.insert(id("0x42", "A"));
        expected.insert(id("0x3", "Doo"));
        expected.insert(id("0x1", "Dove"));
        expected.insert(id("0x2", "Foo"));
        expected.insert(id("0x10", "Foo"));

        assert_eq!(extractor.finish(), expected);
    }

    fn id(addr: &str, name: &str) -> ModuleId {
        ModuleId::new(
            AccountAddress::from_hex_literal(addr).unwrap(),
            Identifier::new(name).unwrap(),
        )
    }
}
