/// Move metadata extractor.
pub mod meta;
/// Dependency resolver.
pub mod resolver;
/// Index store.
pub mod store;

use std::path::{PathBuf, Path};
use std::collections::{HashMap, HashSet};
use anyhow::Error;
use crate::manifest::{Dependence, MANIFEST, read_manifest};
use crate::context::Context;
use std::fs;
use std::str::FromStr;
use crate::index::store::{Module, SourceType};
use resolver::git::GitIndex;
use std::rc::Rc;
use walkdir::{WalkDir, DirEntry};
use crate::index::meta::{source_meta, FileMeta};
use resolver::{git};
use crate::index::resolver::chain;
use crate::index::resolver::chain::ChainIndex;
use move_core_types::language_storage::ModuleId;

/// Modules index.
pub type ModulesIndex = HashMap<Rc<ModuleId>, HashMap<SourceType, Module>>;

/// Modules index.
pub struct Index<'a> {
    /// Modules index.
    pub modules: ModulesIndex,
    /// Set of dependencies names.
    pub dep_names: HashSet<Rc<str>>,
    /// Dove context.
    pub ctx: &'a Context,
}

impl<'a> Index<'a> {
    /// Build modules index.
    pub fn build(&mut self) -> Result<(), Error> {
        let deps_path = self.ctx.path_for(&self.ctx.manifest.layout.target_deps);
        if !deps_path.exists() {
            fs::create_dir_all(&deps_path)?;
        }

        if let Some(dependencies) = &self.ctx.manifest.package.dependencies {
            self.load_deps(&dependencies.deps)?;
        }

        self.modules.iter_mut().for_each(|(_, m)| {
            m.remove(&SourceType::Local);
        });

        let deps_dir = deps_path.read_dir()?;
        let mut new_deps = HashSet::new();
        for dir in deps_dir {
            let dir = dir?;
            let name = Rc::from(dir.file_name().to_str().ok_or_else(|| {
                anyhow!("Failed to convert dependence name:{:?}", dir.file_name())
            })?);

            let path = dir.path();
            if !self.dep_names.contains(&name) {
                if name.starts_with(git::PREFIX) {
                    let git = GitIndex::new(self.ctx, &path);
                    self.store_meta(git.meta()?, SourceType::Git, name.clone());
                } else if name.starts_with(chain::PREFIX) {
                    let chain = ChainIndex::new(self.ctx, &path);
                    self.store_meta(chain.meta()?, SourceType::Chain, name.clone());
                    chain.meta()?;
                }
                new_deps.insert(name.clone());
            }
        }

        self.dep_names = new_deps;

        if let Some(dependencies) = &self.ctx.manifest.package.dependencies {
            dependencies
                .deps
                .iter()
                .filter_map(|dep| {
                    if let Dependence::Path(path) = dep {
                        Some(path.path.as_str())
                    } else {
                        None
                    }
                })
                .map(|path| self.index_deps_for(&path))
                .collect::<Result<Vec<_>, Error>>()?;
        }

        self.store()?;
        Ok(())
    }

    /// Returns set of dependencies paths.
    pub fn make_dependency_set<P: AsRef<Path>>(
        &mut self,
        paths: &[P],
    ) -> Result<HashSet<Rc<str>>, Error> {
        let mut modules = HashSet::new();
        let mut imports = HashSet::new();

        for path in paths {
            let path = path.as_ref();
            if path.is_file() {
                let f_meta = source_meta(
                    path,
                    Some(self.ctx.account_address()?),
                    self.ctx.dialect.as_ref(),
                )?;
                for meta in f_meta.meta {
                    modules.insert(meta.module_id);
                    imports.extend(meta.imports);
                }
            } else {
                for mv_file in move_dir_iter(path) {
                    let f_meta = source_meta(
                        mv_file.path(),
                        Some(self.ctx.account_address()?),
                        self.ctx.dialect.as_ref(),
                    )?;

                    for meta in f_meta.meta {
                        modules.insert(meta.module_id);
                        imports.extend(meta.imports);
                    }
                }
            }
        }

        let imports = imports
            .into_iter()
            .filter(|module| !modules.contains(&module))
            .collect::<HashSet<_>>();

        let modules_count = self.modules.len();

        let mut deps = HashSet::new();
        self.resolve_imports(&imports, &mut deps)?;

        if modules_count != self.modules.len() {
            self.store()?;
        }
        Ok(deps)
    }

