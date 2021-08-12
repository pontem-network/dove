extern crate anyhow;

use std::fmt::Write;

use anyhow::Error;

use generics::Generics;
use imports::Imports;
use module::Module as ModuleAst;
use script::Script as ScriptAst;
use unit::{CompiledUnit as Unit, CompiledUnit, SourceUnit, UnitAccess};

/// Code decompiler.
pub mod code;
/// Function decompiler.
pub mod functions;
/// Generic decompiler.
pub mod generics;
/// Imports decompiler.
pub mod imports;
/// Module decompiler.
pub mod module;
/// Struct decompiler.
pub mod script;
/// Struct decompiler.
pub mod structs;
/// Common types.
pub mod types;
/// Bytecode abstractions.
pub mod unit;

pub mod ext {
    #[cfg(all(target_arch = "wasm32", feature = "cffi"))]
    compile_error!("Target 'wasm32' is incompatible with feature 'cffi'.");
    #[cfg(feature = "cffi")]
    pub mod cffi;
    #[cfg(target_arch = "wasm32")]
    pub mod wasm;
}

pub const VERSION: &str = git_hash::crate_version_with_git_hash_short!();

/// Code indent.
pub const INDENT: usize = 4;

/// Decompile bytecode with config and write source code to writer.
pub fn decompile<W: Write>(bytecode: &[u8], writer: &mut W, config: Config) -> Result<(), Error> {
    let unit = Unit::new(bytecode)?;

    let disasm = Decompiler::new(&unit, config);
    let ast = disasm.make_source_unit();
    ast.write_code(writer)
}

/// Decompile bytecode with config.
pub fn decompile_str(bytecode: &[u8], config: Config) -> Result<String, Error> {
    let mut code = String::new();
    decompile(bytecode, &mut code, config)?;
    Ok(code)
}

/// Decompiler configuration.
#[derive(Debug)]
pub struct Config {
    /// Use light decompiler version.
    pub light_version: bool,
}

/// Move decompiler.
#[derive(Debug)]
pub struct Decompiler<'a> {
    unit: &'a CompiledUnit,
    imports: Imports<'a>,
    generics: Generics,
    config: Config,
}

