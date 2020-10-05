use anyhow::Result;
use std::path::PathBuf;

use libra::{prelude::*, compiler::*};

use std::collections::HashSet;
use termcolor::{StandardStream, ColorChoice};
use std::process::exit;
use crate::builder::convert_path;

/// Extract dependencies from source code.
pub fn extract_dependencies_from_source(
    targets: &[PathBuf],
    address: Option<AccountAddress>,
    print_err: bool,
    shutdown_on_err: bool,
) -> Result<HashSet<ModuleId>> {
    let mut extractor = DefinitionUses::with_address(address);
    let (files, pprog_and_comments_res) = parse_program(&convert_path(targets)?, &[])?;
    match pprog_and_comments_res {
        Ok((program, _)) => {
            for def in program.source_definitions {
                extractor.extract(&def)?;
            }
        }
        Err(errs) => {
            if print_err {
                let mut writer = StandardStream::stderr(ColorChoice::Auto);
                errors::output_errors(&mut writer, files, errs);
            }
            if shutdown_on_err {
                exit(1);
            }
        }
    }

    Ok(extractor.imports())
}

/// Extract dependencies from bytecode.
pub fn extract_dependencies_from_bytecode(bytecode: &[u8]) -> Result<HashSet<ModuleId>> {
    let mut extractor = BytecodeUses::default();
    extractor.extract(
        CompiledModule::deserialize(bytecode)
            .map_err(|e| e.finish(Location::Undefined).into_vm_status())?,
    )?;
    Ok(extractor.imports())
}

/// Source definition dependencies extractor.
#[derive(Default)]
pub struct DefinitionUses {
    imports: HashSet<ModuleId>,
    modules: HashSet<ModuleId>,
    address: Option<AccountAddress>,
}

impl DefinitionUses {
    /// Creates extractor with account address.
    pub fn with_address(address: Option<AccountAddress>) -> DefinitionUses {
        DefinitionUses {
            imports: Default::default(),
            modules: Default::default(),
            address,
        }
    }

    /// Extracts dependencies from definition.
    pub fn extract(&mut self, def: &Definition) -> Result<()> {
        match def {
            Definition::Module(module) => self.module(
                module,
                self.address
                    .ok_or_else(|| anyhow!("Expected account address."))?,
            )?,
            Definition::Address(_, addr, modules) => {
                let addr = AccountAddress::new(addr.to_u8());
                for module in modules {
                    self.module(module, addr)?;
                }
            }
            Definition::Script(script) => self.script(script)?,
        }
        Ok(())
    }

    /// Extracts dependencies from module definition.
    fn module(&mut self, module: &ModuleDefinition, address: AccountAddress) -> Result<()> {
        for member in &module.members {
            match member {
                ModuleMember::Use(_use) => self.uses(_use)?,
                ModuleMember::Function(func) => self.function(func)?,
                ModuleMember::Struct(_struct) => {
                    match &_struct.fields {
                        StructFields::Defined(types) => {
                            for (_, t) in types {
                                self.s_type_usages(&t.value)?;
                            }
                        }
                        StructFields::Native(_) => {
                            //No-op
                        }
                    }
                }
                ModuleMember::Spec(_) => {
                    // no-op
                }
                ModuleMember::Constant(constant) => {
                    self.constant(constant)?;
                }
            }
        }
        self.modules.insert(ModuleId::new(
            address,
            Identifier::new(module.name.0.value.to_owned())?,
        ));

        Ok(())
    }

    /// Extracts dependencies from script.
    fn script(&mut self, script: &Script) -> Result<()> {
        for u in &script.uses {
            self.uses(u)?;
        }
        self.function(&script.function)
    }

    /// Extracts dependencies from constant.
    fn constant(&mut self, constant: &Constant) -> Result<()> {
        self.type_usages(&constant.signature.value)?;
        self.expresion_usages(&constant.value.value)
    }

    /// Extracts dependencies from use definition.
    fn uses(&mut self, u: &Use) -> Result<()> {
        let ident = match u {
            Use::Members(ident, _) => ident,
            Use::Module(ident, _) => ident,
        };

        let ident = &ident.0.value;
        let name = Identifier::new(ident.name.0.value.to_owned())?;
        let address = AccountAddress::new(ident.address.clone().to_u8());
        self.imports.insert(ModuleId::new(address, name));
        Ok(())
    }

    /// Extracts dependencies from function definition.
    fn function(&mut self, func: &Function) -> Result<()> {
        self.signature(&func.signature)?;
        self.internal_usages(&func.body)
    }

