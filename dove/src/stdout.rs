use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};
use once_cell::sync::OnceCell;
use std::fmt::Write as FmtWriter;
use std::io::{Write, Stdout};
use std::sync::Mutex;
use anyhow::Error;

/// Stdout buffer for prints
static STDOUT_STREAM: OnceCell<Mutex<Box<dyn TBufWrite + Send>>> = OnceCell::new();
/// Marker to highlight; WHEN is 'always', 'never', or 'auto'
static STDOUT_COLOR: OnceCell<Mutex<Box<ColorChoice>>> = OnceCell::new();

/// Print output.
pub fn print(text: &str) -> Result<(), Error> {
    STDOUT_STREAM
        .get()
        .unwrap()
        .lock()
        .unwrap()
        .as_mut()
        .print(text)
}

/// Print output with a newline.
pub fn println(text: &str) -> Result<(), Error> {
    print(text)?;
    print("\n")
}

/// print output in green color
pub fn print_good(text: &str) -> Result<(), Error> {
    print_with_color(text, ColorSpec::new().set_fg(Some(Color::Green)))
}

/// print output in red color
pub fn print_error(text: &str) -> Result<(), Error> {
    print_with_color(text, ColorSpec::new().set_fg(Some(Color::Red)))
}

/// print output in yellow color
pub fn print_warning(text: &str) -> Result<(), Error> {
    print_with_color(text, ColorSpec::new().set_fg(Some(Color::Yellow)))
}

/// print output with color settings
fn print_with_color(text: &str, spec: &mut ColorSpec) -> Result<(), Error> {
    let mut buffer = BufferWriter::stdout(get_stdout_color()).buffer();
    buffer.set_color(spec)?;
    write!(&mut buffer, "{}", text)?;
    buffer.reset()?;
    print(&String::from_utf8_lossy(buffer.as_slice()))
}

/// Prints output.
#[macro_export]
macro_rules! stdout {
    ($($args:tt)*) => {
        $crate::stdout::print(&format!($($args)*)).unwrap();
    };
}

/// Prints output with a newline.
#[macro_export]
macro_rules! stdoutln {
    () => ($crate::stdout::print!("\n").unwrap());
    ($($args:tt)*) => {
        $crate::stdout::println(&format!($($args)*)).unwrap();
    }
}

/// Set the stream / buffer to write
pub fn set_buffer<Out>(stdout: Out) -> Result<(), Error>
where
    Out: TBufWrite + Send,
    Out: 'static,
{
    let flag = get_argv_flag_color(&stdout);

    match STDOUT_COLOR.get() {
        Some(mt) => {
            let mut buff = mt.lock().unwrap();
            *buff = Box::new(flag);
            Ok(())
        }
        None => STDOUT_COLOR
            .set(Mutex::new(Box::new(flag)))
            .map_or_else(|_| Err(anyhow!("Failed to set STDOUT_COLOR")), |_| Ok(())),
    }?;

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

/// Get data from the buffer if a string was used
pub fn get_buffer_value() -> Option<String> {
    STDOUT_STREAM
        .get()
        .and_then(|mt| mt.lock().map_or(None, |bx| bx.value()))
}

/// get marker to highlight
fn get_stdout_color() -> ColorChoice {
    match STDOUT_COLOR.get() {
        Some(mt) => *(mt.lock().unwrap()).clone(),
        None => ColorChoice::Auto,
    }
}

/// get flag --color=.. auto|always|never|ansi
fn get_argv_flag_color<Out>(stdout: &Out) -> ColorChoice
where
    Out: TBufWrite + Send,
    Out: 'static,
{
    let flag = std::env::args()
        .find_map(|it| {
            if it.to_lowercase().contains("--color=") {
                Some((&it[8..]).to_lowercase())
            } else {
                None
            }
        })
        .unwrap_or_else(|| "auto".to_string());

    match flag.as_str() {
        "always" => ColorChoice::Always,
        "ansi" => ColorChoice::AlwaysAnsi,
        "auto" => {
            if stdout.is_stdout() && atty::is(atty::Stream::Stdout) {
                ColorChoice::Auto
            } else {
                ColorChoice::Never
            }
        }
        _ => ColorChoice::Never,
    }
}

/// The stream / buffer to write
pub trait TBufWrite {
    /// Write to buffer or stream
    fn print(&mut self, text: &str) -> Result<(), Error>;
    /// Get data from the buffer if a string was used
    fn value(&self) -> Option<String>;
    /// is stdout
    fn is_stdout(&self) -> bool;
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
    /// is_stdout
    fn is_stdout(&self) -> bool {
        true
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
    /// is stdout
    fn is_stdout(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::stdout::{set_buffer, get_buffer_value, print_good};

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

    #[test]
    fn test_stdoutln_string_buff_with_color() {
        set_buffer(String::new()).unwrap();
        print_good("test value").unwrap();
        assert_eq!(Some("test value".to_string()), get_buffer_value());
    }
}
