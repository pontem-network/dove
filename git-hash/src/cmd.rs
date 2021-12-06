use std::{process::Command, ffi::OsStr};

/// Returns the current git (HEAD) commit SHA.
pub fn git_hash() -> Option<String> {
    run_git(&["rev-parse", "HEAD"])
}

/// Returns the current git (HEAD) commit SHA as shorten view.
pub fn git_hash_short() -> Option<String> {
    run_git(&["rev-parse", "--short", "HEAD"])
}

/// Run git
fn run_git<I, S>(args: I) -> Option<String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    Command::new("git")
        .args(args)
        .output()
        .map(|output| {
            let s = String::from_utf8_lossy(&output.stdout);
            Some(s.trim().to_string())
        })
        .unwrap_or(None)
}

#[cfg(test)]
mod tests {
    #[test]
    fn git_hash() {
        let git_hash = super::git_hash().unwrap();
        assert_eq!(git_hash.len(), 40);
    }

    #[test]
    fn git_hash_short() {
        let git_hash = super::git_hash_short().unwrap();
        assert!(git_hash.len() <= 40);
    }
}
