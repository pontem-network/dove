use crate::code::exp::{Exp, ExpLoc, SourceRange, find_range};
use crate::code::translator::Context;
use crate::types::FType;
use crate::imports::Import;
use vm::file_format::{FunctionHandleIndex, SignatureIndex, StructDefinitionIndex};
use crate::{Encode, write_array};
use anyhow::Error;
use std::fmt::Write;
use crate::unit::UnitAccess;

/// Function call representation.
#[derive(Debug)]
pub enum FnCall<'a> {
    /// Call build-in function.
    BuildIn {
        /// Build-in function kind.
        kind: BuildIn,
        /// Type parameter.
        type_param_name: StructName<'a>,
        /// Type parameters of type parameter.
        type_params: Vec<FType<'a>>,
        /// Parameters.
        params: Vec<ExpLoc<'a>>,
    },
    /// Call plain function.
    Plain {
        /// Function module.
        module: Option<Import<'a>>,
        /// Function name.
        name: &'a str,
        /// Type parameters.
        type_params: Vec<FType<'a>>,
        /// Parameters.
        params: Vec<ExpLoc<'a>>,
    },
}

impl<'a> FnCall<'a> {
    /// Creates a new call plain function expression.
    pub fn plain(
        f_index: &FunctionHandleIndex,
        type_params: Option<&SignatureIndex>,
        ctx: &mut impl Context<'a>,
        unit: &'a impl UnitAccess,
    ) -> Exp<'a> {
        let handler = unit.function_handle(*f_index);
        let f_name = unit.identifier(handler.name);

        let params_count = unit.signature(handler.parameters).len();
        let params = ctx.pop_exp_vec(params_count);

        let type_params = ctx.extract_signature(type_params);

        let module_handle = unit.module_handle(handler.module);
        let import = ctx.module_import(module_handle);

        Exp::FnCall(FnCall::Plain {
            module: import,
            name: f_name,
            type_params,
            params,
        })
    }

    /// Creates a new call build-in function expression.
    pub fn build_in(
        kind: BuildIn,
        index: &StructDefinitionIndex,
        type_params: Option<&SignatureIndex>,
        ctx: &mut impl Context<'a>,
        unit: &'a impl UnitAccess,
    ) -> Exp<'a> {
        if let Some(def) = unit.struct_def(*index) {
            let struct_handler = unit.struct_handle(def.struct_handle);
            let module_handle = unit.module_handle(struct_handler.module);

            let import = ctx.module_import(module_handle);
            let params = ctx.pop_exp_vec(kind.parameters_count());

            let type_params = ctx.extract_signature(type_params);

            Exp::FnCall(FnCall::BuildIn {
                kind,
                type_param_name: StructName {
                    name: unit.identifier(struct_handler.name),
                    import,
                },
                type_params,
                params,
            })
        } else {
            ctx.err()
        }
    }
}

impl<'a> SourceRange for FnCall<'a> {
    fn source_range(&self) -> Option<(usize, usize)> {
        match self {
            FnCall::BuildIn {
                kind: _,
                type_param_name: _,
                type_params: _,
                params,
            }
            | FnCall::Plain {
                module: _,
                name: _,
                type_params: _,
                params,
            } => find_range(params.iter()),
        }
    }
}

impl<'a> Encode for FnCall<'a> {
    fn encode<W: Write>(&self, w: &mut W, indent: usize) -> Result<(), Error> {
        match self {
            FnCall::BuildIn {
                kind,
                type_param_name,
                type_params,
                params,
            } => {
                kind.encode(w, indent)?;
                w.write_str("<")?;
                type_param_name.encode(w, 0)?;
                if !type_params.is_empty() {
                    write_array(w, "<", ", ", type_params, ">")?;
                }
                w.write_str(">")?;
                write_array(w, "(", ", ", params, ")")
            }
            FnCall::Plain {
                module,
                name,
                type_params,
                params,
            } => {
                if let Some(import) = module {
                    import.encode(w, 0)?;
                    w.write_str("::")?;
                }
                write!(w, "{}", name)?;
                if !type_params.is_empty() {
                    write_array(w, "<", ", ", type_params, ">")?;
                }
                write_array(w, "(", ", ", params, ")")
            }
        }
    }
}

/// Build-in functions.
#[derive(Debug)]
pub enum BuildIn {
    /// exists
    Exists,
    /// move_from
    MoveFrom,
    /// move_to
    MoveTo,
    /// borrow_global
    BorrowGlobal,
    /// borrow_global_mut
    BorrowGlobalMut,
}

impl BuildIn {
    /// Returns parameters count.
    pub fn parameters_count(&self) -> usize {
        match self {
            BuildIn::Exists => 1,
            BuildIn::MoveFrom => 1,
            BuildIn::MoveTo => 2,
            BuildIn::BorrowGlobal => 1,
            BuildIn::BorrowGlobalMut => 1,
        }
    }
}

impl Encode for BuildIn {
    fn encode<W: Write>(&self, w: &mut W, _: usize) -> Result<(), Error> {
        w.write_str(match self {
            BuildIn::Exists => "exists",
            BuildIn::MoveFrom => "move_from",
            BuildIn::MoveTo => "move_to",
            BuildIn::BorrowGlobal => "borrow_global",
            BuildIn::BorrowGlobalMut => "borrow_global_mut",
        })?;
        Ok(())
    }
}

/// Struct full name.
#[derive(Debug)]
pub struct StructName<'a> {
    /// Struct name.
    pub name: &'a str,
    /// Struct import.
    pub import: Option<Import<'a>>,
}

impl<'a> Encode for StructName<'a> {
    fn encode<W: Write>(&self, w: &mut W, indent: usize) -> Result<(), Error> {
        if let Some(import) = &self.import {
            import.encode(w, indent)?;
        }
        w.write_str(self.name)?;
        Ok(())
    }
}
