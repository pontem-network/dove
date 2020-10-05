use std::path::{Path, PathBuf};
use crate::manifest::MoveToml;
use std::fs;
use walkdir::WalkDir;
use std::fs::{File, OpenOptions};
use std::io::Write;
use anyhow::{Result, Error};
use libra::{prelude::*, compiler::*};
use crate::dependence::extractor::{
    extract_dependencies_from_source, extract_dependencies_from_bytecode,
};
use std::collections::{HashMap, HashSet};
use termcolor::{StandardStream, ColorChoice, Buffer};
use lang::disassembler::{Config, Disassembler, unit::CompiledUnit as Unit};
use crate::dependence::loader::{BytecodeLoader, Loader};
use dialects::shared::bech32::bech32_into_libra;

/// Move builder.
pub struct Builder<'a, S: BytecodeLoader> {
    /// movec project directory.
    project_dir: &'a Path,
    /// movec manifest.
    manifest: MoveToml,
    /// Optional dependencies loader. If none is provided and code has external dependencies, compilation ends with a dependence not found error.
    loader: &'a Option<Loader<S>>,
    /// Print error flag. If true, print compilation errors to stdout.
    print_err: bool,
    /// Shutdown on error flag. If true, the process will exit with error code on compilation error.
    shutdown_on_err: bool,
    /// Static name pool.
    _name_pool: ConstPool,
}

