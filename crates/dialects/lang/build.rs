use std::collections::HashMap;
use std::fs;

fn build_replacement_table() -> HashMap<&'static str, &'static str> {
    let mut table = HashMap::new();
    table.insert("orig_move_core_types::", "dfin_move_core_types::");
    table.insert("orig_move_lang::", "dfin_move_lang::");
    table.insert("orig_move_ir_types::", "dfin_move_ir_types::");

    table.insert("orig_move_vm_runtime::", "dfin_move_vm_runtime::");
    table.insert("orig_move_vm_state::", "dfin_move_vm_state::");
    table.insert("orig_move_vm_types::", "dfin_move_vm_types::");
    table.insert("orig_vm::", "dfin_vm::");

    table.insert("orig_libra_state_view::", "dfin_libra_state_view::");
    table.insert("orig_libra_types::", "dfin_libra_types::");
    table.insert(
        "orig_libra_canonical_serialization::",
        "dfin_libra_canonical_serialization::",
    );
    table.insert("orig_libra_crypto::", "dfin_libra_crypto::");
    table.insert("orig_language_e2e_tests::", "dfin_language_e2e_tests::");

    table.insert("crate::libra", "crate::dfina");
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
    let dfinance_dir = sources_dir.join("dfina");

    for file in walkdir::WalkDir::new(libra_dir).max_depth(1) {
        let entry = file.unwrap();
        if entry.file_name() == "gas.rs" {
            continue;
        }

        let dfinance_file_path = dfinance_dir.join(entry.file_name());
        if entry.file_type().is_file() {
            let original_s = fs::read_to_string(entry.path()).unwrap();

            let old_transformed_s =
                fs::read_to_string(&dfinance_file_path).unwrap_or_else(|_| String::default());

            let transformed_s = convert_contents_into_dialect(&original_s);
            if old_transformed_s != transformed_s {
                fs::write(dfinance_file_path, transformed_s).unwrap();
            }
        }
    }
}
