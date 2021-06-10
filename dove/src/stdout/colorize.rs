use anyhow::{Result, Error};
use std::io::Write;
use std::sync::Mutex;
use once_cell::sync::OnceCell;
use termcolor::{ColorSpec, Color, BufferWriter, WriteColor, ColorChoice};

/// Marker to highlight; WHEN is 'always', 'never', or 'auto'
static STDOUT_COLOR: OnceCell<Mutex<Box<ColorChoice>>> = OnceCell::new();

/// get marker to highlight
fn get_colorchoise_color() -> ColorChoice {
    match STDOUT_COLOR.get() {
        Some(mt) => *(mt.lock().expect("Couldn't get STDOUT_COLOR")).clone(),
        None => ColorChoice::Never,
    }
}

/// set by flag --color=.. auto|always|never|ansi and stdout
pub fn set_colorchoise_for_stdout() -> Result<(), Error> {
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
            if atty::is(atty::Stream::Stdout) {
                ColorChoice::Auto
            } else {
                ColorChoice::Never
            }
        }
        _ => ColorChoice::Never,
    };
    set_colorchoise(color_flag)
}

/// STDOUT_COLOR = ColorChoise::Never
pub fn set_colorchoise_never() -> Result<(), Error> {
    set_colorchoise(ColorChoice::Never)
}

/// set print to stdout
fn set_colorchoise(color: ColorChoice) -> Result<(), Error> {
    STDOUT_COLOR.set(Mutex::new(Box::new(color))).or_else(|mt| {
        let mut bx = mt
            .lock()
            .map_err(|_| anyhow!("couldn't get access to STDOUT_STREAM"))?;
        *bx = Box::new(color);
        Ok(())
    })
}

/// colorize text for output
pub fn colorize_text(text: &str, spec: &mut ColorSpec) -> String {
    let mut buffer = BufferWriter::stdout(get_colorchoise_color()).buffer();
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
