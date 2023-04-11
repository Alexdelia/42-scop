use crate::prelude::*;

use super::{Keyword, Token};
use crate::parse::FileData;

pub struct Format(Vec<Token>);

pub enum FormatError {
	WrongNumberOfToken {
		expected: usize,
		got: usize,
	},
	WrongToken {
	},
}


impl Format {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self(tokens)
    }

    pub fn check(&self, line: &str) -> Result<Vec<String>> {
		let mut split: Vec<String> = line
			.split_whitespace()
			.into_iter()
			.map(|s| s.to_string())
			.collect();

		if split.len() != self.0.len() {

    }
}

fn check_line_format(
    f: &FileData,
    k: &Keyword,
    ef: &ExpectedFormat,
    line: &str,
    line_index: usize,
) -> Result<Vec<String>> {
    let mut split: Vec<String> = line
        .split_whitespace()
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    if split.len() != ef.size + 1 {
        let unprocessed_line = f.content[line_index].clone();
        return Err(pfe!(
            f!("expected {B}{G}{expected_size}{D} token after {B}{Y}{keyword}{D}, found {B}{R}{got_size}{D}",
                expected_size=ef.size,
                got_size=split.len() - 1,
                keyword=k.keyword
            ),
            h:f!(
"{B}{Y}{keyword}{D} define {desc}
the line must follow the format: `{B}{keyword} {M}{format}{D}`",
                keyword=k.keyword,
                desc=k.desc,
                format=ef.format
            ),
            f:f.name.clone(),
            l:ple!(unprocessed_line, i:line_index, w:pwe!((0, unprocessed_line.len())))
        ))?;
    }

    split.remove(0);
    Ok(split)
}

pub struct ExpectedFormat {
    pub format: String, // format without keyword
    pub size: usize,    // number of tokens in format	(without keyword)
}

impl ExpectedFormat {
    pub fn new(keyword: &'static str, expected_format: Option<impl Into<String>>) -> Self {
        let Some(expected_format) = expected_format else {
			return Self {
				format: String::new(),
				size: 0,
			};
		};

        let mut format: String = expected_format.into();
        if format.starts_with(keyword) {
            let format = format.replacen(keyword, "", 1).trim().to_string();
        }

        let size = format
            .split_whitespace()
            .into_iter()
            .collect::<Vec<&str>>()
            .len();

        Self { format, size }
    }
}
