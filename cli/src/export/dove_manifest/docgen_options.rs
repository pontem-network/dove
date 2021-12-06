// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0
use serde::{Deserialize, Serialize};

/// Docgen options.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct DocgenOptions {
    /// Whether to run the documentation generator.
    #[serde(default = "enabled")]
    pub enabled: bool,
    /// The level where we start sectioning. Often markdown sections are rendered with
    /// unnecessary large section fonts, setting this value high reduces the size.
    #[serde(default = "section_level_start")]
    pub section_level_start: usize,
    /// Whether to include private functions in the generated docs.
    #[serde(default = "include_private_fun")]
    pub include_private_fun: bool,
    /// Whether to include specifications in the generated docs.
    #[serde(default = "include_specs")]
    pub include_specs: bool,
    /// Whether to put specifications in the same section as a declaration or put them all
    /// into an independent section.
    #[serde(default = "specs_inlined")]
    pub specs_inlined: bool,
    /// Whether to include Move implementations.
    #[serde(default = "include_impl")]
    pub include_impl: bool,
    /// Max depth to which sections are displayed in table-of-contents.
    #[serde(default = "toc_depth")]
    pub toc_depth: usize,
    /// Whether to use collapsed sections (<details>) for impl and specs
    #[serde(default = "collapsed_sections")]
    pub collapsed_sections: bool,
    /// A list of paths to files containing templates for root documents for the generated
    /// documentation.
    ///
    /// A root document is a markdown file which contains placeholders for generated
    /// documentation content. It is also processed following the same rules than
    /// documentation comments in Move, including creation of cross-references and
    /// Move code highlighting.
    ///
    /// A placeholder is a single line starting with a markdown quotation marker
    /// of the following form:
    ///
    /// ```notrust
    /// > {{move-include NAME_OF_MODULE_OR_SCRIPT}}
    /// > {{move-toc}}
    /// > {{move-index}}
    /// ```
    ///
    /// These lines will be replaced by the generated content of the module or script,
    /// or a table of contents, respectively.
    ///
    /// For a module or script which is included in the root document, no
    /// separate file is generated. References between the included and the standalone
    /// module/script content work transparently.
    #[serde(default)]
    pub root_doc_templates: Vec<String>,
    /// An optional file containing reference definitions. The content of this file will
    /// be added to each generated markdown doc.
    #[serde(default)]
    pub references_file: Option<String>,
    /// Whether to include dependency diagrams in the generated docs.
    #[serde(default = "include_dep_diagrams")]
    pub include_dep_diagrams: bool,
    /// Whether to include call diagrams in the generated docs.
    #[serde(default = "include_call_diagrams")]
    pub include_call_diagrams: bool,
}

fn enabled() -> bool {
    true
}

fn section_level_start() -> usize {
    1
}

fn include_private_fun() -> bool {
    true
}

fn include_specs() -> bool {
    true
}

fn specs_inlined() -> bool {
    true
}

fn include_impl() -> bool {
    true
}

fn toc_depth() -> usize {
    3
}

fn collapsed_sections() -> bool {
    true
}

fn include_dep_diagrams() -> bool {
    false
}

fn include_call_diagrams() -> bool {
    false
}

impl Default for DocgenOptions {
    fn default() -> Self {
        Self {
            enabled: enabled(),
            section_level_start: section_level_start(),
            include_private_fun: include_private_fun(),
            include_specs: include_specs(),
            specs_inlined: specs_inlined(),
            include_impl: include_impl(),
            toc_depth: toc_depth(),
            collapsed_sections: collapsed_sections(),
            root_doc_templates: vec![],
            references_file: None,
            include_dep_diagrams: include_dep_diagrams(),
            include_call_diagrams: include_call_diagrams(),
        }
    }
}
