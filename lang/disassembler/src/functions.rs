use vm::file_format::*;
use crate::imports::Imports;
use crate::generics::{Generics, Generic, extract_type_params, write_type_parameters};
use crate::{Encode, write_array, Config};
use anyhow::Error;
use std::fmt::Write;
use crate::types::{FType, extract_type_signature, FullStructName, extract_struct_name};
use std::sync::atomic::{AtomicBool, Ordering};
use std::rc::Rc;
use crate::code::body::Body;
use crate::unit::{UnitAccess};

/// Function representation.
pub struct FunctionsDef<'a> {
    is_public: Visibility,
    is_native: bool,
    name: &'a str,
    type_params: Vec<Generic>,
    ret: Vec<FType<'a>>,
    params: Vec<Param<'a>>,
    acquires: Vec<FullStructName<'a>>,
    body: Option<Body<'a>>,
}

impl<'a> FunctionsDef<'a> {
    /// Returns module function representation.
    pub fn new(
        def: &'a FunctionDefinition,
        unit: &'a impl UnitAccess,
        generics: &'a Generics,
        imports: &'a Imports,
        config: &'a Config,
    ) -> FunctionsDef<'a> {
        let handler = unit.function_handle(def.function);
        let name = unit.identifier(handler.name);
        let type_params = extract_type_params(&handler.type_parameters, generics);
        let ret = FunctionsDef::ret(unit, imports, unit.signature(handler.return_), &type_params);
        let params = FunctionsDef::params(
            unit,
            imports,
            unit.signature(handler.parameters),
            &type_params,
        );

        if config.light_version {
            FunctionsDef {
                is_public: def.visibility,
                is_native: def.is_native(),
                name,
                type_params,
                ret,
                params,
                acquires: vec![],
                body: Some(Body::mock()),
            }
        } else {
            let body = def
                .code
                .as_ref()
                .map(|code| Body::new(code, ret.len(), unit, &params, &imports, &type_params));

            let acquires = def
                .acquires_global_resources
                .iter()
                .filter_map(|di| unit.struct_def(*di))
                .map(|sd| extract_struct_name(unit, &sd.struct_handle, imports))
                .collect();

            FunctionsDef {
                is_public: def.visibility,
                is_native: def.is_native(),
                name,
                type_params,
                ret,
                params,
                acquires,
                body,
            }
        }
    }

    /// Returns script main function representation.
    pub fn script(
        unit: &'a impl UnitAccess,
        imports: &'a Imports<'a>,
        generics: &'a Generics,
    ) -> FunctionsDef<'a> {
        let (type_params, params, body) =
            if let Some((code, type_parameters, params)) = unit.script_info() {
                let type_params = extract_type_params(type_parameters, generics);
                let params =
                    FunctionsDef::params(unit, imports, unit.signature(params), &type_params);
                let body = Body::new(code, 0, unit, &params, &imports, &type_params);
                (type_params, params, Some(body))
            } else {
                (vec![], vec![], None)
            };

        FunctionsDef {
            is_public: false,
            is_native: false,
            name: "main",
            type_params,
            ret: vec![],
            params,
            acquires: vec![],
            body,
        }
    }

    fn ret(
        unit: &'a impl UnitAccess,
        imports: &'a Imports<'a>,
        sign: &'a Signature,
        type_params: &[Generic],
    ) -> Vec<FType<'a>> {
        sign.0
            .iter()
            .map(|tkn| extract_type_signature(unit, tkn, imports, &type_params))
            .collect::<Vec<_>>()
    }

    fn params(
        unit: &'a impl UnitAccess,
        imports: &'a Imports<'a>,
        sign: &'a Signature,
        type_params: &[Generic],
    ) -> Vec<Param<'a>> {
        sign.0
            .iter()
            .enumerate()
            .map(|(index, tkn)| Param {
                used: Rc::new(AtomicBool::new(false)),
                index,
                f_type: Rc::new(extract_type_signature(unit, tkn, imports, &type_params)),
            })
            .collect::<Vec<_>>()
    }
}

impl<'a> Encode for FunctionsDef<'a> {
    fn encode<W: Write>(&self, w: &mut W, indent: usize) -> Result<(), Error> {
        write!(
            w,
            "{s:width$}{native}{p}fun {name}",
            s = "",
            width = indent as usize,
            p = if self.is_public { "public " } else { "" },
            native = if self.is_native { "native " } else { "" },
            name = self.name,
        )?;
        write_type_parameters(w, &self.type_params)?;

        write_array(w, "(", ", ", &self.params, ")")?;

        if !self.ret.is_empty() {
            w.write_str(": ")?;
            if self.ret.len() == 1 {
                self.ret[0].encode(w, 0)?;
            } else {
                write_array(w, "(", ", ", &self.ret, ")")?;
            }
        }

        if !self.acquires.is_empty() {
            write_array(w, " acquires ", ", ", &self.acquires, " ")?;
        }

        if self.is_native {
            w.write_str(";")?;
        } else {
            w.write_str(" {")?;
            if let Some(body) = self.body.as_ref() {
                body.encode(w, indent)?;
            }
            write!(w, "\n{s:width$}}}", s = "", width = indent as usize)?;
        }
        Ok(())
    }
}

/// Function parameter representation.
#[derive(Debug, Clone)]
pub struct Param<'a> {
    used: Rc<AtomicBool>,
    index: usize,
    f_type: Rc<FType<'a>>,
}

impl<'a> Param<'a> {
    /// Marks a parameter as used.
    pub fn mark_as_used(&self) {
        self.used.store(true, Ordering::Relaxed);
    }

    /// Writes parameter name.
    pub fn write_name<W: Write>(&self, w: &mut W) -> Result<(), Error> {
        if !self.used.load(Ordering::Relaxed) {
            w.write_str("_")?;
        }
        w.write_str("arg")?;

        if self.index != 0 {
            write!(w, "{}", self.index)?;
        }
        Ok(())
    }
}

impl<'a> Encode for Param<'a> {
    fn encode<W: Write>(&self, w: &mut W, indent: usize) -> Result<(), Error> {
        self.write_name(w)?;
        w.write_str(": ")?;
        self.f_type.encode(w, indent)
    }
}
