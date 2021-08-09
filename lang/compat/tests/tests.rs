use move_binary_format::CompiledModule;
use move_compat::{adapt_to_basis, adapt_from_basis, AddressType};
use move_binary_format::file_format::CompiledScript;

const DFI_FOO: &[u8] = include_bytes!("assets/dfi/1_Foo.mv");
const DFI_SCRIPT: &[u8] = include_bytes!("assets/dfi/0_main.mv");

const DIEM_FOO: &[u8] = include_bytes!("assets/diem/1_Foo.mv");
const DIEM_SCRIPT: &[u8] = include_bytes!("assets/diem/0_main.mv");

const PONT_FOO: &[u8] = include_bytes!("assets/pont/1_Foo.mv");
const PONT_SCRIPT: &[u8] = include_bytes!("assets/pont/0_main.mv");

#[test]
#[should_panic]
fn failed_to_load_dfi_module() {
    CompiledModule::deserialize(DFI_FOO).unwrap();
}

#[test]
#[should_panic]
fn failed_to_load_dfi_script() {
    CompiledModule::deserialize(DFI_SCRIPT).unwrap();
}

#[test]
#[should_panic]
fn failed_to_load_diem_module() {
    CompiledModule::deserialize(DIEM_FOO).unwrap();
}

#[test]
#[should_panic]
fn failed_to_load_diem_script() {
    CompiledModule::deserialize(DIEM_SCRIPT).unwrap();
}

#[test]
fn success_load_pont() {
    CompiledModule::deserialize(PONT_FOO).unwrap();
    CompiledScript::deserialize(PONT_SCRIPT).unwrap();
}

#[test]
fn test_adapt_dfi() {
    let mut module = DFI_FOO.to_vec();
    adapt_to_basis(&mut module, AddressType::Dfninance).unwrap();
    CompiledModule::deserialize(&module).unwrap();
    assert_eq!(PONT_FOO, module.as_slice());
    adapt_from_basis(&mut module, AddressType::Dfninance).unwrap();
    assert_eq!(DFI_FOO, module.as_slice());

    let mut script = DFI_SCRIPT.to_vec();
    adapt_to_basis(&mut script, AddressType::Dfninance).unwrap();
    CompiledModule::deserialize(&script).unwrap();
    assert_eq!(PONT_SCRIPT, script.as_slice());
    adapt_from_basis(&mut script, AddressType::Dfninance).unwrap();
    assert_eq!(DFI_SCRIPT, script.as_slice());
}

#[test]
fn test_adapt_diem() {
    let mut module = DIEM_FOO.to_vec();
    adapt_to_basis(&mut module, AddressType::Diem).unwrap();
    CompiledModule::deserialize(&module).unwrap();
    assert_eq!(PONT_FOO, module.as_slice());
    adapt_from_basis(&mut module, AddressType::Diem).unwrap();
    assert_eq!(DIEM_FOO, module.as_slice());

    let mut script = DIEM_SCRIPT.to_vec();
    adapt_to_basis(&mut script, AddressType::Diem).unwrap();
    CompiledModule::deserialize(&script).unwrap();
    assert_eq!(PONT_SCRIPT, script.as_slice());
    adapt_from_basis(&mut script, AddressType::Diem).unwrap();
    assert_eq!(DIEM_SCRIPT, script.as_slice());
}
