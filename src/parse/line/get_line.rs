use crate::prelude::*;

use super::{ExpectedLine, Keyword, Occurence, Token};
use crate::parse::FileData;

use ansi::abbrev::{B, D, G, M, R, Y};

type ParsedLine = (Vec<String>, usize); // (tokens, line_index)

type LineToken = (Vec<String>, usize); // (tokens, line_index)

pub fn get_line(f: &FileData, el: &ExpectedLine) -> Result<Vec<ParsedLine>> {
    let mut ret: Vec<ParsedLine> = Vec::new();

    for (i, line) in f.diluted.iter().enumerate() {
        if line.starts_with(el.k.keyword) {
            ret.push((el.check(f, line, i)?, i));
        }
    }

    Ok(ret)
}

pub fn get_line_u(
    f: &FileData,
    k: &Keyword,
    expected_format: Option<impl Into<String>>,
) -> Result<Option<LineToken>> {
    let ef = ExpectedFormat::new(k.keyword, expected_format);
    let mut ret: Option<LineToken> = None;

    for (i, line) in f.diluted.iter().enumerate() {
        if line.starts_with(k.keyword) {
            let token = check_line_format(f, k, &ef, line, i)?;

            if ret.is_none() {
                ret = Some((token, i));
            } else {
                warn!(
                    "does not support multiple {B}{Y}{}{D} definition",
                    k.keyword
                );
            }
        }
    }

    return Ok(ret);
}

pub fn get_line_m(
    f: &FileData,
    k: &Keyword,
    expected_format: Option<impl Into<String>>,
) -> Result<Vec<LineToken>> {
    let ef = ExpectedFormat::new(k.keyword, expected_format);
    let mut ret: Vec<LineToken> = Vec::new();

    for (i, line) in f.diluted.iter().enumerate() {
        if line.starts_with(k.keyword) {
            let token = check_line_format(f, k, &ef, line, i)?;
            ret.push((token, i));
        }
    }

    Ok(ret)
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
