use crate::prelude::*;

use super::FileData;
use crate::parse::line::{get_line_u, Keyword};
use crate::setting::OBJ_PATH;

use ansi::abbrev::{B, BLU, D, Y};

use std::path::PathBuf;

pub fn get_material(f: &FileData, mtl_path: &Vec<PathBuf>) -> Result<Option<PathBuf>> {
    let keyword = Keyword::new("usemtl", f!("the {B}{BLU}.mlt{D} file to use").as_str());
    if let Some((token, i)) = get_line_u(f, &keyword, Some("file.mtl"))? {
        assert_eq!(token.len(), 1);
        let mut path = PathBuf::from(OBJ_PATH);
        path.push(token[0]);

        if !mtl_path.contains(&path) {
            let unprocessed_line = f.content[i].clone();

            return Err(pfe!(
                f!("cannot find {B}{Y}{path}{D} for {B}{Y}{}{D}"),
                h:"make sure you have a valid {B}{G}.mtl{D} file in the {G}same directory{D} as the {B}{BLU}.obj{D} file",
                f:f.name.clone(),
                l:ple!(unprocessed_line, i:i, w:pwe!((0, unprocessed_line.len())))
            ))?;
        }

        return Ok(Some(path));
    }

    Ok(None)
}
