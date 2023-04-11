mod keyword;
pub use keyword::Keyword;
mod format;
pub use format::Format;
mod occurence;
pub use occurence::Occurence;

use crate::prelude::*;

use crate::parse::FileData;

use ansi::abbrev::{B, D, G, M, R, Y};

type ParsedLine = (Vec<String>, usize); // (tokens, line_index)

pub fn get_line(f: &FileData, el: &ExpectedLine) -> Result<Vec<ParsedLine>> {
    let mut ret: Vec<ParsedLine> = Vec::new();

    for i in 0..f.diluted.len() {
        if f.diluted[i].starts_with(el.k.keyword) {
            ret.push((el.check(f, i)?, i));
        }
    }

    match el.occurence.check(ret.len()) {
        Ok(_) => Ok(ret),
        Err(e) => Err(pfe!(
            f!("{B}{Y}{}{D} {e}", el.k.keyword),
            h:el.help(),
            f:f.name,
        ))?,
    }
}

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
        match self.format.check(line).map_err(|(expected, got)| {
            pfe!(
                f!("expected {B}{G}{expected}{D} token after {B}{Y}{keyword}{D}, got {B}{R}{got}{D}",
					keyword=self.k.keyword,
			),
                h:self.help(),
                f:f.name,
                l:ple!(line, i:line_index, w:pwe!((0, line.len())))
            )
        }) {
			Ok(token) => Ok(token),
			Err(e) => Err(e)?,
		}
    }

    pub fn help(&self) -> String {
        f!(
            "{B}{Y}{keyword}{D} define {desc}
the line must follow the format: `{B}{keyword} {M}{format}{D}`",
            keyword = self.k.keyword,
            desc = self.k.desc,
            format = self.format.token,
        )
    }
}
