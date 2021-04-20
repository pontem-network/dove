use vm::file_format::*;
use anyhow::Error;
use crate::Encode;
use crate::script::Script as ScriptAst;
use crate::module::Module as ModuleAst;
use std::fmt::{Write, Debug};
use vm::errors::Location;
use move_core_types::language_storage::{ModuleId, CORE_CODE_ADDRESS};
use move_core_types::identifier::Identifier;
use vm::access::{ModuleAccess, ScriptAccess};
use move_core_types::account_address::AccountAddress;

/// Undefined bytecode abstraction.
#[derive(Debug)]
pub enum CompiledUnit {
    /// Compiled script.
    Script(CompiledScript),
    /// Compiled module.
    Module(CompiledModule),
}

impl CompiledUnit {
    /// Create a new CompiledUnit with the given bytecode.
    pub fn new(bytecode: &[u8]) -> Result<CompiledUnit, Error> {
        CompiledScript::deserialize(bytecode)
            .map_err(|err| err.finish(Location::Undefined).into_vm_status().into())
            .and_then(|s| {
                if CompiledUnit::check_is_script(&s) {
                    Ok(CompiledUnit::Script(s))
                } else {
                    CompiledUnit::load_as_module(bytecode)
                }
            })
            .or_else(|_| CompiledUnit::load_as_module(bytecode))
    }

    fn check_is_script(s: &CompiledScript) -> bool {
        !s.as_inner().code.code.is_empty()
    }

    fn load_as_module(bytecode: &[u8]) -> Result<CompiledUnit, Error> {
        Ok(CompiledUnit::Module(
            CompiledModule::deserialize(bytecode)
                .map_err(|err| err.finish(Location::Undefined).into_vm_status())?,
        ))
    }
}

/// Undefined bytecode accessor.
pub trait UnitAccess: Debug {
    /// Returns true if the bytecode is script bytecode.
    fn is_script(&self) -> bool;

    /// Returns script-specific data.
    fn script_info(&self) -> Option<(&CodeUnit, &Vec<AbilitySet>, SignatureIndex)>;

    /// Returns unit id.
    fn self_id(&self) -> ModuleId;

    /// Returns modules handlers.
    fn module_handles(&self) -> &[ModuleHandle];

    /// Returns module handle by its index.
    fn module_handle(&self, idx: ModuleHandleIndex) -> &ModuleHandle;

    /// Returns identifiers.
    fn identifiers(&self) -> &[Identifier];

    /// Returns identifier by its index.
    fn identifier(&self, index: IdentifierIndex) -> &str;

    /// Returns account address by its index.
    fn address(&self, index: AddressIdentifierIndex) -> &AccountAddress;

    /// Returns self module handle
    fn self_module_handle_idx(&self) -> Option<ModuleHandleIndex>;

    /// Returns functions definition.
    fn function_defs(&self) -> &[FunctionDefinition];

    /// Returns function definition by its index.
    fn function_handle(&self, idx: FunctionHandleIndex) -> &FunctionHandle;

    /// Returns function instruction by its index.
    fn function_instantiation(&self, idx: FunctionInstantiationIndex) -> &FunctionInstantiation;

    /// Returns signature by its index.
    fn signature(&self, idx: SignatureIndex) -> &Signature;

    /// Returns structures definition.
    fn struct_defs(&self) -> &[StructDefinition];

    /// Returns struct definition by its index.
    fn struct_def(&self, idx: StructDefinitionIndex) -> Option<&StructDefinition>;

    /// Returns struct handle by its index.
    fn struct_handle(&self, idx: StructHandleIndex) -> &StructHandle;

    /// Returns struct definition instruction by its index.
    fn struct_def_instantiation(
        &self,
        idx: StructDefInstantiationIndex,
    ) -> Option<&StructDefInstantiation>;

    /// Returns field instruction by its index.
    fn field_instantiation(&self, idx: FieldInstantiationIndex) -> Option<&FieldInstantiation>;

    /// Returns constant by its index.
    fn constant(&self, idx: ConstantPoolIndex) -> &Constant;

    /// Returns field handle by its index.
    fn field_handle(&self, idx: FieldHandleIndex) -> Option<&FieldHandle>;
}

impl UnitAccess for CompiledUnit {
    fn is_script(&self) -> bool {
        match self {
            CompiledUnit::Script(_) => true,
            CompiledUnit::Module(_) => false,
        }
    }

    fn script_info(&self) -> Option<(&CodeUnit, &Vec<AbilitySet>, SignatureIndex)> {
        match self {
            CompiledUnit::Script(s) => Some((
                s.code(),
                &s.as_inner().type_parameters,
                s.as_inner().parameters,
            )),
            CompiledUnit::Module(_) => None,
        }
    }

    fn self_id(&self) -> ModuleId {
        match self {
            CompiledUnit::Script(_) => ModuleId::new(
                CORE_CODE_ADDRESS,
                Identifier::new("<SELF>").expect("Valid name."),
            ),
            CompiledUnit::Module(m) => m.self_id(),
        }
    }

