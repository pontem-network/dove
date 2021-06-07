use once_cell::sync::OnceCell;
use std::fmt::Write as FmtWriter;
use std::io::{Write, Stdout};
use std::sync::Mutex;
use anyhow::Error;

/// colorize text for output
pub mod colorize;
/// Print output.
pub mod print;

/// Stdout buffer for prints
static STDOUT_STREAM: OnceCell<Mutex<Box<dyn BufWrite + Send>>> = OnceCell::new();

/// Get data from the buffer if a string was used
pub fn get_buffer_value() -> Option<String> {
    STDOUT_STREAM
        .get()
        .and_then(|mt| mt.lock().map_or(None, |bx| bx.value()))
}

/// Set the stream / buffer to write
pub fn set_buffer<Out>(stdout: Out) -> Result<(), Error>
where
    Out: BufWrite + Send,
    Out: 'static,
{
    colorize::set_stdout_by_argv_flag_color(&stdout)?;

    match STDOUT_STREAM.get() {
        Some(mt) => {
            let mut buff = mt.lock().unwrap();
            *buff = Box::new(stdout);
            Ok(())
        }
        None => {
            STDOUT_STREAM
                .set(Mutex::new(Box::new(stdout)))
                .map_err(|_| anyhow!("Failed to set STDOUT_COLOR"))?;
            Ok(())
        }
    }
}

/// The stream / buffer to write
pub trait BufWrite {
    /// Write to buffer or stream
    fn print(&mut self, text: &str) -> Result<(), Error>;
    /// Get data from the buffer if a string was used
    fn value(&self) -> Option<String>;
    /// is stdout
    fn is_stdout(&self) -> bool;
}

impl BufWrite for Stdout {
    /// Write to buffer or stream
    fn print(&mut self, text: &str) -> Result<(), Error> {
        self.write(text.as_bytes())
            .map_or_else(|err| Err(anyhow!(err.to_string())), |_| Ok(()))
    }
    /// Get data from the buffer if a string was used
    fn value(&self) -> Option<String> {
        None
    }
    /// is_stdout
    fn is_stdout(&self) -> bool {
        true
    }
}

impl BufWrite for String {
    /// Write to buffer or stream
    fn print(&mut self, text: &str) -> Result<(), Error> {
        self.write_str(text)
            .map_or_else(|err| Err(anyhow!(err)), |_| Ok(()))
    }
    /// Get data from the buffer if a string was used
    fn value(&self) -> Option<String> {
        Some(self.clone())
    }
    /// is stdout
    fn is_stdout(&self) -> bool {
        false
    }
}