    fn resolve_imports(
        &mut self,
        imports: &HashSet<Rc<ModuleId>>,
        deps: &mut HashSet<Rc<str>>,
    ) -> Result<(), Error> {
        fn resolve<'b>(
            index: &mut Index<'b>,
            import: &Rc<ModuleId>,
            deps: &mut HashSet<Rc<str>>,
        ) -> Result<bool, Error> {
            if let Some(module) = index.get_module(&import) {
                deps.insert(module.path.clone());
                let imports = module.dependencies.clone();
                index.resolve_imports(&imports, deps)?;
                Ok(true)
            } else {
                Ok(false)
            }
        }

        for import in imports {
            if !resolve(self, import, deps)? {
                let path = chain::resolve(self.ctx, import)?;
                let index = ChainIndex::new(self.ctx, &path);
                let name = path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .map(|name| Rc::from(name.to_owned()))
                    .ok_or_else(|| anyhow!("Failed to get dependencies name :[{:?}]", path))?;

                let files_meta = index.meta()?;

                for file in &files_meta {
                    for module in &file.meta {
                        self.resolve_imports(&module.imports, deps)?;
                    }
                }

                self.store_meta(files_meta, SourceType::Chain, name);

                if !resolve(self, import, deps)? {
                    return Err(anyhow!("Failed to resolve dependency:{:?}", import));
                }
            }
        }

        Ok(())
    }

    fn get_module(&self, name: &Rc<ModuleId>) -> Option<&Module> {
        self.modules.get(name).and_then(|modules| {
            modules.get(&SourceType::Local).or_else(|| {
                modules
                    .get(&SourceType::Git)
                    .or_else(|| modules.get(&SourceType::Chain))
            })
        })
    }

    fn index_deps_for<A: AsRef<Path>>(&mut self, path: A) -> Result<(), Error> {
        let dep_name = Rc::<str>::from(
            path.as_ref()
                .to_str()
                .ok_or_else(|| anyhow!("Failed to convert source path:{:?}", path.as_ref()))?
                .to_owned(),
        );

        for file in move_dir_iter(path) {
            let meta = source_meta(
                file.path(),
                Some(self.ctx.account_address()?),
                self.ctx.dialect.as_ref(),
            )?;

            self.store_meta(vec![meta], SourceType::Local, dep_name.clone());
        }
        Ok(())
    }

    fn load_deps(&self, deps: &[Dependence]) -> Result<(), Error> {
        for dep in deps {
            match dep {
                Dependence::Git(git) => {
                    let path = git::resolve(&self.ctx, &git)?;
                    let manifest = path.join(MANIFEST);
                    if manifest.exists() {
                        if let Ok(manifest) = read_manifest(&manifest) {
                            if let Some(dependencies) = manifest.package.dependencies {
                                self.load_deps(&dependencies.deps)?;
                            }
                        }
                    }
                }
                Dependence::Path(path) => {
                    let path = PathBuf::from_str(&path.path)?;
                    let path = if path.is_absolute() {
                        path
                    } else {
                        self.ctx.path_for(path)
                    };

                    if !path.exists() {
                        return Err(anyhow!("Unresolved dependencies path:{:?}", path));
                    }
                }
            }
        }

        Ok(())
    }

    fn store_meta(&mut self, f_meta: Vec<FileMeta>, src_type: SourceType, dep_name: Rc<str>) {
        for file in f_meta {
            for unit in file.meta {
                let name = Rc::new(unit.module_id);
                let entry = self.modules.entry(name.clone());
                let modules = entry.or_insert_with(HashMap::default);

                let dependencies = unit.imports.into_iter().collect();

                modules.insert(
                    src_type,
                    Module {
                        name,
                        dep_name: dep_name.clone(),
                        path: file.path.clone(),
                        source_type: src_type,
                        dependencies,
                    },
                );
            }
        }
    }
}

/// Creates an iterator from move files
pub fn move_dir_iter<P: AsRef<Path>>(path: P) -> impl Iterator<Item = DirEntry> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext.eq("move"))
                .unwrap_or(false)
        })
}
