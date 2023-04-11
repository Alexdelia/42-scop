mod get_line;
pub use get_line::get_line;
mod keyword;
pub use keyword::Keyword;
mod token;
pub use token::Token;
mod format;
pub use format::Format;
mod occurence;
pub use occurence::Occurence;

use crate::prelude::*;

use crate::parse::FileData;

pub struct ExpectedLine {
    pub k: Keyword,
    pub format: Format,
    pub occurence: Occurence,
}

impl ExpectedLine {
    pub fn new(k: Keyword, format: Format, occurence: Occurence) -> Self {
        Self {
            k,
            format,
            occurence,
        }
    }

    pub fn check(&self, f: &FileData, line_index: usize) -> Result<Vec<String>> {
        todo!();
        let line = f.diluted[line_index].replacen(self.k.keyword, "", 1).trim();
        // self.format(line).map_err();
    }
}