    fn module_handles(&self) -> &[ModuleHandle] {
        match self {
            CompiledUnit::Script(s) => s.module_handles(),
            CompiledUnit::Module(m) => m.module_handles(),
        }
    }

    fn module_handle(&self, idx: ModuleHandleIndex) -> &ModuleHandle {
        match self {
            CompiledUnit::Script(s) => s.module_handle_at(idx),
            CompiledUnit::Module(m) => m.module_handle_at(idx),
        }
    }

    fn identifiers(&self) -> &[Identifier] {
        match self {
            CompiledUnit::Script(s) => s.identifiers(),
            CompiledUnit::Module(m) => m.identifiers(),
        }
    }

    fn identifier(&self, index: IdentifierIndex) -> &str {
        match self {
            CompiledUnit::Script(s) => s.identifier_at(index).as_str(),
            CompiledUnit::Module(m) => m.identifier_at(index).as_str(),
        }
    }

    fn address(&self, index: AddressIdentifierIndex) -> &AccountAddress {
        match self {
            CompiledUnit::Script(s) => s.address_identifier_at(index),
            CompiledUnit::Module(m) => m.address_identifier_at(index),
        }
    }

    fn self_module_handle_idx(&self) -> Option<ModuleHandleIndex> {
        match self {
            CompiledUnit::Script(_) => None,
            CompiledUnit::Module(m) => Some(m.self_handle_idx()),
        }
    }

    fn function_defs(&self) -> &[FunctionDefinition] {
        match self {
            CompiledUnit::Script(_) => &[],
            CompiledUnit::Module(m) => &m.as_inner().function_defs,
        }
    }

    fn function_handle(&self, idx: FunctionHandleIndex) -> &FunctionHandle {
        match self {
            CompiledUnit::Script(s) => s.function_handle_at(idx),
            CompiledUnit::Module(m) => m.function_handle_at(idx),
        }
    }

    fn function_instantiation(&self, idx: FunctionInstantiationIndex) -> &FunctionInstantiation {
        match self {
            CompiledUnit::Script(s) => s.function_instantiation_at(idx),
            CompiledUnit::Module(m) => m.function_instantiation_at(idx),
        }
    }

    fn signature(&self, idx: SignatureIndex) -> &Signature {
        match self {
            CompiledUnit::Script(s) => s.signature_at(idx),
            CompiledUnit::Module(m) => m.signature_at(idx),
        }
    }

    fn struct_defs(&self) -> &[StructDefinition] {
        match self {
            CompiledUnit::Script(_) => &[],
            CompiledUnit::Module(m) => &m.as_inner().struct_defs,
        }
    }

    fn struct_def(&self, idx: StructDefinitionIndex) -> Option<&StructDefinition> {
        match self {
            CompiledUnit::Script(_) => None,
            CompiledUnit::Module(m) => Some(m.struct_def_at(idx)),
        }
    }

    fn struct_handle(&self, idx: StructHandleIndex) -> &StructHandle {
        match self {
            CompiledUnit::Script(s) => s.struct_handle_at(idx),
            CompiledUnit::Module(m) => m.struct_handle_at(idx),
        }
    }

    fn struct_def_instantiation(
        &self,
        idx: StructDefInstantiationIndex,
    ) -> Option<&StructDefInstantiation> {
        match self {
            CompiledUnit::Script(_) => None,
            CompiledUnit::Module(m) => Some(m.struct_instantiation_at(idx)),
        }
    }

    fn field_instantiation(&self, idx: FieldInstantiationIndex) -> Option<&FieldInstantiation> {
        match self {
            CompiledUnit::Script(_) => None,
            CompiledUnit::Module(m) => Some(m.field_instantiation_at(idx)),
        }
    }

    fn constant(&self, idx: ConstantPoolIndex) -> &Constant {
        match self {
            CompiledUnit::Script(s) => &s.constant_at(idx),
            CompiledUnit::Module(m) => &m.constant_at(idx),
        }
    }

    fn field_handle(&self, idx: FieldHandleIndex) -> Option<&FieldHandle> {
        match self {
            CompiledUnit::Script(_) => None,
            CompiledUnit::Module(m) => Some(m.field_handle_at(idx)),
        }
    }
}

/// Restored move ast.
pub enum SourceUnit<'a> {
    /// Script ast.
    Script(ScriptAst<'a>),
    /// Module ast.
    Module(ModuleAst<'a>),
}

impl<'a> SourceUnit<'a> {
    /// Writes source code to the given writer.
    pub fn write_code<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        match self {
            SourceUnit::Script(script) => script.encode(writer, 0),
            SourceUnit::Module(module) => module.encode(writer, 0),
        }
    }

    /// Returns source code.
    pub fn code_string(&self) -> Result<String, Error> {
        let mut code = String::new();
        self.write_code(&mut code)?;
        Ok(code)
    }
}