    /// Extracts dependencies from function signature.
    fn signature(&mut self, signature: &FunctionSignature) -> Result<()> {
        for (_, v_type) in &signature.parameters {
            self.type_usages(&v_type.value)?;
        }
        self.type_usages(&signature.return_type.value)
    }

    /// Extracts dependencies from function body.
    fn internal_usages(&mut self, func: &FunctionBody) -> Result<()> {
        match &func.value {
            FunctionBody_::Defined((uses, seq, _, exp)) => {
                for u in uses {
                    self.uses(u)?;
                }

                self.block_usages(seq)?;
                if let Some(exp) = exp.as_ref() {
                    self.expresion_usages(&exp.value)?;
                }
            }
            FunctionBody_::Native => {
                // No-op
            }
        }
        Ok(())
    }

    /// Extracts dependencies from type.
    fn type_usages(&mut self, v_type: &Type_) -> Result<()> {
        match v_type {
            Type_::Unit => { /*No-op*/ }
            Type_::Multiple(s_types) => {
                for s_type in s_types {
                    self.s_type_usages(&s_type.value)?;
                }
            }
            Type_::Apply(access, s_types) => {
                for s_type in s_types {
                    self.s_type_usages(&s_type.value)?;
                }
                self.access_usages(&access.value)?;
            }
            Type_::Ref(_, s_type) => {
                self.s_type_usages(&s_type.value)?;
            }
            Type_::Fun(s_types, s_type) => {
                self.s_type_usages(&s_type.value)?;
                for s_type in s_types {
                    self.s_type_usages(&s_type.value)?;
                }
            }
        }
        Ok(())
    }

    /// Extracts dependencies from block.
    fn block_usages(&mut self, seq: &[SequenceItem]) -> Result<()> {
        for item in seq {
            match &item.value {
                SequenceItem_::Seq(exp) => self.expresion_usages(&exp.value)?,
                SequenceItem_::Declare(bind_list, s_type) => {
                    for bind in &bind_list.value {
                        self.bind_usages(&bind.value)?;
                    }
                    if let Some(s_type) = s_type {
                        self.type_usages(&s_type.value)?;
                    }
                }
                SequenceItem_::Bind(bind_list, s_type, exp) => {
                    for bind in &bind_list.value {
                        self.bind_usages(&bind.value)?;
                    }

                    if let Some(s_type) = s_type {
                        self.type_usages(&s_type.value)?;
                    }

                    self.expresion_usages(&exp.value)?;
                }
            }
        }
        Ok(())
    }

    /// Extracts dependencies from bind statement.
    fn bind_usages(&mut self, bind: &Bind_) -> Result<()> {
        match bind {
            Bind_::Var(_) => { /*no-op*/ }
            Bind_::Unpack(access, s_types, binds) => {
                self.access_usages(&access.value)?;
                if let Some(s_types) = s_types {
                    for s_type in s_types {
                        self.s_type_usages(&s_type.value)?;
                    }
                    for bind in binds {
                        self.bind_usages(&bind.1.value)?;
                    }
                }
            }
        }
        Ok(())
    }

    /// Extracts dependencies from module access.
    fn access_usages(&mut self, access: &ModuleAccess_) -> Result<()> {
        match access {
            ModuleAccess_::QualifiedModuleAccess(ident, _name) => {
                let ident = &ident.0.value;
                self.imports.insert(ModuleId::new(
                    AccountAddress::new(ident.address.clone().to_u8()),
                    Identifier::new(ident.name.0.value.to_owned())?,
                ));
            }
            ModuleAccess_::ModuleAccess(_, _) | ModuleAccess_::Name(_) => { /*no-op*/ }
        }
        Ok(())
    }

    /// Extracts dependencies from type.
    fn s_type_usages(&mut self, s_type: &Type_) -> Result<()> {
        match s_type {
            Type_::Apply(module_access, s_types) => {
                self.access_usages(&module_access.value)?;
                for s_type in s_types {
                    self.s_type_usages(&s_type.value)?;
                }
            }
            Type_::Ref(_, s_type) => {
                self.s_type_usages(&s_type.value)?;
            }
            Type_::Fun(s_types, s_type) => {
                for s_type in s_types {
                    self.s_type_usages(&s_type.value)?;
                }
                self.s_type_usages(&s_type.value)?;
            }
            Type_::Unit => {}
            Type_::Multiple(s_types) => {
                for s_type in s_types {
                    self.s_type_usages(&s_type.value)?;
                }
            }
        }
        Ok(())
    }

