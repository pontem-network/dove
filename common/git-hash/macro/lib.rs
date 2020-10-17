/*!
    Utility proc-macro crate for get current git HEAD hash as constant `Option<&'static str>`.

    ## Usage example:

    ```rust
    // main.rs
    extern crate git_hash_proc_macro as git_hash;

    fn main() {
        println!("{:?}", git_hash::git_hash!());
        println!("{:?}", git_hash::git_hash_short!());
    }
    ```
*/

extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro::LexError;

// Mod `cmd` used in this and parrent crate.
#[path = "../src/cmd.rs"]
mod cmd;

#[proc_macro]
pub fn git_hash(_: TokenStream) -> TokenStream {
    let git_hash = cmd::git_hash();
    opt_tokenize(git_hash).unwrap()
}

#[proc_macro]
pub fn git_hash_short(_: TokenStream) -> TokenStream {
    let git_hash = cmd::git_hash_short();
    opt_tokenize(git_hash).unwrap()
}

#[proc_macro]
pub fn crate_version_with_git_hash(_: TokenStream) -> TokenStream {
    crate_version_with(cmd::git_hash()).unwrap()
}

#[proc_macro]
pub fn crate_version_with_git_hash_short(_: TokenStream) -> TokenStream {
    crate_version_with(cmd::git_hash_short()).unwrap()
}

fn crate_version_with(s: Option<String>) -> Result<TokenStream, LexError> {
    let res = s
        .map(|rev| format!("-{}", rev))
        .unwrap_or_else(Default::default);

    format!("concat!(env!(\"CARGO_PKG_VERSION\"), \"{}\")", res).parse()
}

fn opt_tokenize(s: Option<String>) -> Result<TokenStream, LexError> {
    s.map(|s| format!("Some(\"{}\")", s))
        .unwrap_or_else(|| "None".to_owned())
        .parse()
}
