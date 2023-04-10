use crate::prelude::*;

use super::FileData;
use crate::setting::OBJ_PATH;

use ansi::abbrev::{B, D, Y};

use std::path::PathBuf;

pub fn get_material(f: &FileData, mtl_path: &Vec<PathBuf>) -> Result<Option<PathBuf>> {
    if let Some((i, s)) = get_material_line(f)? {
        let no_comment_size = s.len();

        let mut path = PathBuf::from(OBJ_PATH);
        path.push(s);

        if !mtl_path.contains(&path) {
            return Err(pfe!(
                f!("cannot find {B}{Y}{path}{D} for {B}{Y}{}{D}"),
                h:"make sure you have a valid {B}{G}.mtl{D} file in the {G}same directory{D} as the {B}{BLU}.obj{D} file",
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