    /// Extracts dependencies from expression.
    fn expresion_usages(&mut self, exp: &Exp_) -> Result<()> {
        match exp {
            Exp_::Value(_)
            | Exp_::Move(_)
            | Exp_::Copy(_)
            | Exp_::Unit
            | Exp_::Break
            | Exp_::Continue
            | Exp_::Lambda(_, _)
            | Exp_::Spec(_)
            | Exp_::Index(_, _)
            | Exp_::InferredNum(_)
            | Exp_::UnresolvedError => { /*no op*/ }
            Exp_::Call(access, s_types, exp_list) => {
                self.access_usages(&access.value)?;

                if let Some(s_types) = s_types {
                    for s_type in s_types {
                        self.s_type_usages(&s_type.value)?;
                    }
                }

                for exp in &exp_list.value {
                    self.expresion_usages(&exp.value)?;
                }
            }
            Exp_::Pack(access, s_types, exp_list) => {
                self.access_usages(&access.value)?;

                if let Some(s_types) = s_types {
                    for s_type in s_types {
                        self.s_type_usages(&s_type.value)?;
                    }
                }

                for (_, exp) in exp_list {
                    self.expresion_usages(&exp.value)?;
                }
            }
            Exp_::IfElse(eb, et, ef) => {
                self.expresion_usages(&eb.value)?;
                self.expresion_usages(&et.value)?;
                if let Some(ef) = ef {
                    self.expresion_usages(&ef.value)?;
                }
            }
            Exp_::While(eb, eloop) => {
                self.expresion_usages(&eb.value)?;
                self.expresion_usages(&eloop.value)?;
            }
            Exp_::Block((uses, seq, _, exp)) => {
                for u in uses {
                    self.uses(u)?;
                }

                self.block_usages(seq)?;
                if let Some(exp) = exp.as_ref() {
                    self.expresion_usages(&exp.value)?;
                }
            }
            Exp_::ExpList(exp_list) => {
                for exp in exp_list {
                    self.expresion_usages(&exp.value)?;
                }
            }
            Exp_::Assign(a, e) => {
                self.expresion_usages(&a.value)?;
                self.expresion_usages(&e.value)?;
            }
            Exp_::Abort(e)
            | Exp_::Dereference(e)
            | Exp_::Loop(e)
            | Exp_::UnaryExp(_, e)
            | Exp_::Borrow(_, e)
            | Exp_::Dot(e, _)
            | Exp_::Annotate(e, _) => {
                self.expresion_usages(&e.value)?;
            }
            Exp_::Return(e) => {
                if let Some(e) = e {
                    self.expresion_usages(&e.value)?;
                }
            }
            Exp_::BinopExp(e1, _, e2) => {
                self.expresion_usages(&e1.value)?;
                self.expresion_usages(&e2.value)?;
            }
            Exp_::Name(access, s_types) => {
                self.access_usages(&access.value)?;
                if let Some(s_types) = s_types {
                    for s_type in s_types {
                        self.s_type_usages(&s_type.value)?;
                    }
                }
            }
            Exp_::Cast(e1, s_type) => {
                self.expresion_usages(&e1.value)?;
                self.s_type_usages(&s_type.value)?;
            }
        }
        Ok(())
    }

    /// Returns imports.
    pub fn imports(mut self) -> HashSet<ModuleId> {
        for module_id in self.modules {
            self.imports.remove(&module_id);
        }

        self.imports
    }
}

/// Bytecode dependencies extractor.
#[derive(Default)]
pub struct BytecodeUses {
    imports: HashSet<ModuleId>,
}

impl BytecodeUses {
    /// Returns imports.
    pub fn imports(self) -> HashSet<ModuleId> {
        self.imports
    }

    /// Extracts dependencies from compiled module.
    pub fn extract(&mut self, module: CompiledModule) -> Result<()> {
        let module = module.into_inner();
        let mut module_handles = module.module_handles;
        if !module_handles.is_empty() {
            // Remove self module with 0 index.
            module_handles.remove(0);
        }

        for module_handle in module_handles {
            let name = module.identifiers[module_handle.name.0 as usize]
                .as_str()
                .to_owned();
            let address = module.address_identifiers[module_handle.address.0 as usize];
            self.imports
                .insert(ModuleId::new(address, Identifier::new(name)?));
        }

        Ok(())
    }
}
