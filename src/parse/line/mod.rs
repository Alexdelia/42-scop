mod get_line;
pub use get_line::{get_line_m, get_line_u};
mod keyword;
pub use keyword::Keyword;
mod token;
pub use token::Token;
mod format;
pub use format::Format;

use crate::prelude::*;

use crate::parse::FileData;

pub struct ExpectedLine {
    pub k: Keyword,
    pub format: Format,
    pub occurence: Occurence,
}

#[derive(Default)]
pub enum Occurence {
    #[default]
    Once,
    N(usize),
    Multiple,
}

impl ExpectedLine {
    pub fn new(k: Keyword, format: Format, occurence: Occurence) -> Self {
        Self {
            k,
            format,
            occurence,
        }
    }

    pub fn check(&self, f: &FileData, line: &str, line_index: usize) -> Result<Vec<String>> {
        todo!();
        // self.format.check(f, line, line_index)
    }
}
