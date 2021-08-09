/// Documentation generator options.
pub mod options;

use docgen::{DocgenOptions as DiemDocgenOptions, Docgen};
use move_model::model::GlobalEnv;
use crate::docs::options::DocgenOptions;
use anyhow::Error;
use std::fs;
use std::path::PathBuf;
use termcolor::Buffer;
use codespan_reporting::diagnostic::Severity;

/// Generate move documentation.
pub fn generate_docs(
    env: &GlobalEnv,
    options: &DocgenOptions,
    doc_path: Vec<String>,
    output_directory: String,
) -> Result<(), Error> {
    let options = map_options(options, doc_path, output_directory);
    let docgen = Docgen::new(env, &options);
    for (file, content) in docgen.gen() {
        let path = PathBuf::from(&file);
        fs::create_dir_all(path.parent().unwrap())?;
        fs::write(path.as_path(), content)?;
    }

    if env.has_errors() {
        let mut error_writer = Buffer::no_color();
        env.report_diag(&mut error_writer, Severity::Warning);
        println!("{}", String::from_utf8_lossy(&error_writer.into_inner()));
        Err(anyhow!("exiting with documentation generation errors"))
    } else {
        Ok(())
    }
}

fn map_options(
    options: &DocgenOptions,
    doc_path: Vec<String>,
    output_directory: String,
) -> DiemDocgenOptions {
    DiemDocgenOptions {
        section_level_start: options.section_level_start,
        include_private_fun: options.include_private_fun,
        include_specs: options.include_specs,
        specs_inlined: options.specs_inlined,
        include_impl: options.include_impl,
        toc_depth: options.toc_depth,
        collapsed_sections: options.collapsed_sections,
        output_directory,
        doc_path,
        root_doc_templates: options.root_doc_templates.clone(),
        references_file: options.references_file.clone(),
        include_dep_diagrams: options.include_dep_diagrams,
        include_call_diagrams: options.include_call_diagrams,
    }
}
