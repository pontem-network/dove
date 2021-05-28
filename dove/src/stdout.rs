use once_cell::sync::OnceCell;
use std::fmt::Write as FmtWriter;
use std::io::{Write, Stdout};
use std::sync::Mutex;
use anyhow::Error;

/// Stdout buffer for prints
static STDOUT_STREAM: OnceCell<Mutex<Box<dyn TBufWrite + Send>>> = OnceCell::new();
/// The stream / buffer to write
pub trait TBufWrite {
    /// Write to buffer or stream
    fn print(&mut self, text: &str) -> Result<(), Error>;
    /// Get data from the buffer if a string was used
    fn value(&self) -> Option<String>;
}
impl TBufWrite for Stdout {
    /// Write to buffer or stream
    fn print(&mut self, text: &str) -> Result<(), Error> {
        self.write(text.as_bytes())
            .map_or_else(|err| Err(anyhow!(err.to_string())), |_| Ok(()))
    }
    /// Get data from the buffer if a string was used
    fn value(&self) -> Option<String> {
        None
    }
}
impl TBufWrite for String {
    /// Write to buffer or stream
    fn print(&mut self, text: &str) -> Result<(), Error> {
        self.write_str(text)
            .map_or_else(|err| Err(anyhow!(err)), |_| Ok(()))
    }
    /// Get data from the buffer if a string was used
    fn value(&self) -> Option<String> {
        Some(self.clone())
    }
}

/// Prints to the output.
pub fn print(text: &str) -> Result<(), Error> {
    STDOUT_STREAM
        .get()
        .unwrap()
        .lock()
        .unwrap()
        .as_mut()
        .print(text)
}
/// Get data from the buffer if a string was used
pub fn get_buffer_value() -> Option<String> {
    STDOUT_STREAM
        .get()
        .and_then(|mt| mt.lock().map_or(None, |bx| bx.value()))
}
/// Set the stream / buffer to write
pub fn set_buffer<Out>(stdout: Out) -> Result<(), Error>
where
    Out: TBufWrite + Send,
    Out: 'static,
{
    match STDOUT_STREAM.get() {
        Some(mt) => {
            let mut buff = mt.lock().unwrap();
            *buff = Box::new(stdout);
            Ok(())
        }
        None => STDOUT_STREAM
            .set(Mutex::new(Box::new(stdout)))
            .map_or_else(|_| Err(anyhow!("Failed to set buffer output")), |_| Ok(())),
    }
}
/// Prints to the output.
#[macro_export]
macro_rules! stdout {
    ($($args:tt)*) => {
        $crate::stdout::print(&format!($($args)*)).unwrap();
    };
}

/// Prints to the output, with a newline.
#[macro_export]
macro_rules! stdoutln {
    () => ($crate::stdout::print!("\n").unwrap());
    ($($args:tt)*) => {
        $crate::stdout::print(&format!($($args)*))
            .and($crate::stdout::print("\n")).unwrap();
    }
}

#[test]
fn test_stdout_string_buff() {
    set_buffer(String::new()).unwrap();
    stdout!("test value");
    assert_eq!(Some("test value".to_string()), get_buffer_value());
}
#[test]
fn test_stdoutln_string_buff() {
    set_buffer(String::new()).unwrap();
    stdoutln!("test value");
    assert_eq!(Some("test value\n".to_string()), get_buffer_value());
}
