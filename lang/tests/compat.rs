#[macro_use]
extern crate include_dir;
use include_dir::Dir;
use diem::module::{CompiledModule, CompiledScript};
use lang::compat::adapt;

static MODULES_TESTS_DIR: Dir = include_dir!("tests/libra_std/modules");
static SCRIPTS_TESTS_DIR: Dir = include_dir!("tests/libra_std/scripts");

#[test]
pub fn custom_module() {
    let mut content = include_bytes!("libra_std/modules/3_U.mv").to_vec();
    adapt(&mut content).expect("Valid bytecode.");
    CompiledModule::deserialize(&content).unwrap();
}

#[test]
pub fn test_adapter() {
    for_each(MODULES_TESTS_DIR, UnitType::Module);
    for_each(SCRIPTS_TESTS_DIR, UnitType::Script);
}

fn for_each(dir: Dir, unit_type: UnitType) {
    for file in dir.files() {
        let mut content = file.contents().to_vec();

        adapt(&mut content).expect("Valid bytecode.");

        match unit_type {
            UnitType::Script => {
                CompiledScript::deserialize(&content)
                    .map_err(|err| {
                        format!(
                            "Failed to adapt bytecode {:?}. Error:{:?}",
                            file.path(),
                            err
                        )
                    })
                    .unwrap();
            }
            UnitType::Module => {
                CompiledModule::deserialize(&content)
                    .map_err(|err| {
                        format!(
                            "Failed to adapt bytecode {:?}. Error:{:?}",
                            file.path(),
                            err
                        )
                    })
                    .unwrap();
            }
        }
    }
}

enum UnitType {
    Module,
    Script,
}
