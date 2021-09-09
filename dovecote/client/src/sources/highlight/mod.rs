#[cfg(target_arch = "wasm32")]
use crate::console_log;

use super::Error;

pub mod mov;
pub mod toml;

pub fn mark_code<'input, M: Marker + Default>(id: &str, code: &'input str) -> Vec<Line<'input>> {
    let mut lines = vec![];

    let mut marcker = M::default();
    for (i, line) in code.lines().enumerate() {
        match marcker.mark_line(line) {
            Ok(line) => {
                lines.push(line);
            }
            Err(err) => {
                marcker.reset();
                lines.push(Line {
                    items: vec![(StyleType::Normal, line)],
                });
                #[cfg(target_arch = "wasm32")]
                console_log!(
                    "Highlighting error: line:{}:{:?}. File id:{}",
                    i + 1,
                    err,
                    id
                );
                #[cfg(not(target_arch = "wasm32"))]
                println!(
                    "Highlighting error: line:{}:{:?}. File id:{}",
                    i + 1,
                    err,
                    id
                );
            }
        }
    }

    lines
}

#[derive(Debug, Eq, PartialEq)]
pub struct Line<'input> {
    pub items: Vec<(StyleType, &'input str)>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum StyleType {
    KeyWords,
    Normal,
    Number,
    Comment,
    String,
    Var,
    Space,
}

impl StyleType {
    pub fn as_style_name(&self) -> &str {
        match self {
            StyleType::KeyWords => "cs5",
            StyleType::Normal => "cs19",
            StyleType::Number => "cs6",
            StyleType::Comment => "cs4",
            StyleType::String => "cs11",
            StyleType::Var => "cs9",
            StyleType::Space => "cs19",
        }
    }
}

pub trait Marker {
    fn reset(&mut self) {}

    fn mark_line<'input>(&mut self, line: &'input str) -> Result<Line<'input>, Error>;
}
