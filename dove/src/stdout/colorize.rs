use std::io::Write;
use std::sync::Mutex;
use anyhow::Error;
use once_cell::sync::OnceCell;
use termcolor::{ColorSpec, Color, BufferWriter, WriteColor, ColorChoice};
use crate::stdout::BufWrite;

/// Marker to highlight; WHEN is 'always', 'never', or 'auto'
static STDOUT_COLOR: OnceCell<Mutex<Box<ColorChoice>>> = OnceCell::new();

/// get marker to highlight
fn get_stdout_color() -> ColorChoice {
    match STDOUT_COLOR.get() {
        Some(mt) => *(mt.lock().expect("Couldn't get STDOUT_COLOR")).clone(),
        None => ColorChoice::Auto,
    }
}

/// get flag --color=.. auto|always|never|ansi
pub fn set_stdout_by_argv_flag_color<Out>(stdout: &Out) -> Result<(), Error>
where
    Out: BufWrite + Send,
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

    let color_flag = match flag.as_str() {
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
    };
    match STDOUT_COLOR.get() {
        Some(mt) => {
            let mut buff = mt.lock().expect("Couldn't get STDOUT_COLOR");
            *buff = Box::new(color_flag);
            Ok(())
        }
        None => STDOUT_COLOR
            .set(Mutex::new(Box::new(color_flag)))
            .map_or_else(|_| Err(anyhow!("Failed to set STDOUT_COLOR")), |_| Ok(())),
    }
}

/// colorize text for output
pub fn colorize_text(text: &str, spec: &mut ColorSpec) -> String {
    let mut buffer = BufferWriter::stdout(get_stdout_color()).buffer();
    buffer
        .set_color(spec)
        .and(write!(&mut buffer, "{}", text))
        .and(buffer.reset())
        .expect("Couldn't color the text");
    String::from_utf8_lossy(buffer.as_slice()).to_string()
}

/// green color text
pub fn good(text: &str) -> String {
    colorize_text(text, ColorSpec::new().set_fg(Some(Color::Green)))
}

/// red color text
pub fn error(text: &str) -> String {
    colorize_text(text, ColorSpec::new().set_fg(Some(Color::Red)))
}

/// yellow color text
pub fn warning(text: &str) -> String {
    colorize_text(text, ColorSpec::new().set_fg(Some(Color::Yellow)))
}
