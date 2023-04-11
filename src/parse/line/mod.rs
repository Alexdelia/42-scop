mod get_line;
pub use get_line::get_line;
mod keyword;
pub use keyword::Keyword;
mod format;
pub use format::Format;
mod occurence;
pub use occurence::Occurence;

use crate::prelude::*;

use crate::parse::FileData;

use ansi::abbrev::{B, D, G, M, R, Y};

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
        let line = f.diluted[line_index].replacen(self.k.keyword, "", 1).trim();
        let token = self.format.check(line).map_err(|(expected, got)| {
            pfe!(
                f!("expected {B}{G}{expected}{D} token after {B}{Y}{keyword}{D}, got {B}{R}{got}{D}",
					keyword=self.k.keyword,
			),
                h:f!(
"{B}{Y}{keyword}{D} define {desc}
the line must follow the format: `{B}{keyword} {M}{format}{D}`",
					keyword=self.k.keyword,
					desc=self.k.desc,
					format=self.format.token,
				),
                f:f.name,
                l:ple!(line, i:line_index, w:pwe!((0, line.len())))
            )
        })?;
        Ok(token)
    }
}
