use once_cell::sync::OnceCell;
use std::fmt::Write as FmtWriter;
use std::io::{Write, Stdout, stdout};
use std::sync::Mutex;
use anyhow::{Result, Error};

/// colorize text for output
pub mod colorize;
/// Print output.
pub mod print;

/// Stdout buffer for prints
static STDOUT_STREAM: OnceCell<Mutex<Box<dyn BufWrite + Send>>> = OnceCell::new();

/// Get data from the buffer, if a string was used, and clear the buffer
pub fn get_buffer_value_and_erase() -> Option<String> {
    STDOUT_STREAM
        .get()
        .and_then(|mt| mt.lock().map_or(None, |mut bx| bx.get_value_and_erase()))
}

/// set print to stdout
pub fn set_print_to_stdout() {
    STDOUT_STREAM
        .set(Mutex::new(Box::new(stdout())))
        .map_or_else(
            |mt| {
                let mut ou = mt
                    .lock()
                    .map_err(|_| anyhow!("couldn't get access to STDOUT_STREAM"))?;
                *ou = Box::new(stdout());
                Ok(())
            },
            |_| Ok(()),
        )
        .and(colorize::set_colorchoise_for_stdout())
        .expect("failed set print to stdout");
}

/// set print to string.
pub fn set_print_to_string() {
    STDOUT_STREAM
        .set(Mutex::new(Box::new(String::new())))
        .map_or_else(
            |mt| {
                let mut ou = mt
                    .lock()
                    .map_err(|_| anyhow!("couldn't get access to STDOUT_STREAM"))?;
                *ou = Box::new(String::new());
                Ok(())
            },
            |_| Ok(()),
        )
        .and(colorize::set_colorchoise_never())
        .expect("failed set print to string");
}

/// The stream / buffer to write
trait BufWrite {
    /// Write to buffer or stream
    fn print(&mut self, text: &str) -> Result<(), Error>;
    /// Get data from the buffer, if a string was used, and clear the buffer
    fn get_value_and_erase(&mut self) -> Option<String> {
        None
    }
}

impl BufWrite for Stdout {
    /// Write to stream
    fn print(&mut self, text: &str) -> Result<(), Error> {
        self.write(text.as_bytes())
            .map_or_else(|err| Err(anyhow!(err.to_string())), |_| Ok(()))
    }
}

impl BufWrite for String {
    /// Write to buffer
    fn print(&mut self, text: &str) -> Result<(), Error> {
        self.write_str(text)
            .map_or_else(|err| Err(anyhow!(err)), |_| Ok(()))
    }

    /// Get data from the buffer and clear the buffer
    fn get_value_and_erase(&mut self) -> Option<String> {
        let buff = self.clone();
        *self = String::new();
        Some(buff)
    }
}
