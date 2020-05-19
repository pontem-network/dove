use std::collections::HashMap;
use std::fs;

fn build_replacement_table() -> HashMap<&'static str, &'static str> {
    let mut table = HashMap::new();
    table.insert("move_core_types::", "dfinance_move_core_types::");
    table.insert("move_lang::", "dfinance_move_lang::");
    table.insert("move_ir_types::", "dfinance_move_ir_types::");

    table.insert("move_vm_runtime::", "dfinance_move_vm_runtime::");
    table.insert("move_vm_state::", "dfinance_move_vm_state::");
    table.insert("move_vm_types::", "dfinance_move_vm_types::");
    table.insert("vm::", "dfinance_vm::");

    table.insert("libra_state_view::", "dfinance_libra_state_view::");
    table.insert("libra_types::", "dfinance_libra_types::");
    table.insert(
        "libra_canonical_serialization::",
        "dfinance_libra_canonical_serialization::",
    );
    table.insert("libra_crypto::", "dfinance_libra_crypto::");
    table.insert("language_e2e_tests::", "dfinance_language_e2e_tests::");

    table.insert("crate::libra", "crate::dfinance_generated");
    table
}

fn convert_contents_into_dialect(s: &str) -> String {
    let mut s = s.to_string();
    for (original, replacement) in build_replacement_table().into_iter() {
        s = s.replace(original, replacement);
    }
    s
}

fn main() {
    let sources_dir = std::env::current_dir().unwrap().join("src");
    let libra_dir = sources_dir.join("libra");
    let dfinance_dir = sources_dir.join("dfinance_generated");

    for file in walkdir::WalkDir::new(libra_dir).max_depth(1) {
        let entry = file.unwrap();
        let dfinance_file_path = dfinance_dir.join(entry.file_name());
        if entry.file_type().is_file() {
            let contents = fs::read_to_string(entry.path()).unwrap();
            let dialected_contents = convert_contents_into_dialect(&contents);
            fs::write(dfinance_file_path, dialected_contents).unwrap();
        }
    }
}
