use crate::prelude::*;

use super::FileData;
use crate::setting::OBJ_PATH;

use ansi::abbrev::{B, BLU, D, G, M, R, Y};

use std::path::PathBuf;

pub fn get_material(f: &FileData, mtl_path: &Vec<PathBuf>) -> Result<Option<PathBuf>> {
    if let Some((i, s)) = get_material_line(f)? {
        let no_comment_size = s.len();

        let mut path = PathBuf::from(OBJ_PATH);
        path.push(s);

        if !mtl_path.contains(&path) {
            return Err(pfe!(
                f!("cannot find {B}{Y}{}{D} for {B}{M}{}{D}", path.display(), f.name),
                h:f!("make sure you have a {B}{G}valid .mtl{D} file in {B}{M}{OBJ_PATH}{D}"),
                f:f.name.clone(),
                l:ple!(f.content[i].clone(), i:i, w:pwe!((0, no_comment_size)))
            ))?;
        }

        return Ok(Some(path));
    }

    Ok(None)
}

fn get_material_line(f: &FileData) -> Result<Option<(usize, String)>> {
    let mut ret: Option<(usize, String)> = None;

    for (i, line) in f.diluted.iter().enumerate() {
        if line.starts_with("usemtl") {
            let mut split: Vec<&str> = line.split_whitespace().into_iter().collect();

            if split.len() != 2 {
                return Err(pfe!(
                    if split.len() < 2 {
                        "expected file after {B}{Y}usemtl{D}"
                    } else {
                        "expected only one file after {B}{Y}usemtl{D}"
                    },
                    h:"{B}{Y}usemtl{D} define the {B}{BLU}.mlt{D} file to use\nthe line must follow the format: `{B}usemtl {M}file.mtl{D}`",
                    f:f.name.clone(),
                    l:ple!(f.content[i].clone(), i:i, w:pwe!((0, line.len())))
                ))?;
            }

            if ret.is_none() {
                ret = Some((i, split[1].to_string()));
            } else {
                warn!("does not support multiple {B}{Y}`usemtl`{D}");
            }
        }
    }

    Ok(ret)
}

pub fn get_line_u(
    f: &FileData,
    keyword: &str,
    expected_format: Option<impl Into<String>>,
    keyword_def_desc: &str,
) -> Result<Option<(usize, Vec<String>)>> {
    let (ef, ef_size) = format_ef(keyword, expected_format);
    let mut ret: Option<(usize, Vec<String>)> = None;

    for (i, line) in f.diluted.iter().enumerate() {
        if line.starts_with(keyword) {
            let token =
                check_line_format(f, (keyword, keyword_def_desc), (&ef, ef_size), (line, i))?;

            if ret.is_none() {
                ret = Some((i, token));
            } else {
                warn!("does not support multiple {B}{Y}{keyword}{D} definition");
            }
        }
    }

    return Ok(ret);
}

fn format_ef(keyword: &str, expected_format: Option<impl Into<String>>) -> (String, usize) {
    let Some(expected_format) = expected_format else {
		return (String::new(), 0);
	};

    let mut ef: String = expected_format.into();
    if ef.starts_with(keyword) {
        let ef = ef.replacen(keyword, "", 1).trim().to_string();
    }

    let ef_size = ef
        .split_whitespace()
        .into_iter()
        .collect::<Vec<&str>>()
        .len()
        + 1;

    (ef, ef_size)
}

fn check_line_format(
    f: &FileData,
    keyword: (&str, &str),
    ef: (&str, usize),
    line: (&str, usize),
) -> Result<Vec<String>> {
    let mut split: Vec<String> = line
        .0
        .split_whitespace()
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    if split.len() != ef.1 {
        let unprocessed_line = f.content[line.1].clone();
        return Err(pfe!(
            f!("expected {B}{G}{expected_size}{D} token after {B}{Y}{keyword}{D}, found {B}{R}{got_size}{D}",
                expected_size=ef.1 - 1,
                got_size=split.len() - 1,
                keyword=keyword.0
            ),
            h:f!(
"{B}{Y}{keyword}{D} define {desc}
the line must follow the format: `{B}{keyword} {M}{format}{D}`",
                keyword=keyword.0,
                desc=keyword.1,
                format=ef.0
            ),
            f:f.name.clone(),
            l:ple!(unprocessed_line, i:line.1, w:pwe!((0, unprocessed_line.len())))
        ))?;
    }

    split.remove(0);
    Ok(split)
}