impl<'a> Decompiler<'a> {
    /// Create a new decompiler.
    pub fn new(unit: &'a CompiledUnit, config: Config) -> Decompiler<'a> {
        let imports = Imports::new(unit);
        let generics = Generics::new(unit);

        Decompiler {
            unit,
            imports,
            generics,
            config,
        }
    }

    /// Convert a CompiledUnit to the SourceUnit.
    pub fn make_source_unit(&'a self) -> SourceUnit<'a> {
        if self.unit.is_script() {
            SourceUnit::Script(ScriptAst::new(
                self.unit,
                &self.imports,
                &self.generics,
                &self.config,
            ))
        } else {
            SourceUnit::Module(ModuleAst::new(
                self.unit,
                &self.imports,
                &self.generics,
                &self.config,
            ))
        }
    }
}

/// Encode to move code.
pub trait Encode {
    /// Writes component source representation into writer with given indent.
    fn encode<W: Write>(&self, w: &mut W, indent: usize) -> Result<(), Error>;
}

/// Encode encodable array.
pub fn write_array<E: Encode, W: Write>(
    w: &mut W,
    prefix: &str,
    decimeter: &str,
    encode: &[E],
    suffix: &str,
) -> Result<(), Error> {
    w.write_str(prefix)?;
    for (index, e) in encode.iter().enumerate() {
        e.encode(w, 0)?;
        if index != encode.len() - 1 {
            w.write_str(decimeter)?;
        }
    }
    w.write_str(suffix)?;
    Ok(())
}

#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod tests {
    use std::fs::File;
    use std::io::Write;

    use move_binary_format::CompiledModule;
    use move_binary_format::file_format::{Bytecode, CodeUnit, CompiledScript, FunctionDefinition};
    use move_lang::errors::report_errors_to_buffer;
    use move_lang::shared::Flags;
    use tempfile::tempdir;

    use lang::compiler::build;
    use lang::compiler::dialects::DialectName;

    use crate::{Config, decompile_str};

    fn compile(source: &str) -> Vec<u8> {
        let dialect = DialectName::DFinance.get_dialect();

        let sender = dialect.parse_address("0x1").unwrap();
        let deps = &[
            "src/assets/base.move".to_string(),
            "src/assets/tx.move".to_string(),
        ];

        let (files, prog) = build(
            &[source.to_owned()],
            deps,
            dialect.as_ref(),
            Some(sender),
            None,
            Flags::empty(),
        )
        .unwrap();

        match prog {
            Ok(mut prog) => prog.remove(0).serialize(),
            Err(errors) => {
                panic!(
                    "Failed to compile restored bytecode.{}",
                    String::from_utf8(report_errors_to_buffer(files, errors)).unwrap()
                );
            }
        }
    }

    pub fn perform_test(source: &str) {
        let original_bytecode = compile(source);
        let config = Config {
            light_version: false,
        };
        let restored_source = decompile_str(&original_bytecode, config).unwrap();
        println!("decompiled:\n{}", restored_source);

        let original_bytecode = CompiledModule::deserialize(&original_bytecode)
            .or_else(|_| {
                CompiledScript::deserialize(&original_bytecode).map(|s| s.into_module().1)
            })
            .unwrap();

        let dir = tempdir().unwrap();
        let tmp_file = dir.path().join("source.move");
        let mut file = File::create(&tmp_file).unwrap();
        writeln!(file, "{}", restored_source).unwrap();
        file.flush().unwrap();

        let restored_bytecode = compile(tmp_file.to_string_lossy().as_ref());
        compare_bytecode(
            original_bytecode,
            CompiledModule::deserialize(&restored_bytecode)
                .or_else(|_| {
                    CompiledScript::deserialize(&restored_bytecode).map(|s| s.into_module().1)
                })
                .unwrap(),
        );
    }

    fn compare_bytecode(expected: CompiledModule, actual: CompiledModule) {
        let mut expected = expected.into_inner();
        let mut actual = actual.into_inner();

        fn normalize_bytecode(bytecode: &mut CodeUnit) {
            bytecode.code = bytecode
                .code
                .iter()
                .cloned()
                .map(|mut bc| {
                    if let Bytecode::MoveLoc(i) = &bc {
                        bc = Bytecode::CopyLoc(*i);
                    }

                    bc
                })
                .collect();
        }

        fn normalize_f_def(func_def: &mut [FunctionDefinition]) {
            for def in func_def {
                if let Some(code) = &mut def.code {
                    normalize_bytecode(code);
                }
            }
        }

        normalize_f_def(&mut expected.function_defs);
        normalize_f_def(&mut actual.function_defs);

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_script() {
        perform_test("src/assets/script.move");
    }

    #[test]
    pub fn test_empty_module() {
        perform_test("src/assets/empty.move");
    }

    #[test]
    pub fn test_simple_struct() {
        perform_test("src/assets/struct.move");
    }

    #[test]
    pub fn test_function_signature() {
        perform_test("src/assets/function_sign.move");
    }

    #[test]
    pub fn test_abort() {
        perform_test("src/assets/code/abort.move");
    }

    #[test]
    pub fn test_call() {
        perform_test("src/assets/code/call.move");
    }

    #[test]
    pub fn test_arithmetic() {
        perform_test("src/assets/code/arithmetic.move");
    }

    #[test]
    pub fn test_values() {
        perform_test("src/assets/code/values.move");
    }

    #[test]
    pub fn test_fake_native() {
        perform_test("src/assets/code/fake_native.move");
    }

    #[test]
    pub fn test_let() {
        perform_test("src/assets/code/let.move");
    }

    #[test]
    pub fn test_pack() {
        perform_test("src/assets/code/pack.move");
    }

    #[test]
    pub fn test_unpack() {
        perform_test("src/assets/code/unpack.move");
    }

    #[test]
    pub fn test_loc() {
        perform_test("src/assets/code/loc.move");
    }

    #[ignore]
    #[test]
    pub fn test_loop() {
        perform_test("src/assets/code/loop.move");
    }

    #[ignore]
    #[test]
    pub fn test_while() {
        perform_test("src/assets/code/while.move");
    }

    #[ignore]
    #[test]
    pub fn test_if() {
        perform_test("src/assets/code/if.move");
    }
}