impl<'a, S> Builder<'a, S>
where
    S: BytecodeLoader,
{
    /// Creates a new move builder.
    pub fn new(
        project_dir: &'a Path,
        manifest: MoveToml,
        loader: &'a Option<Loader<S>>,
        print_err: bool,
        shutdown_on_err: bool,
    ) -> Builder<'a, S> {
        Builder {
            project_dir,
            manifest,
            loader,
            print_err,
            shutdown_on_err,
            _name_pool: Default::default(),
        }
    }

    /// Initializes directory layout.
    pub fn init_build_layout(&self) -> Result<()> {
        let temp_dir = self.temp_dir()?;
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir)?;
        }
        fs::create_dir_all(&temp_dir)?;

        let deps_dir = self.deps_dir()?;
        if !temp_dir.exists() {
            fs::create_dir_all(&deps_dir)?;
        }

        let modules_output = self.modules_out_dir()?;
        if !modules_output.exists() {
            fs::create_dir_all(&modules_output)?;
        }

        let scripts_output = self.scripts_out_dir()?;
        if !scripts_output.exists() {
            fs::create_dir_all(&scripts_output)?;
        }

        let deps_dir = self.deps_dir()?;
        if !deps_dir.exists() {
            fs::create_dir_all(&deps_dir)?;
        }

        Ok(())
    }

    /// Load dependencies for each source file.
    pub fn load_dependencies(&self, sources: &[PathBuf]) -> Result<HashMap<ModuleId, Vec<u8>>> {
        let address = self
            .address()?
            .map(|addr| AccountAddress::new(addr.to_u8()));
        let source_imports = extract_dependencies_from_source(
            sources,
            address,
            self.print_err,
            self.shutdown_on_err,
        )?;
        let mut deps = HashMap::new();

        let mut dep_list = HashSet::new();
        if let Some(loader) = &self.loader {
            for import in source_imports {
                let bytecode = loader.get(&import)?;
                self.load_bytecode_tree(&bytecode, &mut deps, &mut dep_list)?;
                deps.insert(import, bytecode);
            }
        }

        Ok(deps)
    }

    /// Load dependencies tree.
    fn load_bytecode_tree(
        &self,
        bytecode: &[u8],
        deps: &mut HashMap<ModuleId, Vec<u8>>,
        dep_list: &mut HashSet<ModuleId>,
    ) -> Result<()> {
        let source_imports = extract_dependencies_from_bytecode(bytecode)?;
        if let Some(loader) = &self.loader {
            for import in source_imports {
                if dep_list.insert(import.clone()) {
                    let bytecode = loader.get(&import)?;
                    self.load_bytecode_tree(&bytecode, deps, dep_list)?;
                    deps.insert(import, bytecode);
                }
            }
        }

        Ok(())
    }

    /// Disassembles dependencies.
    pub fn prepare_deps(&self, bytecode: HashMap<ModuleId, Vec<u8>>) -> Result<Vec<PathBuf>> {
        let deps = self.temp_dir()?.join("deps");
        fs::create_dir_all(&deps)?;

        let mut path_list = Vec::with_capacity(bytecode.len());

        for (id, bytecode) in bytecode {
            let config = Config {
                light_version: true,
            };
            let unit = Unit::new(&bytecode)?;
            let disasm = Disassembler::new(&unit, config);
            let source_unit = disasm.make_source_unit();
            let signature = source_unit.code_string()?;

            let path = deps.join(format!("{}_{}.move", id.address(), id.name().as_str()));

            let mut f = OpenOptions::new()
                .create(true)
                .write(true)
                .open(&deps.join(&path))?;
            f.write_all(signature.as_bytes())?;

            path_list.push(path)
        }

        Ok(path_list)
    }

    /// Makes source map.
    pub fn make_source_map(&self) -> Result<Vec<PathBuf>> {
        fn add_source(sources: &mut Vec<PathBuf>, path: &Path) {
            for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
                let path = entry.into_path();
                if let Some(extension) = path.extension() {
                    if extension == "move" {
                        sources.push(path.to_owned());
                    }
                }
            }
        }

        let mut source_list = vec![];
        add_source(&mut source_list, &self.source_modules_dir()?);
        add_source(&mut source_list, &self.source_scripts_dir()?);

        Ok(source_list)
    }

    /// Compile source list with dependencies.
    pub fn compile(
        &self,
        source_list: Vec<PathBuf>,
        dep_list: Vec<PathBuf>,
    ) -> Result<(FilesSourceText, Vec<CompiledUnit>)> {
        let source_list = convert_path(&source_list)?;
        let dep_list = convert_path(&dep_list)?;
        let addr = self.address()?;

        let (files, pprog_and_comments_res) = parse_program(&source_list, &dep_list)?;
        let pprog_res = pprog_and_comments_res.map(|(pprog, _)| pprog);
        match compile_program(pprog_res, addr) {
            Err(errors) => {
                if self.print_err {
                    let mut writer = StandardStream::stderr(ColorChoice::Auto);
                    output_errors(&mut writer, files, errors);
                    if self.shutdown_on_err {
                        std::process::exit(1)
                    } else {
                        Err(Error::msg("Unexpected errors."))
                    }
                } else {
                    let mut writer = Buffer::ansi();
                    output_errors(&mut writer, files, errors);
                    Err(Error::msg(String::from_utf8(writer.into_inner())?))
                }
            }
            Ok(compiled_units) => Ok((files, compiled_units)),
        }
    }

    /// Check source files.
    pub fn check(&self, source_list: Vec<PathBuf>, dep_list: Vec<PathBuf>) -> Result<()> {
        let source_list = convert_path(&source_list)?;
        let dep_list = convert_path(&dep_list)?;
        let addr = self.address()?;
        Ok(move_check(&source_list, &dep_list, addr)?)
    }

    /// Verify and store compiled units.
    pub fn verify_and_store(
        &self,
        files: FilesSourceText,
        compiled_units: Vec<CompiledUnit>,
    ) -> Result<()> {
        let (compiled_units, ice_errors) = compiled_unit::verify_units(compiled_units);
        let (modules, scripts): (Vec<_>, Vec<_>) = compiled_units
            .into_iter()
            .partition(|u| matches!(u, CompiledUnit::Module { .. }));

        fn store(units: Vec<CompiledUnit>, base_dir: &PathBuf) -> Result<()> {
            for (idx, unit) in units.into_iter().enumerate() {
                let mut path = base_dir.join(format!("{}_{}", idx, unit.name()));
                path.set_extension("mv");
                File::create(&path)?.write_all(&unit.serialize())?
            }
            Ok(())
        }

        if !modules.is_empty() {
            let modules_dir = self.modules_out_dir()?;
            if modules_dir.exists() {
                fs::remove_dir_all(&modules_dir)?;
                fs::create_dir_all(&modules_dir)?;
            }
            store(modules, &modules_dir)?;
        }

        if !scripts.is_empty() {
            let scripts_dir = self.scripts_out_dir()?;
            if scripts_dir.exists() {
                fs::remove_dir_all(&scripts_dir)?;
                fs::create_dir_all(&scripts_dir)?;
            }

            store(scripts, &scripts_dir)?;
        }

        if !ice_errors.is_empty() {
            if self.print_err {
                let mut writer = StandardStream::stderr(ColorChoice::Auto);
                output_errors(&mut writer, files, ice_errors);
            }
            if self.shutdown_on_err {
                std::process::exit(1);
            }
        }
        Ok(())
    }

    /// Verifies sources.
    pub fn verify(
        &self,
        files: FilesSourceText,
        compiled_units: Vec<CompiledUnit>,
    ) -> Result<HashMap<String, Vec<u8>>> {
        let (compiled_units, ice_errors) = compiled_unit::verify_units(compiled_units);
        let (modules, scripts): (Vec<_>, Vec<_>) = compiled_units
            .into_iter()
            .partition(|u| matches!(u, CompiledUnit::Module { .. }));

        let mut bytecode_map = HashMap::new();

        for module in modules {
            bytecode_map.insert(module.name(), module.serialize());
        }

        for script in scripts {
            bytecode_map.insert(script.name(), script.serialize());
        }

        if ice_errors.is_empty() {
            Ok(bytecode_map)
        } else {
            let mut writer = Buffer::ansi();
            output_errors(&mut writer, files, ice_errors);
            Err(Error::msg(String::from_utf8(writer.into_inner())?))
        }
    }

    /// Returns the account address from movec manifest.
    fn address(&self) -> Result<Option<Address>> {
        let package = &self.manifest.package;
        match package.account_address.as_ref().map(|addr| {
            if addr.starts_with("0x") {
                Address::parse_str(&addr).map_err(|err| anyhow!(err))
            } else {
                bech32_into_libra(&addr)
                    .and_then(|addr| Address::parse_str(&addr).map_err(|err| anyhow!(err)))
            }
        }) {
            Some(r) => r.map(Some),
            None => Ok(None),
        }
    }

    /// Temporary directory path.
    fn temp_dir(&self) -> Result<PathBuf> {
        self.manifest
            .layout
            .as_ref()
            .and_then(|l| l.temp_dir.as_ref())
            .map(|t| self.project_dir.join(t))
            .ok_or_else(|| anyhow!("Expected temp_dir"))
    }

    /// Dependencies directory path.
    fn deps_dir(&self) -> Result<PathBuf> {
        self.manifest
            .layout
            .as_ref()
            .and_then(|l| l.bytecode_cache.as_ref())
            .map(|t| self.project_dir.join(t))
            .ok_or_else(|| anyhow!("Expected bytecode_cache"))
    }

    /// Module output directory path.
    fn modules_out_dir(&self) -> Result<PathBuf> {
        self.manifest
            .layout
            .as_ref()
            .and_then(|l| l.module_output.as_ref())
            .map(|t| self.project_dir.join(t))
            .ok_or_else(|| anyhow!("Expected module_output"))
    }

    /// Script output directory.
    fn scripts_out_dir(&self) -> Result<PathBuf> {
        self.manifest
            .layout
            .as_ref()
            .and_then(|l| l.script_output.as_ref())
            .map(|t| self.project_dir.join(t))
            .ok_or_else(|| anyhow!("Expected script_output"))
    }

    /// Modules source directory path.
    fn source_modules_dir(&self) -> Result<PathBuf> {
        self.manifest
            .layout
            .as_ref()
            .and_then(|l| l.module_dir.as_ref())
            .map(|t| self.project_dir.join(t))
            .ok_or_else(|| anyhow!("Expected module_output"))
    }

    /// Scripts source directory path.
    fn source_scripts_dir(&self) -> Result<PathBuf> {
        self.manifest
            .layout
            .as_ref()
            .and_then(|l| l.script_dir.as_ref())
            .map(|t| self.project_dir.join(t))
            .ok_or_else(|| anyhow!("Expected script_output"))
    }
}

/// Prints errors to stdout.
pub fn report_errors(files: FilesSourceText, errors: Errors) {
    let mut writer = StandardStream::stderr(ColorChoice::Auto);
    errors::output_errors(&mut writer, files, errors);
}

/// Converts paths buffers into strings.
pub fn convert_path(path_list: &[PathBuf]) -> Result<Vec<String>> {
    path_list
        .iter()
        .map(|path| path.to_str().map(|path| path.to_owned()))
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| anyhow!("Failed to convert source path"))
}

impl<'a, S> Drop for Builder<'a, S>
where
    S: BytecodeLoader,
{
    /// Cleans up the builder layout.
    fn drop(&mut self) {
        let res = self.temp_dir().and_then(|dir| {
            if dir.exists() {
                Ok(fs::remove_dir_all(&dir)?)
            } else {
                Ok(())
            }
        });

        if let Err(err) = res {
            println!("Failed to clean up temporary directory:{}", err);
        }
    }
}
