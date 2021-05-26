use anyhow::{Result, Error};
use std::path::Path;
use lang::compiler::parser::parse_file;
use move_lang::{errors, parser::ast::*, name_pool::ConstPool};

use std::collections::{HashSet, HashMap};
use termcolor::{StandardStream, ColorChoice};
use lang::compiler::dialects::Dialect;
use std::fs;
use rand::random;
use std::rc::Rc;
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::{ModuleId, CORE_CODE_ADDRESS};
use move_core_types::identifier::Identifier;
use vm::CompiledModule;
use vm::errors::Location;

/// Extracts metadata form source code.
pub fn source_meta(
    file: &Path,
    sender: Option<AccountAddress>,
    dialect: &dyn Dialect,
) -> Result<FileMeta, Error> {
    let name = ConstPool::push(file.to_str().unwrap_or("source"));
    let source = fs::read_to_string(file)?;

    let (defs, _, errors, _) =
        parse_file(dialect, &mut HashMap::default(), name, &source, sender);
    if errors.is_empty() {
        let mut metadata = Vec::new();
        for def in defs {
            for meta in DefinitionMeta::from_definition(def, sender)
                .map_err(|err| anyhow!("{} Path:{:?}", err, file))?
            {
                metadata.push(meta);
            }
        }

        let path = Rc::from(
            file.to_str()
                .ok_or_else(|| anyhow!("Failed to convert source path:{:?}", file))?,
        );

        Ok(FileMeta {
            path,
            meta: metadata,
        })
    } else {
        let mut files = HashMap::new();
        files.insert(name, source);
        let mut writer = StandardStream::stderr(ColorChoice::Auto);
        errors::output_errors(&mut writer, files, errors);
        Err(anyhow!("Failed to parse move file:{}", name))
    }
}

/// Move definition metadata.
pub struct DefinitionMeta {
    /// Module identifier.
    pub module_id: ModuleId,
    /// Module imports.
    pub imports: HashSet<Rc<ModuleId>>,
}

/// Move file metadata.
pub struct FileMeta {
    /// File path.
    pub path: Rc<str>,
    /// Modules metadata.
    pub meta: Vec<DefinitionMeta>,
}

impl DefinitionMeta {
    fn from_definition(
        def: Definition,
        address: Option<AccountAddress>,
    ) -> Result<Vec<DefinitionMeta>> {
        match def {
            Definition::Module(module) => {
                let addr = address.ok_or_else(|| {
                    anyhow!("Address not defined for module {}", module.name.0.value)
                })?;
                Ok(vec![DefinitionMeta::module(&module, addr)?])
            }
            Definition::Address(_, address, modules) => {
                let addr = AccountAddress::new(address.to_u8());
                modules
                    .iter()
                    .map(|m| DefinitionMeta::module(m, addr))
                    .collect()
            }
            Definition::Script(script) => Ok(vec![DefinitionMeta::script(
                &script,
                address.unwrap_or(CORE_CODE_ADDRESS),
            )?]),
        }
    }

    fn module(module: &ModuleDefinition, address: AccountAddress) -> Result<DefinitionMeta> {
        let mut meta = DefinitionMeta {
            module_id: ModuleId::new(address, Identifier::new(module.name.0.value.to_owned())?),
            imports: Default::default(),
        };

        for member in &module.members {
            match member {
                ModuleMember::Use(_use) => meta.extract_uses(_use)?,
                ModuleMember::Function(func) => meta.function(func)?,
                ModuleMember::Struct(_struct) => {
                    match &_struct.fields {
                        StructFields::Defined(types) => {
                            for (_, t) in types {
                                meta.s_type_usages(&t.value)?;
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
                    meta.constant(constant)?;
                }
                ModuleMember::Friend(friend) => {
                    meta.friend(friend)?;
                }
            }
        }

        Ok(meta)
    }

    /// Extracts dependencies from use definition.
    fn extract_uses(&mut self, u: &Use) -> Result<()> {
        let ident = match u {
            Use::Members(ident, _) => ident,
            Use::Module(ident, _) => ident,
        };

        let (address, name) = &ident.value;
        let name = Identifier::new(name.to_owned())?;
        let address = AccountAddress::new(address.to_u8());
        self.imports.insert(Rc::new(ModuleId::new(address, name)));
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
                self.ident(ident)?;
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
            Exp_::Quant(_, _, _, _, _) => {}
        }
        Ok(())
    }

    /// Extracts dependencies from constant.
    fn constant(&mut self, constant: &Constant) -> Result<()> {
        self.type_usages(&constant.signature.value)?;
        self.expresion_usages(&constant.value.value)
    }

    fn friend(&mut self, friend: &Friend) -> Result<()> {
        let friend = &friend.value;
        match friend {
            Friend_::Module(_) => { /*no-op*/ }
            Friend_::QualifiedModule(ident) => {
                self.ident(ident)?;
            }
        }
        Ok(())
    }

    /// Extracts dependencies from use definition.
    fn uses(&mut self, u: &Use) -> Result<()> {
        let ident = match u {
            Use::Members(ident, _) => ident,
            Use::Module(ident, _) => ident,
        };
        self.ident(ident)
    }

    fn ident(&mut self, ident: &ModuleIdent) -> Result<()> {
        let (address, name) = &ident.value;
        let name = Identifier::new(name.to_owned())?;
        let address = AccountAddress::new(address.to_u8());
        self.imports.insert(Rc::new(ModuleId::new(address, name)));
        Ok(())
    }

    fn script(script: &Script, address: AccountAddress) -> Result<DefinitionMeta> {
        let mut meta = DefinitionMeta {
            module_id: ModuleId::new(
                address,
                Identifier::new(format!(
                    "scripts_{}_{}",
                    script.function.name.0.value.to_owned(),
                    random::<u32>()
                ))?,
            ),
            imports: Default::default(),
        };

        for u in &script.uses {
            meta.uses(u)?;
        }
        meta.function(&script.function)?;
        Ok(meta)
    }
}

/// Extract dependencies from bytecode.
pub fn extract_bytecode_dependencies(bytecode: &[u8]) -> Result<HashSet<ModuleId>> {
    let mut extractor = BytecodeUses::default();
    extractor.extract(
        CompiledModule::deserialize(bytecode)
            .map_err(|e| e.finish(Location::Undefined).into_vm_status())?,
    )?;
    Ok(extractor.imports())
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
